//! Rule Engine screen - main state and layout.

use iced::widget::{button, column, container, row, scrollable, text, toggler, Space};
use iced::{Alignment, Element, Fill, Length, Task};

use crate::settings::{AppSettings, IconTheme};
use crate::ui::screens::settings::rule_engine::rules::{
    NotificationRuleSet, RuleAction, ScheduleRule, TimeRule, TypeRule,
};
use crate::ui::{icons, theme};

use super::messages::{RuleEngineMessage, RuleTab};
use super::tabs;

/// Rule Engine screen state.
#[derive(Debug, Clone)]
pub struct RuleEngineScreen {
    pub rules: NotificationRuleSet,
    pub selected_tab: RuleTab,
    pub icon_theme: IconTheme,
    pub sidebar_width: f32,
    pub sidebar_font_scale: f32,
    pub accounts: Vec<String>,

    // Type Rule Creation State
    pub new_type_rule_type: crate::github::types::NotificationReason,
    pub new_type_rule_account: String,
    pub new_type_rule_priority: i32,
    pub new_type_rule_action: RuleAction,
}

impl RuleEngineScreen {
    pub fn new(rules: NotificationRuleSet, settings: AppSettings) -> Self {
        Self {
            rules,
            selected_tab: RuleTab::default(),
            icon_theme: settings.icon_theme,
            sidebar_width: settings.sidebar_width,
            sidebar_font_scale: settings.sidebar_font_scale,
            accounts: settings
                .accounts
                .iter()
                .map(|a| a.username.clone())
                .collect(),
            new_type_rule_type: crate::github::types::NotificationReason::Mention, // Default type
            new_type_rule_account: "Global".to_string(),
            new_type_rule_priority: 0,
            new_type_rule_action: RuleAction::Show,
        }
    }

    pub fn update(&mut self, message: RuleEngineMessage) -> Task<RuleEngineMessage> {
        match message {
            RuleEngineMessage::Back => Task::none(),
            RuleEngineMessage::SelectTab(tab) => {
                self.selected_tab = tab;
                Task::none()
            }
            RuleEngineMessage::ToggleEnabled(enabled) => {
                self.rules.enabled = enabled;
                let _ = self.rules.save();
                Task::none()
            }

            // ================================================================
            // Time Rules
            // ================================================================
            RuleEngineMessage::ToggleTimeRule(id, enabled) => {
                if let Some(rule) = self.rules.time_rules.iter_mut().find(|r| r.id == id) {
                    rule.enabled = enabled;
                }
                let _ = self.rules.save();
                Task::none()
            }
            RuleEngineMessage::AddTimeRule => {
                let rule = TimeRule::new("Night Mode", "22:00", "07:00");
                self.rules.time_rules.push(rule);
                let _ = self.rules.save();
                Task::none()
            }
            RuleEngineMessage::DeleteTimeRule(id) => {
                self.rules.time_rules.retain(|r| r.id != id);
                let _ = self.rules.save();
                Task::none()
            }
            RuleEngineMessage::DuplicateTimeRule(id) => {
                if let Some(rule) = self.rules.time_rules.iter().find(|r| r.id == id).cloned() {
                    let mut new_rule = rule;
                    new_rule.id = uuid::Uuid::new_v4().to_string();
                    new_rule.name = format!("{} (Copy)", new_rule.name);
                    self.rules.time_rules.push(new_rule);
                    let _ = self.rules.save();
                }
                Task::none()
            }

            // ================================================================
            // Schedule Rules
            // ================================================================
            RuleEngineMessage::ToggleScheduleRule(id, enabled) => {
                if let Some(rule) = self.rules.schedule_rules.iter_mut().find(|r| r.id == id) {
                    rule.enabled = enabled;
                }
                let _ = self.rules.save();
                Task::none()
            }
            RuleEngineMessage::AddWeekendRule => {
                let rule = ScheduleRule::weekend_silent();
                self.rules.schedule_rules.push(rule);
                let _ = self.rules.save();
                Task::none()
            }
            RuleEngineMessage::DeleteScheduleRule(id) => {
                self.rules.schedule_rules.retain(|r| r.id != id);
                let _ = self.rules.save();
                Task::none()
            }
            RuleEngineMessage::DuplicateScheduleRule(id) => {
                if let Some(rule) = self
                    .rules
                    .schedule_rules
                    .iter()
                    .find(|r| r.id == id)
                    .cloned()
                {
                    let mut new_rule = rule;
                    new_rule.id = uuid::Uuid::new_v4().to_string();
                    new_rule.name = format!("{} (Copy)", new_rule.name);
                    self.rules.schedule_rules.push(new_rule);
                    let _ = self.rules.save();
                }
                Task::none()
            }

            // ================================================================
            // Account Rules
            // ================================================================
            RuleEngineMessage::ToggleAccountRule(id, enabled) => {
                if let Some(rule) = self.rules.account_rules.iter_mut().find(|r| r.id == id) {
                    rule.enabled = enabled;
                }
                let _ = self.rules.save();
                Task::none()
            }
            RuleEngineMessage::DeleteAccountRule(id) => {
                self.rules.account_rules.retain(|r| r.id != id);
                let _ = self.rules.save();
                Task::none()
            }
            RuleEngineMessage::DuplicateAccountRule(id) => {
                if let Some(rule) = self
                    .rules
                    .account_rules
                    .iter()
                    .find(|r| r.id == id)
                    .cloned()
                {
                    let mut new_rule = rule;
                    new_rule.id = uuid::Uuid::new_v4().to_string();
                    self.rules.account_rules.push(new_rule);
                    let _ = self.rules.save();
                }
                Task::none()
            }

            // ================================================================
            // Org Rules
            // ================================================================
            RuleEngineMessage::ToggleOrgRule(id, enabled) => {
                if let Some(rule) = self.rules.org_rules.iter_mut().find(|r| r.id == id) {
                    rule.enabled = enabled;
                }
                let _ = self.rules.save();
                Task::none()
            }
            RuleEngineMessage::DeleteOrgRule(id) => {
                self.rules.org_rules.retain(|r| r.id != id);
                let _ = self.rules.save();
                Task::none()
            }
            RuleEngineMessage::DuplicateOrgRule(id) => {
                if let Some(rule) = self.rules.org_rules.iter().find(|r| r.id == id).cloned() {
                    let mut new_rule = rule;
                    new_rule.id = uuid::Uuid::new_v4().to_string();
                    self.rules.org_rules.push(new_rule);
                    let _ = self.rules.save();
                }
                Task::none()
            }

            // ================================================================
            // Type Rules
            // ================================================================
            RuleEngineMessage::ToggleTypeRule(id, enabled) => {
                if let Some(rule) = self.rules.type_rules.iter_mut().find(|r| r.id == id) {
                    rule.enabled = enabled;
                }
                let _ = self.rules.save();
                Task::none()
            }
            RuleEngineMessage::DeleteTypeRule(id) => {
                self.rules.type_rules.retain(|r| r.id != id);
                let _ = self.rules.save();
                Task::none()
            }
            RuleEngineMessage::DuplicateTypeRule(id) => {
                if let Some(rule) = self.rules.type_rules.iter().find(|r| r.id == id).cloned() {
                    let mut new_rule = rule;
                    new_rule.id = uuid::Uuid::new_v4().to_string();
                    self.rules.type_rules.push(new_rule);
                    let _ = self.rules.save();
                }
                Task::none()
            }
            RuleEngineMessage::NewTypeRuleTypeChanged(s) => {
                self.new_type_rule_type = s;
                Task::none()
            }
            RuleEngineMessage::NewTypeRuleAccountChanged(s) => {
                self.new_type_rule_account = s;
                Task::none()
            }
            RuleEngineMessage::NewTypeRulePriorityChanged(p) => {
                self.new_type_rule_priority = p;
                Task::none()
            }
            RuleEngineMessage::NewTypeRuleActionChanged(a) => {
                self.new_type_rule_action = a;
                Task::none()
            }
            RuleEngineMessage::AddTypeRule => {
                let account = if self.new_type_rule_account == "Global"
                    || self.new_type_rule_account.trim().is_empty()
                {
                    None
                } else {
                    Some(self.new_type_rule_account.clone())
                };

                let mut rule = TypeRule::new(
                    self.new_type_rule_type.label(),
                    account,
                    self.new_type_rule_priority,
                );
                rule.action = self.new_type_rule_action;

                self.rules.type_rules.push(rule);
                let _ = self.rules.save();

                // Reset form (keep type for convenience, maybe?)
                self.new_type_rule_account = "Global".to_string();
                self.new_type_rule_priority = 0;
                self.new_type_rule_action = RuleAction::Show;

                Task::none()
            }
        }
    }

    // ========================================================================
    // Main Layout
    // ========================================================================

    pub fn view(&self) -> Element<'_, RuleEngineMessage> {
        let header = self.view_header();
        let sidebar = self.view_sidebar();
        let content = self.view_tab_content();

        let main_area = row![sidebar, content].height(Fill);

        column![header, main_area]
            .spacing(0)
            .width(Fill)
            .height(Fill)
            .into()
    }

    fn view_header(&self) -> Element<'_, RuleEngineMessage> {
        let p = theme::palette();

        let back_btn = button(
            row![
                icons::icon_chevron_left(16.0, p.text_secondary, self.icon_theme),
                Space::new().width(4),
                text("Back").size(13).color(p.text_secondary),
            ]
            .align_y(Alignment::Center),
        )
        .style(theme::ghost_button)
        .padding([6, 10])
        .on_press(RuleEngineMessage::Back);

        let title = row![
            icons::icon_filter(18.0, p.accent, self.icon_theme),
            Space::new().width(8),
            text("Rule Engine").size(18).color(p.text_primary),
        ]
        .align_y(Alignment::Center);

        let enabled_toggle = row![
            text("Enabled").size(12).color(p.text_secondary),
            Space::new().width(8),
            toggler(self.rules.enabled)
                .on_toggle(RuleEngineMessage::ToggleEnabled)
                .size(18),
        ]
        .align_y(Alignment::Center);

        let header_row = row![
            back_btn,
            Space::new().width(Fill),
            title,
            Space::new().width(Fill),
            enabled_toggle,
        ]
        .align_y(Alignment::Center)
        .padding([12, 16]);

        container(header_row)
            .width(Fill)
            .style(theme::header)
            .into()
    }

    // ========================================================================
    // Sidebar Navigation
    // ========================================================================

    fn view_sidebar(&self) -> Element<'_, RuleEngineMessage> {
        let p = theme::palette();
        let t = self.icon_theme;
        let scale = self.sidebar_font_scale;

        // Base sizes
        let icon_size = 14.0 * scale;

        let nav_items = column![
            self.view_nav_item(
                "Overview",
                RuleTab::Overview,
                icons::icon_chart(icon_size, self.nav_icon_color(RuleTab::Overview), t)
            ),
            self.view_nav_item(
                "Time Rules",
                RuleTab::TimeRules,
                icons::icon_clock(icon_size, self.nav_icon_color(RuleTab::TimeRules), t)
            ),
            self.view_nav_item(
                "Schedule",
                RuleTab::ScheduleRules,
                icons::icon_calendar(icon_size, self.nav_icon_color(RuleTab::ScheduleRules), t)
            ),
            self.view_nav_item(
                "Accounts",
                RuleTab::AccountRules,
                icons::icon_user(icon_size, self.nav_icon_color(RuleTab::AccountRules), t)
            ),
            self.view_nav_item(
                "Organizations",
                RuleTab::OrgRules,
                icons::icon_building(icon_size, self.nav_icon_color(RuleTab::OrgRules), t)
            ),
            self.view_nav_item(
                "Types",
                RuleTab::TypeRules,
                icons::icon_tag(icon_size, self.nav_icon_color(RuleTab::TypeRules), t)
            ),
        ]
        .spacing(4)
        .padding([16, 8]);

        container(nav_items)
            .width(Length::Fixed(self.sidebar_width))
            .height(Fill)
            .style(move |_| container::Style {
                background: Some(iced::Background::Color(p.bg_sidebar)),
                border: iced::Border {
                    color: p.border_subtle,
                    width: 1.0,
                    radius: 0.0.into(),
                },
                ..Default::default()
            })
            .into()
    }

    fn nav_icon_color(&self, tab: RuleTab) -> iced::Color {
        let p = theme::palette();
        if self.selected_tab == tab {
            p.accent
        } else {
            p.text_muted
        }
    }

    fn view_nav_item(
        &self,
        label: &'static str,
        tab: RuleTab,
        icon: Element<'static, RuleEngineMessage>,
    ) -> Element<'static, RuleEngineMessage> {
        let p = theme::palette();
        let is_selected = self.selected_tab == tab;
        let scale = self.sidebar_font_scale;
        let text_size = 13.0 * scale;

        let text_color = if is_selected {
            p.accent
        } else {
            p.text_primary
        };

        let content = row![
            icon,
            Space::new().width(8.0 * scale), // Scale spacing too? Maybe not explicitly requested but looks better.
            text(label).size(text_size).color(text_color),
        ]
        .align_y(Alignment::Center)
        .padding([8, 10]);

        button(content)
            .style(move |theme, status| (theme::sidebar_button(is_selected))(theme, status))
            .on_press(RuleEngineMessage::SelectTab(tab))
            .width(Fill)
            .into()
    }

    // ========================================================================
    // Tab Content
    // ========================================================================

    fn view_tab_content(&self) -> Element<'_, RuleEngineMessage> {
        let p = theme::palette();
        let t = self.icon_theme;

        let content: Element<'_, RuleEngineMessage> = match self.selected_tab {
            RuleTab::Overview => tabs::view_overview_tab(&self.rules),
            RuleTab::TimeRules => tabs::view_time_rules_tab(&self.rules, t),
            RuleTab::ScheduleRules => tabs::view_schedule_rules_tab(&self.rules, t),
            RuleTab::AccountRules => tabs::view_account_rules_tab(&self.rules, t),
            RuleTab::OrgRules => tabs::view_org_rules_tab(&self.rules, t),
            RuleTab::TypeRules => tabs::view_type_rules_tab(
                &self.rules,
                t,
                self.new_type_rule_type,
                &self.new_type_rule_account,
                &self.accounts,
                self.new_type_rule_priority,
                self.new_type_rule_action,
            ),
        };

        let scrollable_content = scrollable(content)
            .width(Fill)
            .height(Fill)
            .style(theme::scrollbar);

        container(scrollable_content)
            .width(Fill)
            .height(Fill)
            .style(move |_| container::Style {
                background: Some(iced::Background::Color(p.bg_base)),
                ..Default::default()
            })
            .into()
    }
}
