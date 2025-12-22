//! Details Panel widget - Contextual information for the selected item.

use iced::widget::{column, container, text, Space};
use iced::{Element, Fill, Length};

use crate::github::NotificationView;
use crate::ui::screens::notifications::NotificationMessage;
use crate::ui::theme;

pub fn view_details_panel<'a>(
    selected_notification: Option<&'a NotificationView>,
    _icon_theme: crate::settings::IconTheme,
) -> Element<'a, NotificationMessage> {
    let p = theme::palette();

    let content = if let Some(notif) = selected_notification {
        column![
            text(&notif.repo_full_name).size(12).color(p.text_secondary),
            Space::new().height(4),
            text(&notif.title).size(18).color(p.text_primary),
            Space::new().height(16),
            text(format!("Reason: {}", notif.reason.label()))
                .size(12)
                .color(p.text_muted),
            // TODO: Add full thread view here
        ]
        .spacing(0)
        .padding(24)
    } else {
        column![text("Select a notification").size(14).color(p.text_muted),]
            .align_x(iced::Alignment::Center)
            .padding(24)
    };

    container(content)
        .width(Length::Fixed(350.0)) // Fixed width for panel
        .height(Fill)
        .style(move |_| container::Style {
            background: Some(iced::Background::Color(p.bg_base)),
            border: iced::Border {
                width: 1.0,
                color: p.border_subtle,
                radius: 0.0.into(),
            },
            ..Default::default() // TODO: Left border only
        })
        .into()
}
