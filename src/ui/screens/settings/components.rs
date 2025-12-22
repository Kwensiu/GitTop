//! Shared components for settings tabs.

use iced::widget::container;
use iced::Element;

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
