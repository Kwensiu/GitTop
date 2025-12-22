//! Shared UI components for Rule Engine.

use iced::widget::{button, column, container, row, text, toggler, Space};
use iced::{Alignment, Element, Fill};
use iced_aw::ContextMenu;

use crate::settings::IconTheme;
use crate::ui::screens::settings::rule_engine::rules::{
    AccountRule, OrgRule, RuleAction, ScheduleRule, TimeRule, TypeRule,
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
// Warning Row Helper
// ============================================================================

fn view_warning_row(
    message: &'static str,
    icon_theme: IconTheme,
) -> Element<'static, RuleEngineMessage> {
    let p = theme::palette();
    row![
        icons::icon_alert(12.0, p.accent_warning, icon_theme),
        Space::new().width(4),
        text(message).size(11).color(p.accent_warning),
    ]
    .align_y(Alignment::Center)
    .into()
}

// ============================================================================
// Generic Rule Card Wrapper
// ============================================================================

/// Creates a rule card with consistent styling, toggler, and context menu.
///
/// This extracts the common pattern shared by all rule card types:
/// - Card container with bg_card background and rounded corners
/// - Info content on the left, toggler on the right
/// - Context menu with Duplicate and Delete options
fn view_rule_card<F1, F2, F3>(
    id: String,
    enabled: bool,
    info_content: Element<'static, RuleEngineMessage>,
    on_toggle: F1,
    on_duplicate: F2,
    on_delete: F3,
) -> Element<'static, RuleEngineMessage>
where
    F1: Fn(String, bool) -> RuleEngineMessage + 'static + Clone,
    F2: Fn(String) -> RuleEngineMessage + 'static + Clone,
    F3: Fn(String) -> RuleEngineMessage + 'static + Clone,
{
    let id_toggle = id.clone();
    let id_dup = id.clone();
    let id_delete = id;

    let card_content = container(
        row![
            info_content,
            toggler(enabled)
                .on_toggle(move |e| on_toggle(id_toggle.clone(), e))
                .size(18),
        ]
        .align_y(Alignment::Center)
        .padding(14),
    )
    .style(|_| theme::rule_card_container());

    ContextMenu::new(card_content, move || {
        container(
            column![
                view_context_menu_item("Duplicate", on_duplicate(id_dup.clone())),
                view_context_menu_item("Delete", on_delete(id_delete.clone())),
            ]
            .spacing(2),
        )
        .style(|_| theme::context_menu_container())
        .padding(4)
        .width(140)
        .into()
    })
    .into()
}

// ============================================================================
// Rule Cards
// ============================================================================

pub fn view_time_rule_card(
    rule: &TimeRule,
    icon_theme: IconTheme,
) -> Element<'static, RuleEngineMessage> {
    let p = theme::palette();

    let time_range = format!("{} - {}", rule.start_time, rule.end_time);
    let action_str = format!("Action: {}", rule.action.display_label());

    let mut info_column = column![
        text(rule.name.clone()).size(14).color(p.text_primary),
        Space::new().height(4),
        text(time_range).size(12).color(p.text_secondary),
        text(action_str).size(11).color(p.text_muted),
    ]
    .width(Fill);

    if rule.action == RuleAction::Hide {
        info_column = info_column.push(Space::new().height(4));
        info_column = info_column.push(view_warning_row("Hides notifications", icon_theme));
    }

    view_rule_card(
        rule.id.clone(),
        rule.enabled,
        info_column.into(),
        RuleEngineMessage::ToggleTimeRule,
        RuleEngineMessage::DuplicateTimeRule,
        RuleEngineMessage::DeleteTimeRule,
    )
}

pub fn view_schedule_rule_card(
    rule: &ScheduleRule,
    icon_theme: IconTheme,
) -> Element<'static, RuleEngineMessage> {
    let p = theme::palette();

    let action_str = format!("Action: {}", rule.action.display_label());

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

    let mut info_column = column![
        text(rule.name.clone()).size(14).color(p.text_primary),
        Space::new().height(4),
        text(days_text).size(12).color(p.text_secondary),
        text(action_str).size(11).color(p.text_muted),
    ]
    .width(Fill);

    if rule.action == RuleAction::Hide {
        info_column = info_column.push(Space::new().height(4));
        info_column = info_column.push(view_warning_row("Hides notifications", icon_theme));
    }

    view_rule_card(
        rule.id.clone(),
        rule.enabled,
        info_column.into(),
        RuleEngineMessage::ToggleScheduleRule,
        RuleEngineMessage::DuplicateScheduleRule,
        RuleEngineMessage::DeleteScheduleRule,
    )
}

pub fn view_account_rule_card(
    rule: &AccountRule,
    icon_theme: IconTheme,
) -> Element<'static, RuleEngineMessage> {
    let p = theme::palette();

    let action_str = format!("Action: {}", rule.action.display_label());

    let mut info_column = column![
        text(rule.account.clone()).size(14).color(p.text_primary),
        text(action_str).size(11).color(p.text_muted),
    ]
    .width(Fill);

    if rule.action == RuleAction::Hide {
        info_column = info_column.push(Space::new().height(4));
        info_column = info_column.push(view_warning_row("Hides notifications", icon_theme));
    }

    view_rule_card(
        rule.id.clone(),
        rule.enabled,
        info_column.into(),
        RuleEngineMessage::ToggleAccountRule,
        RuleEngineMessage::DuplicateAccountRule,
        RuleEngineMessage::DeleteAccountRule,
    )
}

pub fn view_org_rule_card(
    rule: &OrgRule,
    icon_theme: IconTheme,
) -> Element<'static, RuleEngineMessage> {
    let p = theme::palette();

    let priority = format!("Priority: {}", rule.priority);
    let action_str = format!("Action: {}", rule.action.display_label());

    let mut info_column = column![
        text(rule.org.clone()).size(14).color(p.text_primary),
        text(priority).size(12).color(p.text_secondary),
        text(action_str).size(11).color(p.text_muted),
    ]
    .width(Fill);

    if rule.priority > 100 || rule.priority < -100 {
        info_column = info_column.push(Space::new().height(4));
        info_column = info_column.push(view_warning_row("Non-standard priority", icon_theme));
    }
    if rule.action == RuleAction::Hide {
        info_column = info_column.push(Space::new().height(4));
        info_column = info_column.push(view_warning_row("Hides notifications", icon_theme));
    }

    view_rule_card(
        rule.id.clone(),
        rule.enabled,
        info_column.into(),
        RuleEngineMessage::ToggleOrgRule,
        RuleEngineMessage::DuplicateOrgRule,
        RuleEngineMessage::DeleteOrgRule,
    )
}

pub fn view_type_rule_card(
    rule: &TypeRule,
    icon_theme: IconTheme,
) -> Element<'static, RuleEngineMessage> {
    let p = theme::palette();

    let account = rule.account.clone().unwrap_or_else(|| "Global".to_string());
    let priority = format!("Priority: {}", rule.priority);
    let action_str = format!("Action: {}", rule.action.display_label());

    let mut info_column = column![
        text(rule.notification_type.clone())
            .size(14)
            .color(p.text_primary),
        Space::new().height(4),
        row![
            text(account).size(12).color(p.text_secondary),
            text("•").size(12).color(p.text_muted),
            text(priority).size(12).color(p.text_secondary),
        ]
        .spacing(6),
        text(action_str).size(11).color(p.text_muted),
    ]
    .width(Fill);

    if rule.priority > 100 || rule.priority < -100 {
        info_column = info_column.push(Space::new().height(4));
        info_column = info_column.push(view_warning_row("Non-standard priority", icon_theme));
    }
    if rule.action == RuleAction::Hide {
        info_column = info_column.push(Space::new().height(4));
        info_column = info_column.push(view_warning_row("Hides notifications", icon_theme));
    }

    view_rule_card(
        rule.id.clone(),
        rule.enabled,
        info_column.into(),
        RuleEngineMessage::ToggleTypeRule,
        RuleEngineMessage::DuplicateTypeRule,
        RuleEngineMessage::DeleteTypeRule,
    )
}
