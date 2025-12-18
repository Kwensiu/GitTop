//! Notifications screen - main notification list view.
//!
//! Optimized for scroll performance:
//! - Uses keyed_column for efficient widget diffing
//! - Minimal widget nesting
//! - Simple styling

use chrono::Local;
use iced::widget::{button, column, container, keyed_column, row, scrollable, text, Space};
use iced::{Alignment, Element, Fill, Task};

use crate::github::{GitHubClient, GitHubError, NotificationView, UserInfo};
use crate::ui::theme;
use crate::ui::widgets::notification_item;

/// Group of notifications by time period.
#[derive(Debug, Clone)]
pub struct NotificationGroup {
    pub title: String,
    pub notifications: Vec<NotificationView>,
    pub is_expanded: bool,
}

/// Notifications screen state.
#[derive(Debug, Clone)]
pub struct NotificationsScreen {
    pub client: GitHubClient,
    pub user: UserInfo,
    pub all_notifications: Vec<NotificationView>,
    pub groups: Vec<NotificationGroup>,
    pub is_loading: bool,
    pub error_message: Option<String>,
    pub show_all: bool,
}

/// Notifications screen messages.
#[derive(Debug, Clone)]
pub enum NotificationMessage {
    Refresh,
    RefreshComplete(Result<Vec<NotificationView>, GitHubError>),
    Open(String),
    MarkAsRead(String),
    MarkAsReadComplete(String, Result<(), GitHubError>),
    MarkAllAsRead,
    MarkAllAsReadComplete(Result<(), GitHubError>),
    ToggleShowAll,
    Logout,
    ToggleGroup(usize),
}

impl NotificationsScreen {
    pub fn new(client: GitHubClient, user: UserInfo) -> (Self, Task<NotificationMessage>) {
        let screen = Self {
            client,
            user,
            all_notifications: Vec::new(),
            groups: Vec::new(),
            is_loading: true,
            error_message: None,
            show_all: false,
        };
        let task = screen.fetch_notifications();
        (screen, task)
    }

    fn fetch_notifications(&self) -> Task<NotificationMessage> {
        let client = self.client.clone();
        let show_all = self.show_all;
        Task::perform(
            async move { client.get_notification_views(show_all).await },
            NotificationMessage::RefreshComplete,
        )
    }

    fn group_notifications(&mut self) {
        let now_date = Local::now().date_naive();
        let one_week_ago = now_date - chrono::Duration::days(7);

        let mut today = Vec::new();
        let mut this_week = Vec::new();
        let mut older = Vec::new();

        for notif in &self.all_notifications {
            let notif_date = notif.updated_at.with_timezone(&Local).date_naive();
            if notif_date >= now_date {
                today.push(notif.clone());
            } else if notif_date >= one_week_ago {
                this_week.push(notif.clone());
            } else {
                older.push(notif.clone());
            }
        }

        self.groups = vec![
            NotificationGroup {
                title: "Today".to_string(),
                notifications: today,
                is_expanded: true,
            },
            NotificationGroup {
                title: "This Week".to_string(),
                notifications: this_week,
                is_expanded: true,
            },
            NotificationGroup {
                title: "Older".to_string(),
                notifications: older,
                is_expanded: false,
            },
        ];
    }

    pub fn update(&mut self, message: NotificationMessage) -> Task<NotificationMessage> {
        match message {
            NotificationMessage::Refresh => {
                self.is_loading = true;
                self.error_message = None;
                self.fetch_notifications()
            }
            NotificationMessage::RefreshComplete(result) => {
                self.is_loading = false;
                match result {
                    Ok(notifications) => {
                        self.all_notifications = notifications;
                        self.group_notifications();
                        self.error_message = None;
                    }
                    Err(e) => {
                        self.error_message = Some(e.to_string());
                    }
                }
                Task::none()
            }
            NotificationMessage::Open(id) => {
                if let Some(notif) = self.all_notifications.iter().find(|n| n.id == id) {
                    if let Some(ref url) = notif.url {
                        let web_url = convert_api_url_to_web(url);
                        let _ = open::that(&web_url);
                    }
                }
                let client = self.client.clone();
                let notif_id = id.clone();
                Task::perform(
                    async move { client.mark_as_read(&notif_id).await },
                    move |result| NotificationMessage::MarkAsReadComplete(id.clone(), result),
                )
            }
            NotificationMessage::MarkAsRead(id) => {
                let client = self.client.clone();
                let notif_id = id.clone();
                Task::perform(
                    async move { client.mark_as_read(&notif_id).await },
                    move |result| NotificationMessage::MarkAsReadComplete(id.clone(), result),
                )
            }
            NotificationMessage::MarkAsReadComplete(id, result) => {
                if result.is_ok() {
                    if let Some(notif) = self.all_notifications.iter_mut().find(|n| n.id == id) {
                        notif.unread = false;
                        self.group_notifications();
                    }
                }
                Task::none()
            }
            NotificationMessage::MarkAllAsRead => {
                let client = self.client.clone();
                Task::perform(
                    async move { client.mark_all_as_read().await },
                    NotificationMessage::MarkAllAsReadComplete,
                )
            }
            NotificationMessage::MarkAllAsReadComplete(result) => {
                if result.is_ok() {
                    for notif in &mut self.all_notifications {
                        notif.unread = false;
                    }
                    self.group_notifications();
                }
                Task::none()
            }
            NotificationMessage::ToggleShowAll => {
                self.show_all = !self.show_all;
                self.is_loading = true;
                self.fetch_notifications()
            }
            NotificationMessage::Logout => Task::none(),
            NotificationMessage::ToggleGroup(index) => {
                if let Some(group) = self.groups.get_mut(index) {
                    group.is_expanded = !group.is_expanded;
                }
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, NotificationMessage> {
        column![self.view_header(), self.view_content()]
            .height(Fill)
            .into()
    }

    fn view_header(&self) -> Element<'_, NotificationMessage> {
        let unread_count = self.all_notifications.iter().filter(|n| n.unread).count();

        let title = row![
            text("Notifications").size(18),
            Space::new().width(8),
            text(if unread_count > 0 {
                format!("{}", unread_count)
            } else {
                String::new()
            })
            .size(12)
            .color(theme::TEXT_PRIMARY),
        ]
        .align_y(Alignment::Center);

        let user_info = text(&self.user.login).size(12).color(theme::TEXT_SECONDARY);

        let refresh_btn = button(text("↻").size(16))
            .style(theme::ghost_button)
            .padding(8)
            .on_press(NotificationMessage::Refresh);

        let filter_btn = button(text(if self.show_all { "All" } else { "Unread" }).size(11))
            .style(theme::secondary_button)
            .padding([4, 8])
            .on_press(NotificationMessage::ToggleShowAll);

        let mark_all_btn = if unread_count > 0 {
            button(text("✓ All").size(11))
                .style(theme::ghost_button)
                .padding([4, 8])
                .on_press(NotificationMessage::MarkAllAsRead)
        } else {
            button(text("✓ All").size(11))
                .style(theme::ghost_button)
                .padding([4, 8])
        };

        let logout_btn = button(text("⏻").size(14))
            .style(theme::ghost_button)
            .padding(8)
            .on_press(NotificationMessage::Logout);

        let header_row = row![
            title,
            Space::new().width(Fill),
            user_info,
            Space::new().width(8),
            filter_btn,
            mark_all_btn,
            refresh_btn,
            logout_btn,
        ]
        .align_y(Alignment::Center)
        .padding([12, 16]);

        container(header_row)
            .width(Fill)
            .style(theme::header)
            .into()
    }

    fn view_content(&self) -> Element<'_, NotificationMessage> {
        if self.is_loading && self.all_notifications.is_empty() {
            return self.view_loading();
        }

        if let Some(ref error) = self.error_message {
            return self.view_error(error.clone());
        }

        if self.all_notifications.is_empty() {
            return self.view_empty();
        }

        // Build content with keyed_column for efficient diffing
        let mut content = column![].spacing(8).padding([8, 8]);

        for (group_idx, group) in self.groups.iter().enumerate() {
            if group.notifications.is_empty() {
                continue;
            }

            // Group header
            let header = button(
                row![
                    text(if group.is_expanded { "▼" } else { "▶" })
                        .size(10)
                        .color(theme::TEXT_MUTED),
                    Space::new().width(8),
                    text(&group.title).size(12).color(theme::TEXT_SECONDARY),
                    Space::new().width(4),
                    text(format!("({})", group.notifications.len()))
                        .size(11)
                        .color(theme::TEXT_MUTED),
                ]
                .align_y(Alignment::Center),
            )
            .style(theme::ghost_button)
            .padding(4)
            .on_press(NotificationMessage::ToggleGroup(group_idx))
            .width(Fill);

            content = content.push(header);

            // Items - use keyed_column for efficient updates
            if group.is_expanded {
                let items = group
                    .notifications
                    .iter()
                    .enumerate()
                    .map(|(idx, n)| (idx, notification_item(n)));

                content = content.push(keyed_column(items).spacing(2));
            }

            content = content.push(Space::new().height(8));
        }

        container(
            scrollable(content)
                .height(Fill)
                .width(Fill)
                .style(theme::scrollbar),
        )
        .style(theme::app_container)
        .height(Fill)
        .width(Fill)
        .into()
    }

    fn view_loading(&self) -> Element<'_, NotificationMessage> {
        container(
            text("Loading notifications...")
                .size(14)
                .color(theme::TEXT_SECONDARY),
        )
        .width(Fill)
        .height(Fill)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .style(theme::app_container)
        .into()
    }

    fn view_error(&self, error: String) -> Element<'_, NotificationMessage> {
        let content = column![
            text("⚠").size(32).color(theme::ACCENT_ORANGE),
            Space::new().height(16),
            text("Failed to load notifications")
                .size(16)
                .color(theme::TEXT_PRIMARY),
            Space::new().height(8),
            text(error).size(12).color(theme::TEXT_SECONDARY),
            Space::new().height(24),
            button(text("Retry").size(14))
                .style(theme::primary_button)
                .padding([10, 24])
                .on_press(NotificationMessage::Refresh),
        ]
        .align_x(Alignment::Center);

        container(content)
            .width(Fill)
            .height(Fill)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .style(theme::app_container)
            .into()
    }

    fn view_empty(&self) -> Element<'_, NotificationMessage> {
        let message = if self.show_all {
            "No notifications yet"
        } else {
            "All caught up!"
        };

        let content = column![
            text("✓").size(48).color(theme::ACCENT_GREEN),
            Space::new().height(16),
            text(message).size(16).color(theme::TEXT_PRIMARY),
            Space::new().height(8),
            text("You have no unread notifications")
                .size(12)
                .color(theme::TEXT_SECONDARY),
        ]
        .align_x(Alignment::Center);

        container(content)
            .width(Fill)
            .height(Fill)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .style(theme::app_container)
            .into()
    }
}

/// Convert GitHub API URLs to web URLs.
fn convert_api_url_to_web(api_url: &str) -> String {
    api_url
        .replace("api.github.com/repos", "github.com")
        .replace("/pulls/", "/pull/")
}
