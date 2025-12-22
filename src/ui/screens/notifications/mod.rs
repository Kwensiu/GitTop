//! Notifications screen module.

mod engine;
mod group;
pub mod helper;
mod screen;
pub mod sidebar;
pub mod sidebar_state;
mod states;

// Public API exports for external consumers
#[allow(unused_imports)]
pub use engine::{DesktopNotificationBatch, NotificationEngine};
pub use screen::{NotificationMessage, NotificationsScreen};
