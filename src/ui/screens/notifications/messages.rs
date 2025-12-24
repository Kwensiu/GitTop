use crate::github::{GitHubError, NotificationView, SubjectType};

#[derive(Debug, Clone)]
pub enum NotificationMessage {
    Refresh,
    RefreshComplete(Result<Vec<NotificationView>, GitHubError>),

    Filter(FilterMessage),
    Thread(ThreadMessage),
    Bulk(BulkMessage),
    View(ViewMessage),
    Navigation(NavigationMessage),
}

#[derive(Debug, Clone)]
pub enum FilterMessage {
    ToggleShowAll,
    SelectType(Option<SubjectType>),
    SelectRepo(Option<String>),
}

#[derive(Debug, Clone)]
pub enum ThreadMessage {
    Open(String),
    MarkAsRead(String),
    MarkAsReadComplete(String, Result<(), GitHubError>),
    MarkAsDone(String),
    MarkAsDoneComplete(String, Result<(), GitHubError>),
    MarkAllAsRead,
    MarkAllAsReadComplete(Result<(), GitHubError>),
}

#[derive(Debug, Clone)]
pub enum BulkMessage {
    ToggleMode,
    ToggleSelect(String),
    SelectAll,
    Clear,
    MarkAsRead,
    MarkAsDone,
    Complete,
}

#[derive(Debug, Clone)]
pub enum ViewMessage {
    ToggleGroup(usize),
    OnScroll(iced::widget::scrollable::Viewport),
    SelectNotification(String),
    SelectComplete(
        String,
        Result<crate::github::NotificationSubjectDetail, GitHubError>,
    ),
    OpenInBrowser,
}

#[derive(Debug, Clone)]
pub enum NavigationMessage {
    Logout,
    OpenSettings,
    OpenRuleEngine,
    SwitchAccount(String),
    TogglePowerMode,
}
