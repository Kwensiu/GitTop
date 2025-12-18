//! GitHub notification types matching the GitHub API v3 schema.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// GitHub user information returned after successful authentication.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub login: String,
    pub name: Option<String>,
    pub avatar_url: String,
    pub html_url: String,
}

/// A GitHub notification from the notifications API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub id: String,
    pub unread: bool,
    pub reason: NotificationReason,
    pub updated_at: DateTime<Utc>,
    pub last_read_at: Option<DateTime<Utc>>,
    pub subject: NotificationSubject,
    pub repository: Repository,
    pub url: String,
}

/// The subject of a notification (issue, PR, release, etc.).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationSubject {
    pub title: String,
    pub url: Option<String>,
    pub latest_comment_url: Option<String>,
    #[serde(rename = "type")]
    pub subject_type: SubjectType,
}

/// The type of notification subject.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Copy)]
pub enum SubjectType {
    Issue,
    PullRequest,
    Release,
    Discussion,
    CheckSuite,
    Commit,
    RepositoryVulnerabilityAlert,
    /// Catch-all for unknown types from GitHub.
    #[serde(other)]
    Unknown,
}

impl SubjectType {
    /// Returns a human-readable label for the subject type.
    #[allow(unused)]
    pub fn label(&self) -> &'static str {
        match self {
            Self::Issue => "Issue",
            Self::PullRequest => "Pull Request",
            Self::Release => "Release",
            Self::Discussion => "Discussion",
            Self::CheckSuite => "CI",
            Self::Commit => "Commit",
            Self::RepositoryVulnerabilityAlert => "Security",
            Self::Unknown => "Notification",
        }
    }

    /// Returns a symbol/icon for the subject type.
    pub fn icon(&self) -> &'static str {
        match self {
            Self::Issue => "●",                        // Circle for issues
            Self::PullRequest => "⇄",                  // Arrows for PRs
            Self::Release => "◆",                      // Diamond for releases
            Self::Discussion => "💬",                  // Speech bubble
            Self::CheckSuite => "✓",                   // Checkmark for CI
            Self::Commit => "◉",                       // Dot circle for commits
            Self::RepositoryVulnerabilityAlert => "⚠", // Warning
            Self::Unknown => "○",                      // Empty circle
        }
    }
}

/// Why the user received this notification.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Copy)]
#[serde(rename_all = "snake_case")]
pub enum NotificationReason {
    Assign,
    Author,
    Comment,
    Invitation,
    Manual,
    Mention,
    ReviewRequested,
    SecurityAlert,
    StateChange,
    Subscribed,
    TeamMention,
    CiActivity,
    /// Catch-all for unknown reasons.
    #[serde(other)]
    Unknown,
}

impl NotificationReason {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Assign => "assigned",
            Self::Author => "author",
            Self::Comment => "commented",
            Self::Invitation => "invited",
            Self::Manual => "subscribed",
            Self::Mention => "mentioned",
            Self::ReviewRequested => "review requested",
            Self::SecurityAlert => "security",
            Self::StateChange => "state changed",
            Self::Subscribed => "watching",
            Self::TeamMention => "team mentioned",
            Self::CiActivity => "CI activity",
            Self::Unknown => "notification",
        }
    }
}

/// Repository information from the notification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repository {
    pub id: u64,
    pub name: String,
    pub full_name: String,
    pub owner: Owner,
    pub html_url: String,
    pub private: bool,
}

/// Repository owner (user or organization).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Owner {
    pub login: String,
    pub avatar_url: String,
}

/// Frontend-friendly notification format for the UI.
#[derive(Debug, Clone)]
pub struct NotificationView {
    pub id: String,
    pub title: String,
    pub repo_name: String,
    pub repo_full_name: String,
    pub subject_type: SubjectType,
    pub reason: NotificationReason,
    pub unread: bool,
    pub updated_at: DateTime<Utc>,
    pub time_ago: String,
    pub url: Option<String>,
    pub avatar_url: String,
    pub is_private: bool,
}

impl From<Notification> for NotificationView {
    fn from(n: Notification) -> Self {
        Self {
            id: n.id,
            title: n.subject.title,
            repo_name: n.repository.name,
            repo_full_name: n.repository.full_name,
            subject_type: n.subject.subject_type,
            reason: n.reason,
            unread: n.unread,
            updated_at: n.updated_at,
            time_ago: format_time_ago(n.updated_at),
            url: n.subject.url,
            avatar_url: n.repository.owner.avatar_url,
            is_private: n.repository.private,
        }
    }
}

fn format_time_ago(dt: DateTime<Utc>) -> String {
    let now = Utc::now();
    let duration = now.signed_duration_since(dt);

    if duration.num_minutes() < 1 {
        "just now".to_string()
    } else if duration.num_minutes() < 60 {
        format!("{}m", duration.num_minutes())
    } else if duration.num_hours() < 24 {
        format!("{}h", duration.num_hours())
    } else if duration.num_days() < 7 {
        format!("{}d", duration.num_days())
    } else {
        dt.format("%b %d").to_string()
    }
}
