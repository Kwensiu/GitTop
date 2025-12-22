//! Shared UI components for Rule Engine.

use iced::widget::{button, column, container, row, text, toggler, Space};
use iced::{Alignment, Element, Fill};
use iced_aw::ContextMenu;

use crate::settings::IconTheme;
use crate::ui::screens::settings::rule_engine::rules::{
    AccountRule, OrgRule, ScheduleRule, TimeRule, TypeRule,
};
use crate::ui::{icons, theme};

use super::messages::RuleEngineMessage;

// ============================================================================
// Empty State
// ============================================================================

pub fn view_empty_state(
    message: &'static str,
    icon_theme: IconTheme,
) -> Element<'static, RuleEngineMessage> {
    let p = theme::palette();

    container(
        column![
            icons::icon_inbox_empty(32.0, p.text_muted, icon_theme),
            Space::new().height(8),
            text(message).size(12).color(p.text_muted),
        ]
        .align_x(Alignment::Center)
        .padding(32),
    )
    .width(Fill)
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

// ============================================================================
// Stat Card
// ============================================================================

pub fn view_stat_card(label: &'static str, count: usize) -> Element<'static, RuleEngineMessage> {
    let p = theme::palette();

    container(
        column![
            text(count.to_string()).size(24).color(p.accent),
            text(label).size(11).color(p.text_muted),
        ]
        .align_x(Alignment::Center),
    )
    .padding([12, 16])
    .style(move |_| container::Style {
        background: Some(iced::Background::Color(p.bg_control)),
        border: iced::Border {
            radius: 6.0.into(),
            ..Default::default()
        },
        ..Default::default()
    })
    .into()
}

// ============================================================================
// Context Menu Helpers
// ============================================================================

fn view_context_menu_item(
    label: &'static str,
    message: RuleEngineMessage,
) -> Element<'static, RuleEngineMessage> {
    let p = theme::palette();

    button(text(label).size(12).color(p.text_primary))
        .style(move |_theme, status| {
            let bg = match status {
                iced::widget::button::Status::Hovered => p.bg_hover,
                iced::widget::button::Status::Pressed => p.bg_active,
                _ => p.bg_control,
            };
            iced::widget::button::Style {
                background: Some(iced::Background::Color(bg)),
                ..Default::default()
            }
        })
        .padding([6, 12])
        .width(Fill)
        .on_press(message)
        .into()
}

// ============================================================================
// Rule Cards with Context Menu
// ============================================================================

pub fn view_time_rule_card(rule: &TimeRule) -> Element<'static, RuleEngineMessage> {
    let p = theme::palette();
    let id = rule.id.clone();
    let id_delete = rule.id.clone();
    let id_dup = rule.id.clone();
    let name = rule.name.clone();
    let time_range = format!("{} - {}", rule.start_time, rule.end_time);
    let action = format!("Action: {}", rule.action);
    let enabled = rule.enabled;

    let card_content = container(
        row![
            column![
                text(name).size(14).color(p.text_primary),
                Space::new().height(4),
                text(time_range).size(12).color(p.text_secondary),
                text(action).size(11).color(p.text_muted),
            ]
            .width(Fill),
            toggler(enabled)
                .on_toggle(move |e| RuleEngineMessage::ToggleTimeRule(id.clone(), e))
                .size(18),
        ]
        .align_y(Alignment::Center)
        .padding(14),
    )
    .style(move |_| container::Style {
        background: Some(iced::Background::Color(p.bg_card)),
        border: iced::Border {
            radius: 8.0.into(),
            ..Default::default()
        },
        ..Default::default()
    });

    ContextMenu::new(card_content, move || {
        container(
            column![
                view_context_menu_item(
                    "Duplicate",
                    RuleEngineMessage::DuplicateTimeRule(id_dup.clone())
                ),
                view_context_menu_item(
                    "Delete",
                    RuleEngineMessage::DeleteTimeRule(id_delete.clone())
                ),
            ]
            .spacing(2),
        )
        .style(move |_| container::Style {
            background: Some(iced::Background::Color(p.bg_control)),
            border: iced::Border {
                radius: 6.0.into(),
                color: p.border_subtle,
                width: 1.0,
            },
            shadow: iced::Shadow {
                color: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.3),
                offset: iced::Vector::new(0.0, 2.0),
                blur_radius: 8.0,
            },
            ..Default::default()
        })
        .padding(4)
        .width(140)
        .into()
    })
    .into()
}

pub fn view_schedule_rule_card(rule: &ScheduleRule) -> Element<'static, RuleEngineMessage> {
    let p = theme::palette();
    let id = rule.id.clone();
    let id_delete = rule.id.clone();
    let id_dup = rule.id.clone();
    let name = rule.name.clone();
    let enabled = rule.enabled;
    let action = format!("Action: {}", rule.action);

    let days_text = rule
        .days
        .iter()
        .map(|d| match d {
            0 => "Sun",
            1 => "Mon",
            2 => "Tue",
            3 => "Wed",
            4 => "Thu",
            5 => "Fri",
            6 => "Sat",
            _ => "?",
        })
        .collect::<Vec<_>>()
        .join(", ");

    let card_content = container(
        row![
            column![
                text(name).size(14).color(p.text_primary),
                Space::new().height(4),
                text(days_text).size(12).color(p.text_secondary),
                text(action).size(11).color(p.text_muted),
            ]
            .width(Fill),
            toggler(enabled)
                .on_toggle(move |e| RuleEngineMessage::ToggleScheduleRule(id.clone(), e))
                .size(18),
        ]
        .align_y(Alignment::Center)
        .padding(14),
    )
    .style(move |_| container::Style {
        background: Some(iced::Background::Color(p.bg_card)),
        border: iced::Border {
            radius: 8.0.into(),
            ..Default::default()
        },
        ..Default::default()
    });

    ContextMenu::new(card_content, move || {
        container(
            column![
                view_context_menu_item(
                    "Duplicate",
                    RuleEngineMessage::DuplicateScheduleRule(id_dup.clone())
                ),
                view_context_menu_item(
                    "Delete",
                    RuleEngineMessage::DeleteScheduleRule(id_delete.clone())
                ),
            ]
            .spacing(2),
        )
        .style(move |_| container::Style {
            background: Some(iced::Background::Color(p.bg_control)),
            border: iced::Border {
                radius: 6.0.into(),
                color: p.border_subtle,
                width: 1.0,
            },
            shadow: iced::Shadow {
                color: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.3),
                offset: iced::Vector::new(0.0, 2.0),
                blur_radius: 8.0,
            },
            ..Default::default()
        })
        .padding(4)
        .width(140)
        .into()
    })
    .into()
}

pub fn view_account_rule_card(rule: &AccountRule) -> Element<'static, RuleEngineMessage> {
    let p = theme::palette();
    let id = rule.id.clone();
    let id_delete = rule.id.clone();
    let id_dup = rule.id.clone();
    let account = rule.account.clone();
    let enabled = rule.enabled;
    let action = format!("Action: {}", rule.action);

    let card_content = container(
        row![
            column![
                text(account).size(14).color(p.text_primary),
                text(action).size(11).color(p.text_muted),
            ]
            .width(Fill),
            toggler(enabled)
                .on_toggle(move |e| RuleEngineMessage::ToggleAccountRule(id.clone(), e))
                .size(18),
        ]
        .align_y(Alignment::Center)
        .padding(14),
    )
    .style(move |_| container::Style {
        background: Some(iced::Background::Color(p.bg_card)),
        border: iced::Border {
            radius: 8.0.into(),
            ..Default::default()
        },
        ..Default::default()
    });

    ContextMenu::new(card_content, move || {
        container(
            column![
                view_context_menu_item(
                    "Duplicate",
                    RuleEngineMessage::DuplicateAccountRule(id_dup.clone())
                ),
                view_context_menu_item(
                    "Delete",
                    RuleEngineMessage::DeleteAccountRule(id_delete.clone())
                ),
            ]
            .spacing(2),
        )
        .style(move |_| container::Style {
            background: Some(iced::Background::Color(p.bg_control)),
            border: iced::Border {
                radius: 6.0.into(),
                color: p.border_subtle,
                width: 1.0,
            },
            shadow: iced::Shadow {
                color: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.3),
                offset: iced::Vector::new(0.0, 2.0),
                blur_radius: 8.0,
            },
            ..Default::default()
        })
        .padding(4)
        .width(140)
        .into()
    })
    .into()
}

pub fn view_org_rule_card(rule: &OrgRule) -> Element<'static, RuleEngineMessage> {
    let p = theme::palette();
    let id = rule.id.clone();
    let id_delete = rule.id.clone();
    let id_dup = rule.id.clone();
    let org = rule.org.clone();
    let priority = format!("Priority: {}", rule.priority);
    let action = format!("Action: {}", rule.action);
    let enabled = rule.enabled;

    let card_content = container(
        row![
            column![
                text(org).size(14).color(p.text_primary),
                text(priority).size(12).color(p.text_secondary),
                text(action).size(11).color(p.text_muted),
            ]
            .width(Fill),
            toggler(enabled)
                .on_toggle(move |e| RuleEngineMessage::ToggleOrgRule(id.clone(), e))
                .size(18),
        ]
        .align_y(Alignment::Center)
        .padding(14),
    )
    .style(move |_| container::Style {
        background: Some(iced::Background::Color(p.bg_card)),
        border: iced::Border {
            radius: 8.0.into(),
            ..Default::default()
        },
        ..Default::default()
    });

    ContextMenu::new(card_content, move || {
        container(
            column![
                view_context_menu_item(
                    "Duplicate",
                    RuleEngineMessage::DuplicateOrgRule(id_dup.clone())
                ),
                view_context_menu_item(
                    "Delete",
                    RuleEngineMessage::DeleteOrgRule(id_delete.clone())
                ),
            ]
            .spacing(2),
        )
        .style(move |_| container::Style {
            background: Some(iced::Background::Color(p.bg_control)),
            border: iced::Border {
                radius: 6.0.into(),
                color: p.border_subtle,
                width: 1.0,
            },
            shadow: iced::Shadow {
                color: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.3),
                offset: iced::Vector::new(0.0, 2.0),
                blur_radius: 8.0,
            },
            ..Default::default()
        })
        .padding(4)
        .width(140)
        .into()
    })
    .into()
}

pub fn view_type_rule_card(rule: &TypeRule) -> Element<'static, RuleEngineMessage> {
    let p = theme::palette();
    let id = rule.id.clone();
    let id_delete = rule.id.clone();
    let id_dup = rule.id.clone();
    let notification_type = rule.notification_type.clone();
    let account = rule.account.clone().unwrap_or_else(|| "Global".to_string());
    let priority = format!("Priority: {}", rule.priority);
    let action = format!("Action: {}", rule.action);
    let enabled = rule.enabled;

    let card_content = container(
        row![
            column![
                text(notification_type).size(14).color(p.text_primary),
                Space::new().height(4),
                row![
                    text(account).size(12).color(p.text_secondary),
                    text("•").size(12).color(p.text_muted),
                    text(priority).size(12).color(p.text_secondary),
                ]
                .spacing(6),
                text(action).size(11).color(p.text_muted),
            ]
            .width(Fill),
            toggler(enabled)
                .on_toggle(move |e| RuleEngineMessage::ToggleTypeRule(id.clone(), e))
                .size(18),
        ]
        .align_y(Alignment::Center)
        .padding(14),
    )
    .style(move |_| container::Style {
        background: Some(iced::Background::Color(p.bg_card)),
        border: iced::Border {
            radius: 8.0.into(),
            ..Default::default()
        },
        ..Default::default()
    });

    ContextMenu::new(card_content, move || {
        container(
            column![
                view_context_menu_item(
                    "Duplicate",
                    RuleEngineMessage::DuplicateTypeRule(id_dup.clone())
                ),
                view_context_menu_item(
                    "Delete",
                    RuleEngineMessage::DeleteTypeRule(id_delete.clone())
                ),
            ]
            .spacing(2),
        )
        .style(move |_| container::Style {
            background: Some(iced::Background::Color(p.bg_control)),
            border: iced::Border {
                radius: 6.0.into(),
                color: p.border_subtle,
                width: 1.0,
            },
            shadow: iced::Shadow {
                color: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.3),
                offset: iced::Vector::new(0.0, 2.0),
                blur_radius: 8.0,
            },
            ..Default::default()
        })
        .padding(4)
        .width(140)
        .into()
    })
    .into()
}
