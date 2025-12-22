//! Overview tab for Rule Engine.

use iced::widget::{column, container, row, text, Space};
use iced::{Element, Fill};

use crate::ui::screens::settings::rule_engine::rules::NotificationRuleSet;
use crate::ui::theme;

use super::super::components::view_stat_card;
use super::super::messages::RuleEngineMessage;

pub fn view_overview_tab(rules: &NotificationRuleSet) -> Element<'static, RuleEngineMessage> {
    let p = theme::palette();
    let active_count = rules.active_rule_count();

    let status_text = if rules.enabled {
        format!("{} active rules", active_count)
    } else {
        "Rule Engine is disabled".to_string()
    };

    column![
        text("Overview").size(20).color(p.text_primary),
        text("Manage your notification filtering rules.")
            .size(12)
            .color(p.text_secondary),
        Space::new().height(24),
        container(
            column![
                text("Status").size(14).color(p.text_primary),
                Space::new().height(8),
                text(status_text).size(12).color(p.text_muted),
                Space::new().height(16),
                row![
                    view_stat_card("Time Rules", rules.time_rules.len()),
                    Space::new().width(8),
                    view_stat_card("Schedule Rules", rules.schedule_rules.len()),
                    Space::new().width(8),
                    view_stat_card("Account Rules", rules.account_rules.len()),
                ],
                Space::new().height(8),
                row![
                    view_stat_card("Org Rules", rules.org_rules.len()),
                    Space::new().width(8),
                    view_stat_card("Type Rules", rules.type_rules.len()),
                ],
            ]
            .padding(16)
        )
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
