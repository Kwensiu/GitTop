//! Organization Rules tab for Rule Engine.

use iced::widget::{column, text, Space};
use iced::{Element, Fill};

use crate::settings::IconTheme;
use crate::ui::screens::settings::rule_engine::rules::NotificationRuleSet;
use crate::ui::theme;

use super::super::components::{view_empty_state, view_org_rule_card};
use super::super::messages::RuleEngineMessage;

pub fn view_org_rules_tab(
    rules: &NotificationRuleSet,
    icon_theme: IconTheme,
) -> Element<'static, RuleEngineMessage> {
    let p = theme::palette();

    let mut content = column![
        text("Organization Rules").size(20).color(p.text_primary),
        text("Set priority levels for organizations.")
            .size(12)
            .color(p.text_secondary),
        Space::new().height(16),
    ]
    .spacing(4);

    if rules.org_rules.is_empty() {
        content = content.push(view_empty_state(
            "No organization rules configured. Organizations will be auto-discovered from your notifications.",
            icon_theme,
        ));
    } else {
        for rule in &rules.org_rules {
            content = content.push(view_org_rule_card(rule));
            content = content.push(Space::new().height(8));
        }
    }

    content.padding(24).width(Fill).into()
}
