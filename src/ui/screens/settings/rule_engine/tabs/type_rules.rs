//! Type Rules tab for Rule Engine - with grouped collapsible sections.

use std::collections::{HashMap, HashSet};

use iced::widget::{button, column, container, pick_list, row, text, text_input, Space};
use iced::{Alignment, Element, Fill, Length};

use crate::settings::IconTheme;
use crate::ui::icons;
use crate::ui::screens::settings::rule_engine::rules::{NotificationRuleSet, RuleAction, TypeRule};
use crate::ui::theme;

use super::super::components::{view_empty_state, view_type_rule_card};
use super::super::messages::RuleEngineMessage;

/// Groups type rules by notification_type and returns them in a sorted order.
fn group_type_rules(rules: &[TypeRule]) -> Vec<(String, Vec<&TypeRule>)> {
    let mut groups: HashMap<String, Vec<&TypeRule>> = HashMap::new();

    for rule in rules {
        groups
            .entry(rule.notification_type.clone())
            .or_default()
            .push(rule);
    }

    // Sort groups alphabetically by type name
    let mut sorted: Vec<_> = groups.into_iter().collect();
    sorted.sort_by(|a, b| a.0.cmp(&b.0));
    sorted
}

pub fn view_type_rules_tab(
    rules: &NotificationRuleSet,
    icon_theme: IconTheme,
    new_type: crate::github::types::NotificationReason,
    new_account: &str,
    available_accounts: &[String],
    new_priority: i32,
    new_action: RuleAction,
    expanded_groups: &HashSet<String>,
) -> Element<'static, RuleEngineMessage> {
    let p = theme::palette();

    // ========================================================================
    // Form Section
    // ========================================================================
    let type_input = container(
        column![
            text("Type").size(12).color(p.text_secondary),
            pick_list(
                crate::github::types::NotificationReason::ALL,
                Some(new_type),
                RuleEngineMessage::NewTypeRuleTypeChanged
            )
            .width(Length::Fixed(180.0))
            .style(theme::pick_list_style)
            .menu_style(theme::menu_style),
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
            .width(Length::Fixed(150.0))
            .style(theme::pick_list_style)
            .menu_style(theme::menu_style),
        ]
        .spacing(4),
    );

    // Priority Input with Warning
    let priority_label_row = if new_priority > 100 || new_priority < -100 {
        row![
            text("Priority").size(12).color(p.text_secondary),
            Space::new().width(4),
            icons::icon_alert(12.0, p.accent_warning, icon_theme),
            Space::new().width(4),
            text("Non-std").size(10).color(p.accent_warning),
        ]
        .align_y(Alignment::Center)
    } else {
        row![text("Priority").size(12).color(p.text_secondary)]
    };

    let priority_input = container(
        column![
            priority_label_row,
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
                .width(Length::Fixed(80.0))
                .style(theme::text_input_style),
        ]
        .spacing(4),
    );

    // Action Input with Warning
    let action_label_row = if new_action == RuleAction::Hide {
        row![
            text("Action").size(12).color(p.text_secondary),
            Space::new().width(4),
            icons::icon_alert(12.0, p.accent_warning, icon_theme),
        ]
        .align_y(Alignment::Center)
    } else {
        row![text("Action").size(12).color(p.text_secondary)]
    };

    let action_input = container(
        column![
            action_label_row,
            pick_list(
                RuleAction::ALL,
                Some(new_action),
                RuleEngineMessage::NewTypeRuleActionChanged
            )
            .width(Length::Fixed(100.0))
            .style(theme::pick_list_style)
            .menu_style(theme::menu_style),
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
    .align_y(Alignment::End);

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

    // ========================================================================
    // Grouped Rules Section
    // ========================================================================
    let mut content = column![
        header,
        Space::new().height(16),
        form_section,
        Space::new().height(24)
    ];

    if rules.type_rules.is_empty() {
        content = content.push(view_empty_state("No type rules configured.", icon_theme));
    } else {
        // Group rules by notification_type
        let grouped = group_type_rules(&rules.type_rules);

        for (group_name, group_rules) in grouped {
            let is_expanded = expanded_groups.contains(&group_name);
            let count = group_rules.len();

            // Group header with chevron
            let chevron = if is_expanded {
                icons::icon_chevron_down(12.0, p.text_secondary, icon_theme)
            } else {
                icons::icon_chevron_right(12.0, p.text_secondary, icon_theme)
            };

            let group_header_row = row![
                chevron,
                Space::new().width(8),
                text(group_name.clone()).size(14).color(p.text_primary),
                Space::new().width(8),
                text(format!("({} rules)", count))
                    .size(12)
                    .color(p.text_muted),
            ]
            .align_y(Alignment::Center)
            .padding([8, 12]);

            let group_name_clone = group_name.clone();
            let group_header_btn = button(group_header_row)
                .style(move |theme, status| (theme::ghost_button)(theme, status))
                .on_press(RuleEngineMessage::ToggleTypeGroup(group_name_clone))
                .width(Fill);

            let group_header_container =
                container(group_header_btn).style(move |_| container::Style {
                    background: Some(iced::Background::Color(p.bg_control)),
                    border: iced::Border {
                        radius: 6.0.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                });

            content = content.push(group_header_container);
            content = content.push(Space::new().height(4));

            // Show rules only if expanded
            if is_expanded {
                let mut rules_column = column![].spacing(8);

                for rule in group_rules {
                    rules_column = rules_column.push(view_type_rule_card(rule, icon_theme));
                }

                // Add left indent using a row with space
                let indented_rules = row![Space::new().width(24), rules_column];

                content = content.push(indented_rules);
                content = content.push(Space::new().height(8));
            }
        }
    }

    content.padding(24).width(Fill).into()
}
