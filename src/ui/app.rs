//! Main application state and logic.

use std::time::Duration;

use iced::window::Id as WindowId;
use iced::{event, exit, time, window, Element, Event, Subscription, Task, Theme};

use crate::github::{AuthManager, SessionManager};
use crate::settings::AppSettings;
use crate::tray::{TrayCommand, TrayManager};
use crate::ui::screens::settings::rule_engine::rules::NotificationRuleSet;
use crate::ui::screens::{
    login::{LoginMessage, LoginScreen},
    notifications::{NotificationMessage, NotificationsScreen},
    settings::{
        rule_engine::{RuleEngineMessage, RuleEngineScreen},
        SettingsMessage, SettingsScreen,
    },
};
use crate::ui::widgets::power::{details_panel, status_bar, top_bar};
use crate::ui::{theme, window_state};

/// Application state - which screen we're on.
pub enum App {
    /// Checking for existing auth on startup.
    Loading,
    /// Login screen - no auth.
    Login(LoginScreen),
    /// Main notifications screen - authenticated.
    Notifications(NotificationsScreen, AppSettings, SessionManager),
    /// Settings screen.
    Settings(SettingsScreen, SessionManager),
    /// Rule Engine screen.
    RuleEngine(
        RuleEngineScreen,
        SessionManager,
        AppSettings,
        RuleEngineOrigin,
    ),
}

/// Where the Rule Engine was opened from (for back navigation).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleEngineOrigin {
    Settings,
    Notifications,
}

/// Top-level application messages.
#[derive(Debug, Clone)]
pub enum Message {
    /// Restore result (returns SessionManager with restored accounts).
    RestoreComplete(SessionManager),
    /// Login screen messages.
    Login(LoginMessage),
    /// Notifications screen messages.
    Notifications(NotificationMessage),
    /// Settings screen messages.
    Settings(SettingsMessage),
    /// Rule Engine screen messages.
    RuleEngine(RuleEngineMessage),
    /// Periodic refresh tick.
    Tick,
    /// Tray event poll tick.
    TrayPoll,
    /// Window event.
    WindowEvent(WindowId, window::Event),
}

impl App {
    /// Create the app and start the restore task.
    pub fn new() -> (Self, Task<Message>) {
        (
            App::Loading,
            Task::perform(
                async {
                    // Try to restore from old AuthManager first (migration)
                    let mut sessions = SessionManager::new();

                    // Load stored accounts from settings
                    let settings = AppSettings::load();
                    for account in &settings.accounts {
                        // Try to restore each account
                        let _ = sessions.restore_account(&account.username).await;
                    }

                    // Fallback: try old single-token auth and migrate
                    if sessions.is_empty() {
                        if let Ok(Some((client, user))) = AuthManager::try_restore().await {
                            // Old token exists, migrate to new system
                            let token = client.token().to_string();
                            let _ = sessions.add_account(&token).await;
                            // Delete old token after migration
                            let _ = AuthManager::delete_token();
                        }
                    }

                    sessions
                },
                Message::RestoreComplete,
            ),
        )
    }

    /// Update application state.
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::RestoreComplete(sessions) => {
                if let Some(session) = sessions.primary() {
                    let mut settings = AppSettings::load();
                    settings.set_active_account(&session.username);
                    let _ = settings.save();

                    // Initialize global theme and font scales from settings
                    theme::set_theme(settings.theme);
                    theme::set_notification_font_scale(settings.notification_font_scale);
                    theme::set_sidebar_font_scale(settings.sidebar_font_scale);

                    let (screen, task) =
                        NotificationsScreen::new(session.client.clone(), session.user.clone());
                    *self = App::Notifications(screen, settings, sessions);
                    task.map(Message::Notifications)
                } else {
                    *self = App::Login(LoginScreen::new());
                    Task::none()
                }
            }

            Message::Login(login_msg) => {
                if let App::Login(screen) = self {
                    // Check for successful login
                    if let LoginMessage::LoginSuccess(client, user) = login_msg.clone() {
                        let mut settings = AppSettings::load();
                        settings.set_active_account(&user.login);
                        let _ = settings.save();

                        // Initialize global theme and font scales from settings
                        theme::set_theme(settings.theme);
                        theme::set_notification_font_scale(settings.notification_font_scale);
                        theme::set_sidebar_font_scale(settings.sidebar_font_scale);

                        // Save token to new keyring system
                        let token = client.token().to_string();
                        let username = user.login.clone();
                        let _ = crate::github::AccountKeyring::save_token(&username, &token);

                        // Create session manager with this account
                        let sessions = SessionManager::new();
                        // Note: Session will be restored on next app launch from keyring

                        let (notif_screen, task) = NotificationsScreen::new(client, user);
                        *self = App::Notifications(notif_screen, settings, sessions);
                        return task.map(Message::Notifications);
                    }

                    screen.update(login_msg).map(Message::Login)
                } else {
                    Task::none()
                }
            }

            Message::Notifications(notif_msg) => {
                if let App::Notifications(screen, settings, sessions) = self {
                    // Check for logout
                    if let NotificationMessage::Logout = notif_msg {
                        // Collect usernames as owned strings first to avoid borrow issues
                        let usernames: Vec<String> =
                            sessions.usernames().map(String::from).collect();
                        for username in usernames {
                            let _ = sessions.remove_account(&username);
                        }
                        // Also clean up old auth
                        let _ = AuthManager::delete_token();
                        *self = App::Login(LoginScreen::new());
                        return Task::none();
                    }

                    // Check for open settings
                    if let NotificationMessage::OpenSettings = notif_msg {
                        let settings_screen = SettingsScreen::new(settings.clone());
                        *self = App::Settings(settings_screen, sessions.clone());
                        return Task::none();
                    }

                    // Check for open rule engine
                    if let NotificationMessage::OpenRuleEngine = notif_msg {
                        let rules = NotificationRuleSet::load();
                        let rule_engine_screen = RuleEngineScreen::new(rules, settings.clone());
                        *self = App::RuleEngine(
                            rule_engine_screen,
                            sessions.clone(),
                            settings.clone(),
                            RuleEngineOrigin::Notifications,
                        );
                        return Task::none();
                    }

                    // Handle Account Switching
                    if let NotificationMessage::SwitchAccount(username) = notif_msg {
                        // Check if we need to switch
                        if let Some(current) = sessions.primary() {
                            if current.username == username {
                                return Task::none();
                            }
                        }

                        // Set new active account
                        sessions.set_primary(&username);

                        // Load new session
                        if let Some(session) = sessions.primary() {
                            let (notif_screen, task) = NotificationsScreen::new(
                                session.client.clone(),
                                session.user.clone(),
                            );
                            *self = App::Notifications(
                                notif_screen,
                                settings.clone(),
                                sessions.clone(),
                            );
                            return task.map(Message::Notifications);
                        }
                        return Task::none();
                    }

                    // Handle Settings Navigation
                    if let NotificationMessage::OpenSettings = notif_msg {
                        // Create and switch to settings screen
                        let settings_screen = SettingsScreen::new(settings.clone());
                        *self = App::Settings(settings_screen, sessions.clone());
                        return Task::none();
                    }

                    if let NotificationMessage::TogglePowerMode = notif_msg {
                        settings.power_mode = !settings.power_mode;
                        let _ = settings.save();
                        // reset view state
                        screen.collapse_all_groups();
                        return Task::none();
                    }

                    screen.update(notif_msg).map(Message::Notifications)
                } else {
                    Task::none()
                }
            }

            Message::Settings(settings_msg) => {
                if let App::Settings(screen, sessions) = self {
                    // Check for back navigation
                    if let SettingsMessage::Back = settings_msg {
                        if let Some(session) = sessions.primary() {
                            let (notif_screen, task) = NotificationsScreen::new(
                                session.client.clone(),
                                session.user.clone(),
                            );
                            *self = App::Notifications(
                                notif_screen,
                                screen.settings.clone(),
                                sessions.clone(),
                            );
                            return task.map(Message::Notifications);
                        }
                    }

                    // Check for open rule engine
                    if let SettingsMessage::OpenRuleEngine = settings_msg {
                        let rules = NotificationRuleSet::load();
                        let rule_engine_screen =
                            RuleEngineScreen::new(rules, screen.settings.clone());
                        *self = App::RuleEngine(
                            rule_engine_screen,
                            sessions.clone(),
                            screen.settings.clone(),
                            RuleEngineOrigin::Settings,
                        );
                        return Task::none();
                    }

                    screen.update(settings_msg).map(Message::Settings)
                } else {
                    Task::none()
                }
            }

            Message::RuleEngine(rule_msg) => {
                if let App::RuleEngine(screen, sessions, settings, origin) = self {
                    // Check for back navigation
                    if let RuleEngineMessage::Back = rule_msg {
                        match origin {
                            RuleEngineOrigin::Settings => {
                                let settings_screen = SettingsScreen::new(settings.clone());
                                *self = App::Settings(settings_screen, sessions.clone());
                            }
                            RuleEngineOrigin::Notifications => {
                                if let Some(session) = sessions.primary() {
                                    let (notif_screen, task) = NotificationsScreen::new(
                                        session.client.clone(),
                                        session.user.clone(),
                                    );
                                    *self = App::Notifications(
                                        notif_screen,
                                        settings.clone(),
                                        sessions.clone(),
                                    );
                                    return task.map(Message::Notifications);
                                }
                            }
                        }
                        return Task::none();
                    }

                    screen.update(rule_msg).map(Message::RuleEngine)
                } else {
                    Task::none()
                }
            }

            Message::Tick => {
                // Auto-refresh notifications every tick
                if let App::Notifications(screen, _, _) = self {
                    if !screen.is_loading {
                        return screen
                            .update(NotificationMessage::Refresh)
                            .map(Message::Notifications);
                    }
                }
                Task::none()
            }

            Message::TrayPoll => {
                // Poll tray events
                if let Some(cmd) = TrayManager::poll_global_events() {
                    match cmd {
                        TrayCommand::ShowWindow => {
                            // Show and focus the window using stored ID
                            let was_hidden = window_state::restore_from_hidden();

                            let window_task = if let Some(id) = window_state::get_window_id() {
                                Task::batch([
                                    window::set_mode(id, window::Mode::Windowed),
                                    window::gain_focus(id),
                                ])
                            } else {
                                Task::none()
                            };

                            // If coming back from hidden, trigger a refresh
                            if was_hidden {
                                if let App::Notifications(screen, _, _) = self {
                                    return Task::batch([
                                        window_task,
                                        screen
                                            .update(NotificationMessage::Refresh)
                                            .map(Message::Notifications),
                                    ]);
                                }
                            }

                            window_task
                        }
                        TrayCommand::Quit => {
                            // Exit the application
                            exit()
                        }
                    }
                } else {
                    Task::none()
                }
            }

            Message::WindowEvent(id, event) => {
                // Store the main window ID on first event
                window_state::set_window_id(id);

                if let window::Event::CloseRequested = event {
                    // Check if minimize to tray is enabled
                    let minimize_to_tray = match self {
                        App::Notifications(_, settings, _) => settings.minimize_to_tray,
                        App::Settings(screen, _) => screen.settings.minimize_to_tray,
                        _ => AppSettings::load().minimize_to_tray,
                    };

                    if minimize_to_tray {
                        // Hide the window to tray instead of quitting
                        window_state::set_hidden(true);

                        // Clear notification data to free memory
                        if let App::Notifications(screen, _, _) = self {
                            screen.all_notifications.clear();
                            screen.all_notifications.shrink_to_fit();
                            screen.filtered_notifications.clear();
                            screen.filtered_notifications.shrink_to_fit();
                            screen.groups.clear();
                            screen.groups.shrink_to_fit();
                            screen.type_counts.clear();
                            screen.type_counts.shrink_to_fit();
                            screen.repo_counts.clear();
                            screen.repo_counts.shrink_to_fit();
                        }

                        // Aggressively trim memory
                        crate::platform::trim_memory();

                        window::set_mode(id, window::Mode::Hidden)
                    } else {
                        // Exit the application
                        exit()
                    }
                } else if let window::Event::Moved(position) = event {
                    // Save window position when moved
                    if let Some(settings) = self.get_settings_mut() {
                        settings.window_x = Some(position.x as i32);
                        settings.window_y = Some(position.y as i32);
                        let _ = settings.save();
                    }
                    Task::none()
                } else if let window::Event::Resized(size) = event {
                    // Save window size when resized
                    if let Some(settings) = self.get_settings_mut() {
                        settings.window_width = size.width;
                        settings.window_height = size.height;
                        let _ = settings.save();
                    }
                    Task::none()
                } else {
                    Task::none()
                }
            }
        }
    }

    /// Get mutable reference to settings if available
    fn get_settings_mut(&mut self) -> Option<&mut AppSettings> {
        match self {
            App::Notifications(_, settings, _) => Some(settings),
            App::Settings(screen, _) => Some(&mut screen.settings),
            _ => None,
        }
    }

    /// Render the current view.
    pub fn view(&self) -> Element<'_, Message> {
        match self {
            App::Loading => {
                use crate::ui::theme;
                use iced::widget::{container, text};

                container(text("Loading...").size(14))
                    .width(iced::Fill)
                    .height(iced::Fill)
                    .align_x(iced::Alignment::Center)
                    .align_y(iced::Alignment::Center)
                    .style(theme::app_container)
                    .into()
            }
            App::Login(screen) => screen.view().map(Message::Login),
            App::Notifications(screen, settings, sessions) => {
                // Get list of accounts
                let accounts: Vec<String> = sessions.usernames().map(String::from).collect();

                if settings.power_mode {
                    // Power Mode: TopBar + [Sidebar | Content | Details] + StatusBar
                    let content =
                        screen.view(&accounts, settings.icon_theme, settings.sidebar_width, true);

                    let main_area: iced::Element<NotificationMessage> =
                        if settings.show_details_panel {
                            use iced::widget::row;
                            // TODO: Get actual selected notification
                            row![
                                content,
                                details_panel::view_details_panel(None, settings.icon_theme)
                            ]
                            .height(iced::Fill)
                            .into()
                        } else {
                            use iced::widget::row;
                            row![content].height(iced::Fill).into()
                        };

                    // Build account info for top bar
                    let account_infos: Vec<top_bar::AccountInfo> = accounts
                        .iter()
                        .map(|username| top_bar::AccountInfo {
                            username: username.clone(),
                            is_primary: sessions.primary().map(|s| s.username.as_str())
                                == Some(username),
                        })
                        .collect();

                    let unread_count = screen
                        .filtered_notifications
                        .iter()
                        .filter(|n| n.unread)
                        .count();

                    use iced::widget::column;
                    let power_layout: iced::Element<NotificationMessage> = column![
                        top_bar::view_top_bar(
                            &screen.user,
                            account_infos,
                            screen.is_loading,
                            unread_count,
                            screen.filters.show_all,
                            settings.icon_theme
                        ),
                        main_area,
                        status_bar::view_status_bar(settings.icon_theme)
                    ]
                    .into();

                    power_layout.map(Message::Notifications)
                } else {
                    // Standard Layout
                    screen
                        .view(
                            &accounts,
                            settings.icon_theme,
                            settings.sidebar_width,
                            false,
                        )
                        .map(Message::Notifications)
                }
            }
            App::Settings(screen, _) => screen.view().map(Message::Settings),
            App::RuleEngine(screen, _, _, _) => screen.view().map(Message::RuleEngine),
        }
    }

    /// Window title.
    pub fn title(&self) -> String {
        match self {
            App::Loading => "GitTop".to_string(),
            App::Login(_) => "GitTop - Sign In".to_string(),
            App::Notifications(screen, _, _) => {
                let unread = screen.all_notifications.iter().filter(|n| n.unread).count();
                if unread > 0 {
                    format!("GitTop ({} unread)", unread)
                } else {
                    "GitTop".to_string()
                }
            }
            App::Settings(_, _) => "GitTop - Settings".to_string(),
            App::RuleEngine(_, _, _, _) => "GitTop - Rule Engine".to_string(),
        }
    }

    /// Application theme - using a dark theme.
    pub fn theme(&self) -> Theme {
        Theme::TokyoNightStorm
    }

    /// Subscriptions - periodic refresh, tray events, and window events.
    pub fn subscription(&self) -> Subscription<Message> {
        let is_hidden = window_state::is_hidden();

        // Poll tray events - slower when hidden to save CPU
        let tray_poll_interval = if is_hidden { 500 } else { 100 };
        let tray_sub =
            time::every(Duration::from_millis(tray_poll_interval)).map(|_| Message::TrayPoll);

        // Subscribe to window events
        let window_sub = event::listen_with(|event, _status, id| {
            if let Event::Window(window_event) = event {
                Some(Message::WindowEvent(id, window_event))
            } else {
                None
            }
        });

        match self {
            App::Notifications(_, _, _) => {
                if is_hidden {
                    // When hidden, still refresh but less frequently (every 2 minutes)
                    // This allows desktop notifications to fire for new items
                    Subscription::batch([
                        time::every(Duration::from_secs(60)).map(|_| Message::Tick),
                        tray_sub,
                        window_sub,
                    ])
                } else {
                    // Refresh every 60 seconds + tray events + window events
                    Subscription::batch([
                        time::every(Duration::from_secs(60)).map(|_| Message::Tick),
                        tray_sub,
                        window_sub,
                    ])
                }
            }
            _ => Subscription::batch([tray_sub, window_sub]),
        }
    }
}
