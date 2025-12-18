//! GitHub API module - authentication, client, and types.

pub mod auth;
pub mod client;
pub mod types;

pub use auth::AuthManager;
pub use client::{GitHubClient, GitHubError};
pub use types::*;
