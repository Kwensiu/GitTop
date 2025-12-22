//! Notifications tab - notification preferences (placeholder).

use iced::widget::{column, container, text, Space};
use iced::{Alignment, Element, Fill};

use crate::ui::theme;

use super::super::messages::SettingsMessage;

/// Render the notifications tab content.
pub fn view() -> Element<'static, SettingsMessage> {
    let p = theme::palette();

    column![
        tab_title("Notifications"),
        text("Configure notification preferences.")
            .size(12)
            .color(p.text_secondary),
        Space::new().height(24),
        container(
            column![
                text("Coming Soon").size(14).color(p.text_muted),
                Space::new().height(8),
                text("Notification settings will be available in a future update.")
                    .size(12)
                    .color(p.text_muted),
            ]
            .align_x(Alignment::Center)
            .padding(32)
        )
        .width(Fill)
        .style(move |_| container::Style {
            background: Some(iced::Background::Color(p.bg_card)),
            border: iced::Border {
                radius: 8.0.into(),
                ..Default::default()
            },
            ..Default::default()
        }),
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
