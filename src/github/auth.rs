//! Authentication module for secure credential storage and validation.

use keyring::Entry;
use thiserror::Error;

use super::client::{GitHubClient, GitHubError};
use super::types::UserInfo;

/// Service name for keyring storage.
const SERVICE_NAME: &str = "gittop";
const ACCOUNT_NAME: &str = "github_pat";

/// Authentication-specific errors.
#[derive(Debug, Error, Clone)]
pub enum AuthError {
    #[error("Keyring error: {0}")]
    Keyring(String),

    #[error("GitHub API error: {0}")]
    GitHub(#[from] GitHubError),
}

/// Authentication manager handling token storage and validation.
pub struct AuthManager;

impl AuthManager {
    /// Creates a new keyring entry.
    fn get_entry() -> Result<Entry, AuthError> {
        Entry::new(SERVICE_NAME, ACCOUNT_NAME).map_err(|e| AuthError::Keyring(e.to_string()))
    }

    /// Saves the token to secure storage.
    pub fn save_token(token: &str) -> Result<(), AuthError> {
        let entry = Self::get_entry()?;
        entry
            .set_password(token)
            .map_err(|e| AuthError::Keyring(e.to_string()))?;
        Ok(())
    }

    /// Deletes the stored token.
    pub fn delete_token() -> Result<(), AuthError> {
        let entry = Self::get_entry()?;
        match entry.delete_credential() {
            Ok(()) => Ok(()),
            Err(keyring::Error::NoEntry) => Ok(()), // Already deleted
            Err(e) => Err(AuthError::Keyring(e.to_string())),
        }
    }

    /// Full authentication flow: validate token, save to keyring, return user info.
    pub async fn authenticate(token: &str) -> Result<(GitHubClient, UserInfo), AuthError> {
        // Validate and create client
        let (client, user) = GitHubClient::validate_token(token).await?;

        // Save to secure storage
        Self::save_token(token)?;

        Ok((client, user))
    }
}
