//! Time Rules tab for Rule Engine.

use iced::widget::{button, column, row, text, Space};
use iced::{Alignment, Element, Fill};

use crate::settings::IconTheme;
use crate::ui::screens::settings::rule_engine::rules::NotificationRuleSet;
use crate::ui::theme;

use super::super::components::{view_empty_state, view_time_rule_card};
use super::super::messages::RuleEngineMessage;

pub fn view_time_rules_tab(
    rules: &NotificationRuleSet,
    icon_theme: IconTheme,
) -> Element<'static, RuleEngineMessage> {
    let p = theme::palette();

    let mut content = column![
        text("Time Rules").size(20).color(p.text_primary),
        text("Set quiet hours when notifications are silenced.")
            .size(12)
            .color(p.text_secondary),
        Space::new().height(16),
        button(
            row![
                text("+").size(14).color(iced::Color::WHITE),
                Space::new().width(8),
                text("Add Night Mode Rule")
                    .size(13)
                    .color(iced::Color::WHITE),
            ]
            .align_y(Alignment::Center),
        )
        .style(theme::primary_button)
        .padding([8, 16])
        .on_press(RuleEngineMessage::AddTimeRule),
        Space::new().height(16),
    ]
    .spacing(4);

    if rules.time_rules.is_empty() {
        content = content.push(view_empty_state("No time rules configured", icon_theme));
    } else {
        for rule in &rules.time_rules {
            content = content.push(view_time_rule_card(rule, icon_theme));
            content = content.push(Space::new().height(8));
        }
    }

    content.padding(24).width(Fill).into()
}
