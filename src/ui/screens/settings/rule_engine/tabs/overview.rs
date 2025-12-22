//! Overview tab for Rule Engine - System health and high-impact rules.

use chrono::Local;
use iced::widget::{column, container, row, text, Space};
use iced::{Element, Fill};

use crate::settings::IconTheme;
use crate::ui::icons;
use crate::ui::screens::settings::rule_engine::rules::{NotificationRuleSet, RuleAction};
use crate::ui::theme;

use super::super::components::view_stat_card;
use super::super::messages::RuleEngineMessage;

pub fn view_overview_tab(
    rules: &NotificationRuleSet,
    icon_theme: IconTheme,
) -> Element<'static, RuleEngineMessage> {
    let p = theme::palette();
    let now = Local::now();

    // System health stats
    let active_count = rules.active_rule_count();
    let suppress_count = rules.count_suppress_rules();
    let high_priority_count = rules.count_high_priority_rules();
    let time_based_active = rules.count_active_time_based_rules(&now);

    // ========================================================================
    // Header
    // ========================================================================
    let header = column![
        text("Overview").size(20).color(p.text_primary),
        text("System health and high-impact rules at a glance.")
            .size(12)
            .color(p.text_secondary),
    ]
    .spacing(4);

    // ========================================================================
    // System Health Section
    // ========================================================================
    let health_title = text("System Health").size(14).color(p.text_primary);

    let status_badge = if rules.enabled {
        row![
            icons::icon_check(12.0, p.accent_success, icon_theme),
            Space::new().width(4),
            text("Rule Engine Active").size(12).color(p.accent_success),
        ]
        .align_y(iced::Alignment::Center)
    } else {
        row![
            icons::icon_x(12.0, p.text_muted, icon_theme),
            Space::new().width(4),
            text("Rule Engine Disabled").size(12).color(p.text_muted),
        ]
        .align_y(iced::Alignment::Center)
    };

    // Stat items with icons
    let active_stat = row![
        icons::icon_check(12.0, p.accent, icon_theme),
        Space::new().width(6),
        text(format!("{} active rules", active_count))
            .size(12)
            .color(p.text_primary),
    ]
    .align_y(iced::Alignment::Center);

    let suppress_stat = if suppress_count > 0 {
        row![
            icons::icon_alert(12.0, p.accent_warning, icon_theme),
            Space::new().width(6),
            text(format!("{} suppress notifications", suppress_count))
                .size(12)
                .color(p.accent_warning),
        ]
        .align_y(iced::Alignment::Center)
    } else {
        row![
            icons::icon_check(12.0, p.text_muted, icon_theme),
            Space::new().width(6),
            text("No suppression rules").size(12).color(p.text_muted),
        ]
        .align_y(iced::Alignment::Center)
    };

    let priority_stat = if high_priority_count > 0 {
        row![
            icons::icon_zap(12.0, p.accent_warning, icon_theme),
            Space::new().width(6),
            text(format!("{} priority overrides", high_priority_count))
                .size(12)
                .color(p.accent_warning),
        ]
        .align_y(iced::Alignment::Center)
    } else {
        row![
            icons::icon_check(12.0, p.text_muted, icon_theme),
            Space::new().width(6),
            text("No priority overrides").size(12).color(p.text_muted),
        ]
        .align_y(iced::Alignment::Center)
    };

    let time_stat = if time_based_active > 0 {
        row![
            icons::icon_clock(12.0, p.accent, icon_theme),
            Space::new().width(6),
            text(format!("{} time-based active now", time_based_active))
                .size(12)
                .color(p.accent),
        ]
        .align_y(iced::Alignment::Center)
    } else {
        row![
            icons::icon_clock(12.0, p.text_muted, icon_theme),
            Space::new().width(6),
            text("No time-based rules active")
                .size(12)
                .color(p.text_muted),
        ]
        .align_y(iced::Alignment::Center)
    };

    let health_content = column![
        status_badge,
        Space::new().height(12),
        active_stat,
        Space::new().height(6),
        suppress_stat,
        Space::new().height(6),
        priority_stat,
        Space::new().height(6),
        time_stat,
    ];

    let health_section =
        container(column![health_title, Space::new().height(12), health_content,].padding(16))
            .style(move |_| container::Style {
                background: Some(iced::Background::Color(p.bg_card)),
                border: iced::Border {
                    radius: 8.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            });

    // ========================================================================
    // Rule Counts Section
    // ========================================================================
    let counts_title = text("Rule Counts").size(14).color(p.text_primary);

    let counts_row1 = row![
        view_stat_card("Time Rules", rules.time_rules.len()),
        Space::new().width(8),
        view_stat_card("Schedule Rules", rules.schedule_rules.len()),
        Space::new().width(8),
        view_stat_card("Account Rules", rules.account_rules.len()),
    ];

    let counts_row2 = row![
        view_stat_card("Org Rules", rules.org_rules.len()),
        Space::new().width(8),
        view_stat_card("Type Rules", rules.type_rules.len()),
    ];

    let counts_section = container(
        column![
            counts_title,
            Space::new().height(12),
            counts_row1,
            Space::new().height(8),
            counts_row2,
        ]
        .padding(16),
    )
    .style(move |_| container::Style {
        background: Some(iced::Background::Color(p.bg_card)),
        border: iced::Border {
            radius: 8.0.into(),
            ..Default::default()
        },
        ..Default::default()
    });

    // ========================================================================
    // High-Impact Rules Section
    // ========================================================================
    let high_impact_rules = rules.get_high_impact_rules();

    let high_impact_section = if high_impact_rules.is_empty() {
        container(
            column![
                row![
                    icons::icon_alert(14.0, p.text_muted, icon_theme),
                    Space::new().width(8),
                    text("High-Impact Rules").size(14).color(p.text_primary),
                ]
                .align_y(iced::Alignment::Center),
                Space::new().height(12),
                text("No high-impact rules active.")
                    .size(12)
                    .color(p.text_muted),
                text("Rules that hide notifications or have high priority will appear here.")
                    .size(11)
                    .color(p.text_muted),
            ]
            .padding(16),
        )
        .style(move |_| container::Style {
            background: Some(iced::Background::Color(p.bg_card)),
            border: iced::Border {
                radius: 8.0.into(),
                ..Default::default()
            },
            ..Default::default()
        })
    } else {
        let high_impact_title = row![
            icons::icon_alert(14.0, p.accent_warning, icon_theme),
            Space::new().width(8),
            text("High-Impact Rules").size(14).color(p.text_primary),
            Space::new().width(8),
            text(format!("({})", high_impact_rules.len()))
                .size(12)
                .color(p.text_muted),
        ]
        .align_y(iced::Alignment::Center);

        let mut rules_list = column![].spacing(6);

        for rule in high_impact_rules.iter().take(10) {
            // Limit display to 10
            let action_color = match rule.action {
                RuleAction::Hide => p.accent_warning,
                RuleAction::Priority => p.accent_warning,
                _ => p.text_secondary,
            };

            let action_icon = match rule.action {
                RuleAction::Hide => icons::icon_eye_off(11.0, action_color, icon_theme),
                RuleAction::Priority => icons::icon_zap(11.0, action_color, icon_theme),
                _ => icons::icon_check(11.0, action_color, icon_theme),
            };

            // Clone strings to avoid lifetime issues with local variable
            let rule_name = rule.name.clone();
            let rule_type = rule.rule_type.clone();
            let action_label = rule.action.display_label();

            let rule_row = row![
                action_icon,
                Space::new().width(6),
                text(rule_name).size(12).color(p.text_primary),
                Space::new().width(8),
                text(rule_type).size(10).color(p.text_muted),
                Space::new().width(Fill),
                text(action_label).size(11).color(action_color),
            ]
            .align_y(iced::Alignment::Center)
            .padding([4, 8]);

            let rule_container = container(rule_row).style(move |_| container::Style {
                background: Some(iced::Background::Color(p.bg_control)),
                border: iced::Border {
                    radius: 4.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            });

            rules_list = rules_list.push(rule_container);
        }

        if high_impact_rules.len() > 10 {
            rules_list = rules_list.push(
                text(format!("... and {} more", high_impact_rules.len() - 10))
                    .size(11)
                    .color(p.text_muted),
            );
        }

        container(column![high_impact_title, Space::new().height(12), rules_list,].padding(16))
            .style(move |_| container::Style {
                background: Some(iced::Background::Color(p.bg_card)),
                border: iced::Border {
                    radius: 8.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            })
    };

    // ========================================================================
    // Assemble Layout
    // ========================================================================
    column![
        header,
        Space::new().height(24),
        health_section,
        Space::new().height(16),
        counts_section,
        Space::new().height(16),
        high_impact_section,
    ]
    .spacing(0)
    .padding(24)
    .width(Fill)
    .into()
}
