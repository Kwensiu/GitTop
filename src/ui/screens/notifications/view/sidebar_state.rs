//! Sidebar state structure for view rendering.

use crate::github::{SubjectType, UserInfo};
use crate::settings::IconTheme;

pub struct SidebarState<'a> {
    pub user: &'a UserInfo,
    /// List of accounts.
    /// Kept as owned Vec because it is constructed ephemerally from an iterator in app.rs,
    /// and storing it as a slice would require a persistent storage in the parent scope which doesn't exist.
    pub accounts: Vec<String>,
    pub type_counts: &'a [(SubjectType, usize)],
    pub repo_counts: &'a [(String, usize)],
    pub selected_type: Option<SubjectType>,
    pub selected_repo: Option<&'a str>,
    pub total_count: usize,
    /// Total count for repositories section (for "All" item)
    pub total_repo_count: usize,
    pub icon_theme: IconTheme,
    pub width: f32,
    pub power_mode: bool,
}
