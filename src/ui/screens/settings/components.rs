//! Shared components for settings tabs.

use iced::Element;
use iced::widget::{container, text};

use crate::ui::theme;

use super::messages::SettingsMessage;

/// A styled card container for settings items.
pub fn setting_card<'a>(
    content: impl Into<Element<'a, SettingsMessage>>,
) -> Element<'a, SettingsMessage> {
    let p = theme::palette();

    container(container(content).padding(14))
        .style(move |_| container::Style {
            background: Some(iced::Background::Color(p.bg_card)),
            border: iced::Border {
                radius: 8.0.into(),
                ..Default::default()
            },
            ..Default::default()
        })
        .into()
}

/// Styled title for settings tabs.
pub fn tab_title(title: &'static str) -> Element<'static, SettingsMessage> {
    text(title)
        .size(20)
        .color(theme::palette().text_primary)
        .into()
}
