//! Session Manager - Multi-account GitHub session handling.
//!
//! Manages multiple authenticated GitHub clients simultaneously.

use std::collections::HashMap;

use super::client::{GitHubClient, GitHubError};
use super::keyring::{AccountKeyring, KeyringError};
use super::types::UserInfo;
use thiserror::Error;

/// Session-related errors.
#[derive(Debug, Error, Clone)]
pub enum SessionError {
    #[error("Keyring error: {0}")]
    Keyring(#[from] KeyringError),

    #[error("GitHub API error: {0}")]
    GitHub(#[from] GitHubError),

    #[error("Token validation failed: {0}")]
    ValidationFailed(String),

    #[error("Account not found: {0}")]
    AccountNotFound(String),
}

/// An authenticated session for a single account.
#[derive(Debug, Clone)]
pub struct Session {
    pub username: String,
    pub client: GitHubClient,
    pub user: UserInfo,
}

/// Manages multiple GitHub sessions.
#[derive(Debug, Clone, Default)]
pub struct SessionManager {
    sessions: HashMap<String, Session>,
    primary: Option<String>,
}

impl SessionManager {
    /// Creates a new empty session manager.
    pub fn new() -> Self {
        Self::default()
    }

    /// Validates a token and returns user info.
    async fn validate_token(token: &str) -> Result<UserInfo, SessionError> {
        // Basic format validation
        if !token.starts_with("ghp_") && !token.starts_with("github_pat_") {
            return Err(SessionError::ValidationFailed(
                "Token must start with 'ghp_' or 'github_pat_'".to_string(),
            ));
        }

        // Create a temporary client to validate
        let client = GitHubClient::new(token)?;

        // Fetch user info to validate the token
        let user = client.get_authenticated_user().await?;

        Ok(user)
    }

    /// Add a new account by validating and storing its token.
    /// Returns the username of the added account.
    pub async fn add_account(&mut self, token: &str) -> Result<String, SessionError> {
        // Validate the token first
        let user = Self::validate_token(token).await?;
        let username = user.login.clone();

        // Store in keyring
        AccountKeyring::save_token(&username, token)?;

        // Create client and session
        let client = GitHubClient::new(token)?;
        let session = Session {
            username: username.clone(),
            client,
            user,
        };

        // If this is the first account, make it primary
        if self.sessions.is_empty() {
            self.primary = Some(username.clone());
        }

        self.sessions.insert(username.clone(), session);

        Ok(username)
    }

    /// Restore a session for a known username (loads token from keyring).
    pub async fn restore_account(&mut self, username: &str) -> Result<(), SessionError> {
        let token = AccountKeyring::load_token(username)?
            .ok_or_else(|| SessionError::AccountNotFound(username.to_string()))?;

        // Validate the token
        let user = match Self::validate_token(&token).await {
            Ok(user) => user,
            Err(SessionError::GitHub(GitHubError::Unauthorized)) => {
                // Token expired, clean up
                let _ = AccountKeyring::delete_token(username);
                return Err(SessionError::AccountNotFound(username.to_string()));
            }
            Err(e) => return Err(e),
        };

        // Create client and session
        let client = GitHubClient::new(&token)?;
        let session = Session {
            username: username.to_string(),
            client,
            user,
        };

        // If this is the first account, make it primary
        if self.sessions.is_empty() {
            self.primary = Some(username.to_string());
        }

        self.sessions.insert(username.to_string(), session);

        Ok(())
    }

    /// Remove an account (also deletes from keyring).
    pub fn remove_account(&mut self, username: &str) -> Result<(), SessionError> {
        self.sessions.remove(username);
        AccountKeyring::delete_token(username)?;

        // If we removed the primary, pick a new one
        if self.primary.as_deref() == Some(username) {
            self.primary = self.sessions.keys().next().cloned();
        }

        Ok(())
    }

    /// Get the primary session.
    pub fn primary(&self) -> Option<&Session> {
        self.primary
            .as_ref()
            .and_then(|name| self.sessions.get(name))
    }

    /// Get mutable primary session.
    #[allow(dead_code)]
    pub fn primary_mut(&mut self) -> Option<&mut Session> {
        if let Some(name) = self.primary.clone() {
            self.sessions.get_mut(&name)
        } else {
            None
        }
    }

    /// Set which account is primary.
    #[allow(dead_code)]
    pub fn set_primary(&mut self, username: &str) {
        if self.sessions.contains_key(username) {
            self.primary = Some(username.to_string());
        }
    }

    /// Get all active session usernames.
    pub fn usernames(&self) -> impl Iterator<Item = &str> {
        self.sessions.keys().map(String::as_str)
    }

    /// Get a specific session by username.
    #[allow(dead_code)]
    pub fn get(&self, username: &str) -> Option<&Session> {
        self.sessions.get(username)
    }

    /// Check if any sessions exist.
    pub fn is_empty(&self) -> bool {
        self.sessions.is_empty()
    }

    /// Number of active sessions.
    #[allow(unused)]
    pub fn len(&self) -> usize {
        self.sessions.len()
    }
}
