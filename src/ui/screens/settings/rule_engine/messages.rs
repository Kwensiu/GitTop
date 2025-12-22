//! Rule Engine messages.

/// Rule Engine screen messages.
#[derive(Debug, Clone)]
pub enum RuleEngineMessage {
    /// Go back to settings.
    Back,
    /// Select a rule tab.
    SelectTab(RuleTab),
    /// Toggle the rule engine globally.
    ToggleEnabled(bool),

    // ========================================================================
    // Time Rules
    // ========================================================================
    ToggleTimeRule(String, bool),
    AddTimeRule,
    DeleteTimeRule(String),
    DuplicateTimeRule(String),

    // ========================================================================
    // Schedule Rules
    // ========================================================================
    ToggleScheduleRule(String, bool),
    AddWeekendRule,
    DeleteScheduleRule(String),
    DuplicateScheduleRule(String),

    // ========================================================================
    // Account Rules
    // ========================================================================
    ToggleAccountRule(String, bool),
    DeleteAccountRule(String),
    DuplicateAccountRule(String),

    // ========================================================================
    // Org Rules
    // ========================================================================
    ToggleOrgRule(String, bool),
    DeleteOrgRule(String),
    DuplicateOrgRule(String),

    // ========================================================================
    // Type Rules
    // ========================================================================
    ToggleTypeRule(String, bool),
    DeleteTypeRule(String),
    DuplicateTypeRule(String),

    // Type Rule Creation
    NewTypeRuleTypeChanged(crate::github::types::NotificationReason),
    NewTypeRuleAccountChanged(String),
    NewTypeRulePriorityChanged(i32),
    NewTypeRuleActionChanged(crate::ui::screens::settings::rule_engine::rules::RuleAction),
    AddTypeRule,
}

/// Rule Engine tabs for navigation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RuleTab {
    #[default]
    Overview,
    TimeRules,
    ScheduleRules,
    AccountRules,
    OrgRules,
    TypeRules,
}
