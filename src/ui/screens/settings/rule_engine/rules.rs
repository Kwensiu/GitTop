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

    /// User-facing display label for the action (used in rule cards).
    pub fn display_label(&self) -> &'static str {
        match self {
            Self::Show => "Show",
            Self::Silent => "Silent",
            Self::Hide => "Suppress",
            Self::Priority => "Priority",
        }
    }
}

impl std::fmt::Display for RuleAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Show => write!(f, "Show"),
            Self::Silent => write!(f, "Silent"),
            Self::Hide => write!(f, "Suppress"),
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

#[allow(dead_code)]
pub const PRIORITY_LEVELS: &[(&str, i32)] = &[
    ("Max (100)", PRIORITY_MAX),
    ("High (50)", PRIORITY_HIGH),
    ("Default (0)", PRIORITY_DEFAULT),
    ("Low (-50)", PRIORITY_LOW),
    ("Min (-100)", PRIORITY_MIN),
];

fn default_priority() -> i32 {
    PRIORITY_DEFAULT
}

// ============================================================================
// OVERVIEW HELPERS
// ============================================================================

/// High-impact rule info for Overview display.
#[derive(Debug, Clone)]
pub struct HighImpactRule {
    pub name: String,
    pub rule_type: String,
    pub action: RuleAction,
    pub priority: i32,
}

/// Helper to check if current time string "HH:MM" is in range [start, end].
/// Handles wrap-around (e.g. 22:00 -> 07:00).
fn is_time_in_range(current: &str, start: &str, end: &str) -> bool {
    if start <= end {
        current >= start && current <= end
    } else {
        // crossing midnight
        current >= start || current <= end
    }
}

// ============================================================================
// RULE TYPES
// ============================================================================

/// Time-based quiet hours rule.
/// TODO: v0.2+ Refactor time parsing to use a dedicated TimeHM struct to handle validation and locale issues.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRule {
    pub id: String,
    pub name: String,
    pub enabled: bool,
    /// Start time in HH:MM format (24h).
    pub start_time: String,
    /// End time in HH:MM format (24h).
    pub end_time: String,
    #[serde(default = "default_priority")]
    pub priority: i32,
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
            priority: PRIORITY_DEFAULT,
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
    #[serde(default = "default_priority")]
    pub priority: i32,
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
            priority: PRIORITY_DEFAULT,
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
    #[allow(dead_code)]
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

    // ========================================================================
    // Overview Stats Helpers
    // ========================================================================

    /// Count rules with Hide (suppress) action.
    pub fn count_suppress_rules(&self) -> usize {
        if !self.enabled {
            return 0;
        }
        let mut count = 0;
        count += self
            .time_rules
            .iter()
            .filter(|r| r.enabled && r.action == RuleAction::Hide)
            .count();
        count += self
            .schedule_rules
            .iter()
            .filter(|r| r.enabled && r.action == RuleAction::Hide)
            .count();
        count += self
            .account_rules
            .iter()
            .filter(|r| r.enabled && r.action == RuleAction::Hide)
            .count();
        count += self
            .org_rules
            .iter()
            .filter(|r| r.enabled && r.action == RuleAction::Hide)
            .count();
        count += self
            .type_rules
            .iter()
            .filter(|r| r.enabled && r.action == RuleAction::Hide)
            .count();
        count
    }

    /// Count rules with priority >= 50.
    pub fn count_high_priority_rules(&self) -> usize {
        if !self.enabled {
            return 0;
        }
        let mut count = 0;
        count += self
            .time_rules
            .iter()
            .filter(|r| r.enabled && r.priority >= PRIORITY_HIGH)
            .count();
        count += self
            .schedule_rules
            .iter()
            .filter(|r| r.enabled && r.priority >= PRIORITY_HIGH)
            .count();
        count += self
            .org_rules
            .iter()
            .filter(|r| r.enabled && r.priority >= PRIORITY_HIGH)
            .count();
        count += self
            .type_rules
            .iter()
            .filter(|r| r.enabled && r.priority >= PRIORITY_HIGH)
            .count();
        count
    }

    /// Count time/schedule rules that are currently active based on current time.
    pub fn count_active_time_based_rules(&self, now: &chrono::DateTime<chrono::Local>) -> usize {
        use chrono::Datelike;

        if !self.enabled {
            return 0;
        }

        let time_str = now.format("%H:%M").to_string();
        let weekday = now.weekday().num_days_from_sunday() as u8;

        let mut count = 0;

        // Check time rules
        for rule in &self.time_rules {
            if rule.enabled && is_time_in_range(&time_str, &rule.start_time, &rule.end_time) {
                count += 1;
            }
        }

        // Check schedule rules
        for rule in &self.schedule_rules {
            if rule.enabled && rule.days.contains(&weekday) {
                let in_time = if let (Some(start), Some(end)) = (&rule.start_time, &rule.end_time) {
                    is_time_in_range(&time_str, start, end)
                } else {
                    true
                };
                if in_time {
                    count += 1;
                }
            }
        }

        count
    }

    /// Collected high-impact rule info for Overview display.
    pub fn get_high_impact_rules(&self) -> Vec<HighImpactRule> {
        if !self.enabled {
            return Vec::new();
        }

        let mut rules = Vec::new();

        // Time rules with Hide or high priority
        for rule in &self.time_rules {
            if rule.enabled && (rule.action == RuleAction::Hide || rule.priority >= PRIORITY_HIGH) {
                rules.push(HighImpactRule {
                    name: rule.name.clone(),
                    rule_type: "Time".to_string(),
                    action: rule.action,
                    priority: rule.priority,
                });
            }
        }

        // Schedule rules with Hide or high priority
        for rule in &self.schedule_rules {
            if rule.enabled && (rule.action == RuleAction::Hide || rule.priority >= PRIORITY_HIGH) {
                rules.push(HighImpactRule {
                    name: rule.name.clone(),
                    rule_type: "Schedule".to_string(),
                    action: rule.action,
                    priority: rule.priority,
                });
            }
        }

        // Account rules with Hide action (accounts don't have priority)
        for rule in &self.account_rules {
            if rule.enabled && rule.action == RuleAction::Hide {
                rules.push(HighImpactRule {
                    name: rule.account.clone(),
                    rule_type: "Account".to_string(),
                    action: rule.action,
                    priority: 0,
                });
            }
        }

        // Org rules with Hide or high priority
        for rule in &self.org_rules {
            if rule.enabled && (rule.action == RuleAction::Hide || rule.priority >= PRIORITY_HIGH) {
                rules.push(HighImpactRule {
                    name: rule.org.clone(),
                    rule_type: "Org".to_string(),
                    action: rule.action,
                    priority: rule.priority,
                });
            }
        }

        // Type rules with Hide or high priority
        for rule in &self.type_rules {
            if rule.enabled && (rule.action == RuleAction::Hide || rule.priority >= PRIORITY_HIGH) {
                let name = if let Some(acc) = &rule.account {
                    format!("{} ({})", rule.notification_type, acc)
                } else {
                    format!("{} (Global)", rule.notification_type)
                };
                rules.push(HighImpactRule {
                    name,
                    rule_type: "Type".to_string(),
                    action: rule.action,
                    priority: rule.priority,
                });
            }
        }

        rules
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

    /// Evaluate a notification and return the action to take.
    ///
    /// Priority order:
    /// 1. Priority rules always win (highest priority value wins among them)
    /// 2. Hide rules evaluated next
    /// 3. Silent rules
    /// 4. Default to Show
    #[allow(dead_code)]
    pub fn evaluate(
        &self,
        notification_type: &str,
        repo_owner: Option<&str>,
        account: Option<&str>,
        now: &chrono::DateTime<chrono::Local>,
    ) -> RuleAction {
        self.evaluate_detailed(notification_type, repo_owner, account, now)
            .0
    }

    /// Evaluate with full trace of the decision.
    pub fn evaluate_detailed(
        &self,
        notification_type: &str,
        repo_owner: Option<&str>,
        account: Option<&str>,
        now: &chrono::DateTime<chrono::Local>,
    ) -> (RuleAction, Option<RuleDecision>) {
        if !self.rules.enabled {
            return (RuleAction::Show, None);
        }

        let mut matching_actions: Vec<(i32, RuleAction, String, RuleDecisionReason)> = Vec::new();

        // 1. Time Rules
        let time_str = now.format("%H:%M").to_string();
        for rule in &self.rules.time_rules {
            if rule.enabled && self.is_time_in_range(&time_str, &rule.start_time, &rule.end_time) {
                matching_actions.push((
                    rule.priority,
                    rule.action,
                    rule.id.clone(),
                    RuleDecisionReason::TimeRule(rule.name.clone()),
                ));
            }
        }

        // 2. Schedule Rules
        let weekday = now.weekday().num_days_from_sunday() as u8; // 0=Sun, 6=Sat
        for rule in &self.rules.schedule_rules {
            if rule.enabled && rule.days.contains(&weekday) {
                let in_time = if let (Some(start), Some(end)) = (&rule.start_time, &rule.end_time) {
                    self.is_time_in_range(&time_str, start, end)
                } else {
                    true
                };

                if in_time {
                    matching_actions.push((
                        rule.priority,
                        rule.action,
                        rule.id.clone(),
                        RuleDecisionReason::ScheduleRule(rule.name.clone()),
                    ));
                }
            }
        }

        // 3. Account Rules
        if let Some(acc) = account {
            for rule in &self.rules.account_rules {
                if rule.enabled && rule.account.eq_ignore_ascii_case(acc) {
                    matching_actions.push((
                        PRIORITY_DEFAULT,
                        rule.action,
                        rule.id.clone(),
                        RuleDecisionReason::AccountRule(rule.account.clone()),
                    ));
                }
            }
        }

        // 4. Org Rules
        if let Some(owner) = repo_owner {
            for rule in &self.rules.org_rules {
                if rule.enabled && rule.org.eq_ignore_ascii_case(owner) {
                    matching_actions.push((
                        rule.priority,
                        rule.action,
                        rule.id.clone(),
                        RuleDecisionReason::OrgRule(rule.org.clone()),
                    ));
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
                let account_match = match (&rule.account, account) {
                    (None, _) => true,
                    (Some(rule_acc), Some(notif_acc)) => rule_acc.eq_ignore_ascii_case(notif_acc),
                    (Some(_), None) => false,
                };

                if account_match {
                    matching_actions.push((
                        rule.priority,
                        rule.action,
                        rule.id.clone(),
                        RuleDecisionReason::TypeRule(rule.notification_type.clone()),
                    ));
                }
            }
        }

        if matching_actions.is_empty() {
            return (RuleAction::Show, None);
        }

        // Sort by priority descending
        matching_actions.sort_by(|a, b| b.0.cmp(&a.0));

        // Get max priority
        let max_priority = matching_actions[0].0;

        // Iterate through matching actions to find the winner
        // We only care about rules with the max priority level
        // Conflict Resolution: Priority > Hide > Silent > Show

        // Optimization: Find the best action within the max priority band without cloning
        let mut best_action = RuleAction::Show;
        let mut best_decision: Option<RuleDecision> = None;

        for (p, action, id, reason) in matching_actions.iter() {
            if *p != max_priority {
                break; // Because sorted descending, we can stop once priority drops
            }

            let is_better = match (best_action, action) {
                (RuleAction::Priority, _) => false, // Only another Priority could match, which is same level
                (_, RuleAction::Priority) => true,  // Priority always beats non-Priority
                (RuleAction::Hide, _) => false,     // Hide beats Silent/Show
                (_, RuleAction::Hide) => true,      // Hide beats Silent/Show
                (RuleAction::Silent, _) => false,   // Silent beats Show
                (_, RuleAction::Silent) => true,    // Silent beats Show
                (RuleAction::Show, RuleAction::Show) => false, // Same level
            };

            // First valid rule sets the baseline.
            // If best_decision is None, we take the first one.
            // If we found a strictly "better" action, we take it.
            if best_decision.is_none() || is_better {
                best_action = *action;
                best_decision = Some(RuleDecision {
                    applied_rule_id: id.clone(),
                    action: *action,
                    priority: *p,
                    reason: reason.clone(),
                });
            }
        }

        (best_action, best_decision)
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

/// Trace of why a specific rule was applied.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuleDecision {
    pub applied_rule_id: String,
    pub action: RuleAction,
    pub priority: i32,
    pub reason: RuleDecisionReason,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RuleDecisionReason {
    TimeRule(String),
    ScheduleRule(String),
    AccountRule(String),
    OrgRule(String),
    TypeRule(String),
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

    #[test]
    fn test_priority_resolution() {
        let mut rules = NotificationRuleSet::default();
        rules.enabled = true;

        // 1. High Priority Org Rule (Priority 50) -> Show
        let org_rule = OrgRule {
            id: "org1".to_string(),
            enabled: true,
            org: "WorkOrg".to_string(),
            priority: 50,
            action: RuleAction::Show,
        };
        rules.org_rules.push(org_rule);

        // 2. Default Priority Time Rule (Priority 0) -> Silent
        // This simulates "Night Mode" which usually defaults to 0 priority
        let time_rule = TimeRule {
            id: "time1".to_string(),
            name: "Night".to_string(),
            enabled: true,
            start_time: "00:00".to_string(),
            end_time: "23:59".to_string(), // Always active
            priority: PRIORITY_DEFAULT,
            action: RuleAction::Silent,
        };
        rules.time_rules.push(time_rule);

        let engine = RuleEngine::new(rules);
        let now = chrono::Local::now();

        // Evaluate for WorkOrg
        // Org Rule (50) > Time Rule (0). Action should be Show (from Org Rule).
        let (action, decision) = engine.evaluate_detailed("mention", Some("WorkOrg"), None, &now);

        assert_eq!(action, RuleAction::Show);
        let decision = decision.expect("Should have a decision");

        // Check for context in reason
        match decision.reason {
            RuleDecisionReason::OrgRule(ref r) => assert_eq!(r, "WorkOrg"),
            _ => panic!("Expected OrgRule reason"),
        }
        assert_eq!(decision.priority, 50);
    }

    #[test]
    fn test_action_conflict_resolution() {
        // Test: Same priority, different actions. Priority > Hide > Silent > Show
        let mut rules = NotificationRuleSet::default();
        rules.enabled = true;

        // 1. Type Rule: Hide (Priority 0)
        let type_rule = TypeRule {
            id: "type1".to_string(),
            enabled: true,
            notification_type: "ci_activity".to_string(),
            account: None,
            priority: 0,
            action: RuleAction::Hide,
        };
        rules.type_rules.push(type_rule);

        // 2. Time Rule: Silent (Priority 0)
        let time_rule = TimeRule {
            id: "time1".to_string(),
            name: "Always".to_string(),
            enabled: true,
            start_time: "00:00".to_string(),
            end_time: "23:59".to_string(),
            priority: 0,
            action: RuleAction::Silent,
        };
        rules.time_rules.push(time_rule);

        let engine = RuleEngine::new(rules);
        let now = chrono::Local::now();

        // Both apply with Priority 0.
        // Hide > Silent. Should be Hide.
        let (action, decision) = engine.evaluate_detailed("ci_activity", None, None, &now);

        assert_eq!(action, RuleAction::Hide);
        let decision = decision.expect("Should have a decision");

        match decision.reason {
            RuleDecisionReason::TypeRule(ref t) => assert_eq!(t, "ci_activity"),
            _ => panic!("Expected TypeRule reason"),
        }
    }

    #[test]
    fn test_priority_action_does_not_override_higher_numeric_priority() {
        // Regression test: A rule with RuleAction::Priority should NOT override a rule with higher numeric priority.
        let mut rules = NotificationRuleSet::default();
        rules.enabled = true;

        // 1. Low Priority Rule with "Priority" action (Priority -10)
        // Using TypeRule for this
        let low_prio_rule = TypeRule {
            id: "low_prio".to_string(),
            enabled: true,
            notification_type: "review_requested".to_string(),
            account: None,
            priority: -10,
            action: RuleAction::Priority,
        };
        rules.type_rules.push(low_prio_rule);

        // 2. High Priority Rule with "Silent" action (Priority 50)
        // Using OrgRule
        let high_prio_rule = OrgRule {
            id: "high_prio".to_string(),
            enabled: true,
            org: "WorkOrg".to_string(),
            priority: 50,
            action: RuleAction::Silent,
        };
        rules.org_rules.push(high_prio_rule);

        let engine = RuleEngine::new(rules);
        let now = chrono::Local::now();

        // Evaluate
        // High Prio (50) > Low Prio (-10), even though Low Prio has Action::Priority.
        // The numeric priority band wins first.
        let (action, decision) =
            engine.evaluate_detailed("review_requested", Some("WorkOrg"), None, &now);

        assert_eq!(action, RuleAction::Silent);
        let decision = decision.expect("Should have a decision");

        // Assert that the high priority rule won
        assert_eq!(decision.priority, 50);
        match decision.reason {
            RuleDecisionReason::OrgRule(ref o) => assert_eq!(o, "WorkOrg"),
            _ => panic!("Expected OrgRule to win due to higher numeric priority"),
        }
    }
}
