//! Type Rules tab for Rule Engine.

use iced::widget::{column, container, text, Space};
use iced::{Alignment, Element, Fill};

use crate::settings::IconTheme;
use crate::ui::screens::settings::rule_engine::rules::NotificationRuleSet;
use crate::ui::theme;

use super::super::components::{view_empty_state, view_type_rule_card};
use super::super::messages::RuleEngineMessage;

use iced::widget::{button, pick_list, row, text_input};
use iced::Length;

use crate::ui::screens::settings::rule_engine::rules::RuleAction;

pub fn view_type_rules_tab(
    rules: &NotificationRuleSet,
    icon_theme: IconTheme,
    new_type: crate::github::types::NotificationReason,
    new_account: &str,
    available_accounts: &[String],
    new_priority: i32,
    new_action: RuleAction,
) -> Element<'static, RuleEngineMessage> {
    let p = theme::palette();

    // Form Section
    let type_input = container(
        column![
            text("Type").size(12).color(p.text_secondary),
            pick_list(
                crate::github::types::NotificationReason::ALL,
                Some(new_type),
                RuleEngineMessage::NewTypeRuleTypeChanged
            )
            .width(Length::Fixed(180.0)),
        ]
        .spacing(4),
    );

    let account_input = container(
        column![
            text("Account").size(12).color(p.text_secondary),
            pick_list(
                {
                    let mut options = vec!["Global".to_string()];
                    options.extend_from_slice(available_accounts);
                    options
                },
                Some(new_account.to_string()),
                RuleEngineMessage::NewTypeRuleAccountChanged
            )
            .width(Length::Fixed(150.0)),
        ]
        .spacing(4),
    );

    let priority_input = container(
        column![
            text("Priority").size(12).color(p.text_secondary),
            text_input("0", &new_priority.to_string())
                .on_input(move |s| {
                    if let Ok(val) = s.parse::<i32>() {
                        RuleEngineMessage::NewTypeRulePriorityChanged(val)
                    } else if s.is_empty() || s == "-" {
                        RuleEngineMessage::NewTypeRulePriorityChanged(0)
                    } else {
                        RuleEngineMessage::NewTypeRulePriorityChanged(new_priority)
                    }
                })
                .width(Length::Fixed(80.0)),
        ]
        .spacing(4),
    );

    let action_input = container(
        column![
            text("Action").size(12).color(p.text_secondary),
            pick_list(
                RuleAction::ALL,
                Some(new_action),
                RuleEngineMessage::NewTypeRuleActionChanged
            )
            .width(Length::Fixed(100.0)),
        ]
        .spacing(4),
    );

    let add_btn = button(text("Add Rule").size(13))
        .style(theme::primary_button)
        .on_press(RuleEngineMessage::AddTypeRule)
        .padding([8, 16]);

    let form_row = row![
        type_input,
        account_input,
        priority_input,
        action_input,
        Space::new().width(Fill),
        column![Space::new().height(19), add_btn].spacing(0),
    ]
    .spacing(12)
    .align_y(Alignment::End); // Align to bottom so inputs align with button

    let form_section = container(form_row)
        .padding(16)
        .style(move |_| container::Style {
            background: Some(iced::Background::Color(p.bg_control)),
            border: iced::Border {
                radius: 8.0.into(),
                ..Default::default()
            },
            ..Default::default()
        });

    let header = column![
        text("Type Rules").size(20).color(p.text_primary),
        text("Filter notifications by type, account, and priority.")
            .size(12)
            .color(p.text_secondary),
    ]
    .spacing(4);

    let mut content = column![
        header,
        Space::new().height(16),
        form_section,
        Space::new().height(24)
    ];

    if rules.type_rules.is_empty() {
        content = content.push(view_empty_state("No type rules configured.", icon_theme));
    } else {
        for rule in &rules.type_rules {
            content = content.push(view_type_rule_card(rule));
            content = content.push(Space::new().height(8));
        }
    }

    content.padding(24).width(Fill).into()
}
