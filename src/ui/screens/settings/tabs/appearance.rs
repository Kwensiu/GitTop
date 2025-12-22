//! Appearance tab - theme and visual settings.

use iced::widget::{column, pick_list, row, text, toggler, Space};
use iced::{Alignment, Element, Fill};

use crate::settings::{AppSettings, AppTheme, IconTheme};
use crate::ui::theme;

use super::super::components::{setting_card, tab_title};
use super::super::messages::SettingsMessage;

/// Render the appearance tab content.
pub fn view(settings: &AppSettings) -> Element<'_, SettingsMessage> {
    column![
        tab_title("Appearance"),
        text("Customize the look and feel of GitTop.")
            .size(12)
            .color(theme::palette().text_secondary),
        Space::new().height(16),
        view_theme_setting(settings),
        Space::new().height(8),
        view_icon_theme_setting(settings),
        Space::new().height(8),
        view_notification_font_scale_setting(settings),
        Space::new().height(8),
        view_sidebar_font_scale_setting(settings),
        Space::new().height(8),
        view_sidebar_width_setting(settings),
    ]
    .spacing(4)
    .padding(24)
    .width(Fill)
    .into()
}

fn view_theme_setting(settings: &AppSettings) -> Element<'_, SettingsMessage> {
    let p = theme::palette();
    let current_theme = settings.theme;
    let themes = vec![
        AppTheme::Light,
        AppTheme::Steam,
        AppTheme::GtkDark,
        AppTheme::Windows11,
        AppTheme::MacOS,
        AppTheme::HighContrast,
    ];

    setting_card(
        row![
            column![
                text("Theme").size(14).color(p.text_primary),
                Space::new().height(4),
                text("Choose your preferred color scheme")
                    .size(11)
                    .color(p.text_secondary),
            ]
            .width(Fill),
            pick_list(themes, Some(current_theme), SettingsMessage::ChangeTheme)
                .text_size(13)
                .padding([8, 12])
                .style(theme::pick_list_style)
                .menu_style(theme::menu_style),
        ]
        .align_y(Alignment::Center),
    )
}

fn view_icon_theme_setting(settings: &AppSettings) -> Element<'_, SettingsMessage> {
    let p = theme::palette();
    let use_svg = settings.icon_theme == IconTheme::Svg;

    let description = if use_svg {
        "High quality SVG icons"
    } else {
        "Emoji icons (minimal memory)"
    };

    setting_card(
        row![
            column![
                text("Icon Style").size(14).color(p.text_primary),
                Space::new().height(4),
                text(description).size(11).color(p.text_secondary),
            ]
            .width(Fill),
            toggler(use_svg)
                .on_toggle(SettingsMessage::ToggleIconTheme)
                .size(20),
        ]
        .align_y(Alignment::Center),
    )
}

fn view_notification_font_scale_setting(settings: &AppSettings) -> Element<'_, SettingsMessage> {
    let p = theme::palette();
    let scale = settings.notification_font_scale;
    let scale_text = format!("{}%", (scale * 100.0) as i32);

    setting_card(column![
        row![
            text("Notification Text Size")
                .size(14)
                .color(p.text_primary),
            Space::new().width(Fill),
            text(scale_text).size(12).color(p.text_secondary),
        ]
        .align_y(Alignment::Center),
        Space::new().height(12),
        iced::widget::slider(0.8..=1.5, scale, SettingsMessage::SetNotificationFontScale)
            .step(0.05),
    ])
}

fn view_sidebar_font_scale_setting(settings: &AppSettings) -> Element<'_, SettingsMessage> {
    let p = theme::palette();
    let scale = settings.sidebar_font_scale;
    let scale_text = format!("{}%", (scale * 100.0) as i32);

    setting_card(column![
        row![
            text("Sidebar Text Size").size(14).color(p.text_primary),
            Space::new().width(Fill),
            text(scale_text).size(12).color(p.text_secondary),
        ]
        .align_y(Alignment::Center),
        Space::new().height(12),
        iced::widget::slider(0.8..=1.5, scale, SettingsMessage::SetSidebarFontScale).step(0.05),
    ])
}

fn view_sidebar_width_setting(settings: &AppSettings) -> Element<'_, SettingsMessage> {
    let p = theme::palette();
    let width = settings.sidebar_width;
    let width_text = format!("{}px", width as i32);

    setting_card(column![
        row![
            text("Sidebar Width").size(14).color(p.text_primary),
            Space::new().width(Fill),
            text(width_text).size(12).color(p.text_secondary),
        ]
        .align_y(Alignment::Center),
        Space::new().height(12),
        iced::widget::slider(180.0..=400.0, width, SettingsMessage::SetSidebarWidth).step(10.0),
    ])
}
