//! Account Rules tab for Rule Engine.

use iced::widget::{column, text, Space};
use iced::{Element, Fill};

use crate::settings::IconTheme;
use crate::ui::screens::settings::rule_engine::rules::NotificationRuleSet;
use crate::ui::theme;

use super::super::components::{view_account_rule_card, view_empty_state};
use super::super::messages::RuleEngineMessage;

pub fn view_account_rules_tab(
    rules: &NotificationRuleSet,
    icon_theme: IconTheme,
) -> Element<'static, RuleEngineMessage> {
    let p = theme::palette();

    let mut content = column![
        text("Account Rules").size(20).color(p.text_primary),
        text("Configure per-account notification preferences.")
            .size(12)
            .color(p.text_secondary),
        Space::new().height(16),
    ]
    .spacing(4);

    if rules.account_rules.is_empty() {
        content = content.push(view_empty_state(
            "No account rules configured. Account rules will be auto-populated when you add multiple accounts.",
            icon_theme,
        ));
    } else {
        for rule in &rules.account_rules {
            content = content.push(view_account_rule_card(rule));
            content = content.push(Space::new().height(8));
        }
    }

    content.padding(24).width(Fill).into()
}
