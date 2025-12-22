//! Notification Rule Engine - complex filtering rules for Power Mode.
//!
//! Provides time-based, account-based, and type-based notification filtering
//! with priority organization support and schedule-based quiet hours.

use chrono::Datelike;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;

// ============================================================================
// RULE ACTIONS
// ============================================================================

/// Actions that rules can take on matching notifications.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum RuleAction {
    /// **Show**: Standard behavior. The notification appears in the list and triggers a system desktop notification.
    #[default]
    Show,
    /// **Silent**: The notification appears in the list but **does not** trigger a desktop notification or sound.
    Silent,
    /// **Hide**: The notification is completely hidden from the list.
    Hide,
    /// **Priority**: The notification is prominently highlighted and always triggers a desktop notification, overriding other rules.
    Priority,
}

impl RuleAction {
    pub const ALL: &'static [Self] = &[Self::Show, Self::Silent, Self::Hide, Self::Priority];
}

impl std::fmt::Display for RuleAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Show => write!(f, "Show"),
            Self::Silent => write!(f, "Silent"),
            Self::Hide => write!(f, "Hide"),
            Self::Priority => write!(f, "Priority"),
        }
    }
}

// ============================================================================
// PRIORITY LEVELS
// ============================================================================

pub const PRIORITY_MAX: i32 = 100;
pub const PRIORITY_HIGH: i32 = 50;
pub const PRIORITY_DEFAULT: i32 = 0;
pub const PRIORITY_LOW: i32 = -50;
pub const PRIORITY_MIN: i32 = -100;

pub const PRIORITY_LEVELS: &[(&str, i32)] = &[
    ("Max (100)", PRIORITY_MAX),
    ("High (50)", PRIORITY_HIGH),
    ("Default (0)", PRIORITY_DEFAULT),
    ("Low (-50)", PRIORITY_LOW),
    ("Min (-100)", PRIORITY_MIN),
];

// ============================================================================
// RULE TYPES
// ============================================================================

/// Time-based quiet hours rule.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRule {
    pub id: String,
    pub name: String,
    pub enabled: bool,
    /// Start time in HH:MM format (24h).
    pub start_time: String,
    /// End time in HH:MM format (24h).
    pub end_time: String,
    pub action: RuleAction,
}

impl TimeRule {
    pub fn new(name: impl Into<String>, start: impl Into<String>, end: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: name.into(),
            enabled: true,
            start_time: start.into(),
            end_time: end.into(),
            action: RuleAction::Silent,
        }
    }
}

/// Day-of-week schedule rule.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleRule {
    pub id: String,
    pub name: String,
    pub enabled: bool,
    /// Days this rule applies (0 = Sunday, 6 = Saturday).
    pub days: Vec<u8>,
    /// Optional time range within those days.
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub action: RuleAction,
}

impl ScheduleRule {
    pub fn weekend_silent() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: "Weekend Mode".into(),
            enabled: false,
            days: vec![0, 6], // Sunday, Saturday
            start_time: None,
            end_time: None,
            action: RuleAction::Silent,
        }
    }
}

/// Per-account notification preferences.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountRule {
    pub id: String,
    pub enabled: bool,
    /// GitHub username.
    pub account: String,
    pub action: RuleAction,
}

impl AccountRule {
    pub fn new(account: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            enabled: true,
            account: account.into(),
            action: RuleAction::Show,
        }
    }
}

/// Organization-level priority and filtering.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrgRule {
    pub id: String,
    pub enabled: bool,
    /// GitHub organization name.
    pub org: String,
    /// Priority level (higher = more important).
    pub priority: i32,
    pub action: RuleAction,
}

impl OrgRule {
    pub fn new(org: impl Into<String>, priority: i32) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            enabled: true,
            org: org.into(),
            priority,
            action: RuleAction::Show,
        }
    }
}

/// Notification type suppression rule.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeRule {
    pub id: String,
    pub enabled: bool,
    /// Notification reason type (mention, review_requested, etc.).
    pub notification_type: String,
    /// Optional GitHub username to scope this rule to.
    #[serde(default)]
    pub account: Option<String>,
    /// Priority level (higher = more important).
    #[serde(default)]
    pub priority: i32,
    pub action: RuleAction,
}

impl TypeRule {
    pub fn new(
        notification_type: impl Into<String>,
        account: Option<String>,
        priority: i32,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            enabled: true,
            notification_type: notification_type.into(),
            account,
            priority,
            action: RuleAction::Show,
        }
    }
}

// ============================================================================
// RULE SET (ROOT CONTAINER)
// ============================================================================

/// Complete notification rule configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NotificationRuleSet {
    /// Global enable/disable for all rules.
    pub enabled: bool,
    /// Time-based quiet hours.
    pub time_rules: Vec<TimeRule>,
    /// Day-of-week scheduling.
    pub schedule_rules: Vec<ScheduleRule>,
    /// Per-account filtering.
    pub account_rules: Vec<AccountRule>,
    /// Organization priority rules.
    pub org_rules: Vec<OrgRule>,
    /// Notification type filtering.
    pub type_rules: Vec<TypeRule>,
}

impl NotificationRuleSet {
    /// Get the rules file path.
    fn rules_path() -> Option<PathBuf> {
        dirs::config_dir().map(|p| p.join("gittop").join("rules.json"))
    }

    /// Load rules from disk, or return defaults.
    pub fn load() -> Self {
        Self::rules_path()
            .and_then(|path| fs::read_to_string(path).ok())
            .and_then(|content| serde_json::from_str(&content).ok())
            .unwrap_or_default()
    }

    /// Save rules to disk.
    pub fn save(&self) -> Result<(), std::io::Error> {
        if let Some(path) = Self::rules_path() {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }
            let content = serde_json::to_string_pretty(self)?;
            fs::write(path, content)?;
        }
        Ok(())
    }

    /// Count total active rules.
    pub fn active_rule_count(&self) -> usize {
        if !self.enabled {
            return 0;
        }
        self.time_rules.iter().filter(|r| r.enabled).count()
            + self.schedule_rules.iter().filter(|r| r.enabled).count()
            + self.account_rules.iter().filter(|r| r.enabled).count()
            + self.org_rules.iter().filter(|r| r.enabled).count()
            + self.type_rules.iter().filter(|r| r.enabled).count()
    }
}

// ============================================================================
// RULE ENGINE (EVALUATION)
// ============================================================================

/// Rule engine for evaluating notifications against the rule set.
pub struct RuleEngine {
    rules: NotificationRuleSet,
}

impl RuleEngine {
    pub fn new(rules: NotificationRuleSet) -> Self {
        Self { rules }
    }

    /// Check if the rule engine is enabled.
    pub fn is_enabled(&self) -> bool {
        self.rules.enabled
    }

    /// Evaluate a notification and return the action to take.
    ///
    /// Priority order:
    /// 1. Priority rules always win
    /// 2. Hide rules evaluated next
    /// 3. Silent rules
    /// 4. Default to Show
    /// Evaluate a notification and return the action to take.
    pub fn evaluate(
        &self,
        notification_type: &str,
        repo_owner: Option<&str>,
        account: Option<&str>,
        // We'll need current time for time/schedule rules
        now: &chrono::DateTime<chrono::Local>,
    ) -> RuleAction {
        if !self.rules.enabled {
            return RuleAction::Show;
        }

        let mut matching_actions: Vec<(i32, RuleAction)> = Vec::new();

        // 1. Time Rules
        let time_str = now.format("%H:%M").to_string();
        for rule in &self.rules.time_rules {
            if rule.enabled && self.is_time_in_range(&time_str, &rule.start_time, &rule.end_time) {
                // Time rules usually imply defaults, let's say they have generic "High" priority if strict,
                // but usually they are just "Silent" or "Hide".
                // We'll assign them a base priority of 0 unless otherwise specified (TimeRule needs priority field eventually?)
                // For now, let's treat them as PRIORITY_DEFAULT (0).
                matching_actions.push((PRIORITY_DEFAULT, rule.action));
            }
        }

        // 2. Schedule Rules
        let weekday = now.weekday().num_days_from_sunday() as u8; // 0=Sun, 6=Sat
        for rule in &self.rules.schedule_rules {
            if rule.enabled && rule.days.contains(&weekday) {
                // Check optional time range
                let in_time = if let (Some(start), Some(end)) = (&rule.start_time, &rule.end_time) {
                    self.is_time_in_range(&time_str, start, end)
                } else {
                    true
                };

                if in_time {
                    matching_actions.push((PRIORITY_DEFAULT, rule.action));
                }
            }
        }

        // 3. Account Rules
        if let Some(acc) = account {
            for rule in &self.rules.account_rules {
                if rule.enabled && rule.account.eq_ignore_ascii_case(acc) {
                    // Account rules are usually user preference, treating as Default priority
                    matching_actions.push((PRIORITY_DEFAULT, rule.action));
                }
            }
        }

        // 4. Org Rules
        if let Some(owner) = repo_owner {
            for rule in &self.rules.org_rules {
                if rule.enabled && rule.org.eq_ignore_ascii_case(owner) {
                    matching_actions.push((rule.priority, rule.action));
                }
            }
        }

        // 5. Type Rules
        for rule in &self.rules.type_rules {
            if rule.enabled
                && rule
                    .notification_type
                    .eq_ignore_ascii_case(notification_type)
            {
                // Check scoping
                let account_match = match (&rule.account, account) {
                    (None, _) => true, // Global rule
                    (Some(rule_acc), Some(notif_acc)) => rule_acc.eq_ignore_ascii_case(notif_acc),
                    (Some(_), None) => false,
                };

                if account_match {
                    matching_actions.push((rule.priority, rule.action));
                }
            }
        }

        // Logic Implementation:
        // 1. Sort by Priority (Desc)
        // 2. Resolve conflicts (Priority Action > Hide > Silent > Show)

        if matching_actions.is_empty() {
            return RuleAction::Show;
        }

        // sort by priority descending
        matching_actions.sort_by(|a, b| b.0.cmp(&a.0));

        // Get the highest priority value found
        let max_priority = matching_actions[0].0;

        // Filter for only those with the max priority
        let same_priority_rules: Vec<RuleAction> = matching_actions
            .iter()
            .filter(|(p, _)| *p == max_priority)
            .map(|(_, a)| *a)
            .collect();

        // Conflict Resolution:
        // Priority > Hide > Silent > Show
        if same_priority_rules.contains(&RuleAction::Priority) {
            return RuleAction::Priority;
        }
        if same_priority_rules.contains(&RuleAction::Hide) {
            return RuleAction::Hide;
        }
        if same_priority_rules.contains(&RuleAction::Silent) {
            return RuleAction::Silent;
        }

        RuleAction::Show
    }

    /// Helper to check if current time string "HH:MM" is in range [start, end].
    /// Handles wrap-around (e.g. 22:00 -> 07:00).
    fn is_time_in_range(&self, current: &str, start: &str, end: &str) -> bool {
        if start <= end {
            current >= start && current <= end
        } else {
            // crossing midnight
            current >= start || current <= end
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rule_set_default() {
        let rules = NotificationRuleSet::default();
        assert!(!rules.enabled);
        assert_eq!(rules.active_rule_count(), 0);
    }

    #[test]
    fn test_time_rule_creation() {
        let rule = TimeRule::new("Night Mode", "22:00", "07:00");
        assert!(rule.enabled);
        assert_eq!(rule.start_time, "22:00");
        assert_eq!(rule.end_time, "07:00");
    }
}
