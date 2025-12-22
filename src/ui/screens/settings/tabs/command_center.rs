//! Command Center tab - Power Mode and Rule Engine settings.

use iced::widget::{button, column, container, row, text, toggler, Space};
use iced::{Alignment, Element, Fill};

use crate::settings::AppSettings;
use crate::ui::{icons, theme};

use super::super::components::setting_card;
use super::super::messages::SettingsMessage;

/// Render the command center tab content.
pub fn view(settings: &AppSettings) -> Element<'_, SettingsMessage> {
    let p = theme::palette();
    let power_mode_enabled = settings.power_mode;

    let mut content = column![
        tab_title("Command Center"),
        text("Enterprise-grade notification management.")
            .size(12)
            .color(p.text_secondary),
        Space::new().height(16),
        view_power_mode_setting(settings),
    ]
    .spacing(4);

    // Show Rule Engine button only when Power Mode is enabled
    if power_mode_enabled {
        content = content.push(Space::new().height(16));
        content = content.push(view_rule_engine_section(settings));
    }

    content.padding(24).width(Fill).into()
}

fn tab_title(title: &'static str) -> Element<'static, SettingsMessage> {
    text(title)
        .size(20)
        .color(theme::palette().text_primary)
        .into()
}

fn view_power_mode_setting(settings: &AppSettings) -> Element<'_, SettingsMessage> {
    let p = theme::palette();
    let enabled = settings.power_mode;
    let icon_theme = settings.icon_theme;

    let description = if enabled {
        "Enterprise Command Center layout enabled"
    } else {
        "Simple single-pane layout"
    };

    setting_card(
        row![
            column![
                row![
                    icons::icon_power(14.0, p.accent, icon_theme),
                    Space::new().width(8),
                    text("Power Mode").size(14).color(p.text_primary),
                ]
                .align_y(Alignment::Center),
                Space::new().height(4),
                text(description).size(11).color(p.text_secondary),
            ]
            .width(Fill),
            toggler(enabled)
                .on_toggle(SettingsMessage::TogglePowerMode)
                .size(20),
        ]
        .align_y(Alignment::Center),
    )
}

fn view_rule_engine_section(settings: &AppSettings) -> Element<'_, SettingsMessage> {
    let p = theme::palette();
    let icon_theme = settings.icon_theme;

    container(
        column![
            row![
                icons::icon_filter(14.0, p.accent, icon_theme),
                Space::new().width(8),
                text("Rule Engine").size(14).color(p.text_primary),
            ]
            .align_y(Alignment::Center),
            Space::new().height(4),
            text("Configure advanced notification filtering rules.")
                .size(11)
                .color(p.text_secondary),
            Space::new().height(12),
            button(
                row![
                    icons::icon_external_link(14.0, iced::Color::WHITE, icon_theme),
                    Space::new().width(8),
                    text("Open Rule Engine").size(13).color(iced::Color::WHITE),
                ]
                .align_y(Alignment::Center),
            )
            .style(theme::primary_button)
            .padding([8, 16])
            .on_press(SettingsMessage::OpenRuleEngine),
        ]
        .padding(14),
    )
    .style(move |_| container::Style {
        background: Some(iced::Background::Color(p.bg_card)),
        border: iced::Border {
            radius: 8.0.into(),
            width: 1.0,
            color: p.accent.scale_alpha(0.3),
        },
        ..Default::default()
    })
    .into()
}
