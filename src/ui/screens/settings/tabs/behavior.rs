//! Behavior tab - minimize to tray and app behavior settings.

use iced::widget::{column, row, text, toggler, Space};
use iced::{Alignment, Element, Fill};

use crate::settings::AppSettings;
use crate::ui::theme;

use super::super::components::setting_card;
use super::super::messages::SettingsMessage;

/// Render the behavior tab content.
pub fn view(settings: &AppSettings) -> Element<'_, SettingsMessage> {
    column![
        tab_title("Behavior"),
        text("Control how GitTop behaves.")
            .size(12)
            .color(theme::palette().text_secondary),
        Space::new().height(16),
        view_minimize_to_tray_setting(settings),
    ]
    .spacing(4)
    .padding(24)
    .width(Fill)
    .into()
}

fn tab_title(title: &'static str) -> Element<'static, SettingsMessage> {
    text(title)
        .size(20)
        .color(theme::palette().text_primary)
        .into()
}

fn view_minimize_to_tray_setting(settings: &AppSettings) -> Element<'_, SettingsMessage> {
    let p = theme::palette();
    let enabled = settings.minimize_to_tray;

    let description = if enabled {
        "App stays in system tray when closed"
    } else {
        "App exits when closed"
    };

    setting_card(
        row![
            column![
                text("Minimize to Tray").size(14).color(p.text_primary),
                Space::new().height(4),
                text(description).size(11).color(p.text_secondary),
            ]
            .width(Fill),
            toggler(enabled)
                .on_toggle(SettingsMessage::ToggleMinimizeToTray)
                .size(20),
        ]
        .align_y(Alignment::Center),
    )
}
