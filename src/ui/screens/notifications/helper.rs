//! Notification screen helpers - grouping, filtering, URL conversion.
//!
//! Architecture Notes:
//! - `ProcessedNotification` and `NotificationGroup`: Data structures for processed data
//! - `FilterSettings`: UI filter state
//! - `group_processed_notifications`: Presentation logic (time-based grouping)
//! - `apply_filters`, `count_by_type`, `count_by_repo`: Pure data transformations
//!
//! For rule evaluation, use `NotificationEngine` from `engine.rs` instead of
//! the legacy `process_with_rules` function.

use crate::github::{NotificationView, SubjectType};
use crate::ui::screens::settings::rule_engine::RuleAction;
use chrono::Local;
use std::collections::HashMap;

/// A notification with its evaluated rule action.
#[derive(Debug, Clone)]
pub struct ProcessedNotification {
    pub notification: NotificationView,
    pub action: RuleAction,
}

impl ProcessedNotification {
    pub fn is_priority(&self) -> bool {
        self.action == RuleAction::Important
    }
}

/// Group of notifications by time period.
#[derive(Debug, Clone)]
pub struct NotificationGroup {
    pub title: String,
    pub notifications: Vec<ProcessedNotification>,
    pub is_expanded: bool,
    /// True if this is the priority group (always shown first, special styling).
    pub is_priority: bool,
}

/// Filter settings for notification list.
#[derive(Debug, Clone, Default)]
pub struct FilterSettings {
    /// Show all notifications including read ones.
    pub show_all: bool,
    /// Filter by specific subject type (None = show all).
    pub selected_type: Option<SubjectType>,
    /// Filter by specific repository (None = show all).
    pub selected_repo: Option<String>,
}

/// Group processed notifications by time period (Today, This Week, Older).
/// Important notifications are extracted into a separate group shown first (only if `show_priority_group` is true).
pub fn group_processed_notifications(
    processed: &[ProcessedNotification],
    show_priority_group: bool,
) -> Vec<NotificationGroup> {
    let now_date = Local::now().date_naive();
    let one_week_ago = now_date - chrono::Duration::days(7);

    // Single pass accumulation into buckets
    let (priority, today, this_week, older) = processed.iter().fold(
        (Vec::new(), Vec::new(), Vec::new(), Vec::new()),
        |(mut p, mut t, mut w, mut o), notif| {
            if show_priority_group && notif.action == RuleAction::Important {
                p.push(notif.clone());
            } else {
                let notif_date = notif
                    .notification
                    .updated_at
                    .with_timezone(&Local)
                    .date_naive();

                if notif_date >= now_date {
                    t.push(notif.clone());
                } else if notif_date >= one_week_ago {
                    w.push(notif.clone());
                } else {
                    o.push(notif.clone());
                }
            }
            (p, t, w, o)
        },
    );

    let mut groups = Vec::with_capacity(4);

    // Important group always first (if not empty and enabled)
    if show_priority_group && !priority.is_empty() {
        groups.push(NotificationGroup {
            title: "Important".to_string(),
            notifications: priority,
            is_expanded: true,
            is_priority: true,
        });
    }

    groups.push(NotificationGroup {
        title: "Today".to_string(),
        notifications: today,
        is_expanded: true,
        is_priority: false,
    });

    groups.push(NotificationGroup {
        title: "This Week".to_string(),
        notifications: this_week,
        is_expanded: true,
        is_priority: false,
    });

    groups.push(NotificationGroup {
        title: "Older".to_string(),
        notifications: older,
        is_expanded: false,
        is_priority: false,
    });

    groups
}

/// Apply filters to a list of notifications.
pub fn apply_filters(
    notifications: &[NotificationView],
    filters: &FilterSettings,
) -> Vec<NotificationView> {
    notifications
        .iter()
        .filter(|n| {
            let passes_read = filters.show_all || n.unread;
            let passes_type = filters
                .selected_type
                .as_ref()
                .is_none_or(|t| &n.subject_type == t);
            let passes_repo = filters
                .selected_repo
                .as_ref()
                .is_none_or(|r| &n.repo_full_name == r);
            passes_read && passes_type && passes_repo
        })
        .cloned()
        .collect()
}

const SUBJECT_TYPE_ORDER: &[SubjectType] = &[
    SubjectType::PullRequest,
    SubjectType::Issue,
    SubjectType::Commit,
    SubjectType::CheckSuite,
    SubjectType::Discussion,
    SubjectType::Release,
    SubjectType::RepositoryVulnerabilityAlert,
];

/// Count notifications by subject type.
pub fn count_by_type(notifications: &[NotificationView]) -> Vec<(SubjectType, usize)> {
    let counts = notifications.iter().fold(HashMap::new(), |mut acc, n| {
        *acc.entry(n.subject_type).or_insert(0) += 1;
        acc
    });

    SUBJECT_TYPE_ORDER
        .iter()
        .filter_map(|t| counts.get(t).map(|&c| (*t, c)))
        .collect()
}

/// Count notifications by repository.
pub fn count_by_repo(notifications: &[NotificationView]) -> Vec<(String, usize)> {
    let mut counts: HashMap<&str, usize> = HashMap::new();
    for n in notifications {
        *counts.entry(&n.repo_full_name).or_insert(0) += 1;
    }

    let mut result: Vec<_> = counts.into_iter().map(|(k, v)| (k.to_owned(), v)).collect();

    result.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    result
}

/// Convert GitHub API URL to web URL.
pub fn api_url_to_web_url(api_url: &str) -> String {
    api_url
        .replace("api.github.com/repos", "github.com")
        .replace("/pulls/", "/pull/")
}
