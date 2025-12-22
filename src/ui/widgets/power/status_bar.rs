//! Status Bar widget - System health and background tasks.

use iced::widget::{button, container, row, text, Space};
use iced::{Alignment, Element, Fill};

use crate::settings::IconTheme;
use crate::ui::screens::notifications::NotificationMessage;
use crate::ui::{icons, theme};

pub fn view_status_bar<'a>(icon_theme: IconTheme) -> Element<'a, NotificationMessage> {
    let p = theme::palette();

    let rule_engine_btn = button(
        row![
            icons::icon_filter(12.0, p.text_muted, icon_theme),
            Space::new().width(4),
            text("Rules").size(11).color(p.text_muted),
        ]
        .align_y(Alignment::Center),
    )
    .style(theme::ghost_button)
    .padding([2, 8])
    .on_press(NotificationMessage::OpenRuleEngine);

    container(
        row![
            text("Online").size(11).color(p.accent_success),
            Space::new().width(16),
            text("Last synced: Just now").size(11).color(p.text_muted),
            Space::new().width(Fill),
            rule_engine_btn,
            Space::new().width(12),
            text("GitTop Enterprise").size(11).color(p.text_muted),
        ]
        .align_y(Alignment::Center)
        .padding([4, 12]),
    )
    .width(Fill)
    .style(move |_| container::Style {
        background: Some(iced::Background::Color(p.bg_card)),
        border: iced::Border {
            width: 1.0,
            color: p.border_subtle,
            radius: 0.0.into(),
        },
        ..Default::default()
    })
    .into()
}
