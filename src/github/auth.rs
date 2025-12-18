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

    #[error("Token validation failed: {0}")]
    ValidationFailed(String),
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

    /// Loads the token from secure storage.
    pub fn load_token() -> Result<Option<String>, AuthError> {
        let entry = Self::get_entry()?;
        match entry.get_password() {
            Ok(token) => Ok(Some(token)),
            Err(keyring::Error::NoEntry) => Ok(None),
            Err(e) => Err(AuthError::Keyring(e.to_string())),
        }
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

    /// Validates a token by making a test API call.
    /// Returns user info if valid.
    pub async fn validate_token(token: &str) -> Result<UserInfo, AuthError> {
        // Basic format validation
        if !token.starts_with("ghp_") && !token.starts_with("github_pat_") {
            return Err(AuthError::ValidationFailed(
                "Token must start with 'ghp_' or 'github_pat_'".to_string(),
            ));
        }

        // Create a temporary client to validate
        let client = GitHubClient::new(token)?;

        // Fetch user info to validate the token
        let user = client.get_authenticated_user().await?;

        Ok(user)
    }

    /// Full authentication flow: validate token, save to keyring, return user info.
    pub async fn authenticate(token: &str) -> Result<(GitHubClient, UserInfo), AuthError> {
        // Validate by fetching user info
        let user = Self::validate_token(token).await?;

        // Save to secure storage
        Self::save_token(token)?;

        // Create the client
        let client = GitHubClient::new(token)?;

        Ok((client, user))
    }

    /// Attempt to load saved credentials and create a client.
    /// Returns None if no token is stored or if the token is invalid.
    pub async fn try_restore() -> Result<Option<(GitHubClient, UserInfo)>, AuthError> {
        let token = match Self::load_token()? {
            Some(t) => t,
            None => return Ok(None),
        };

        // Validate the stored token
        match Self::validate_token(&token).await {
            Ok(user) => {
                let client = GitHubClient::new(&token)?;
                Ok(Some((client, user)))
            }
            Err(AuthError::GitHub(GitHubError::Unauthorized)) => {
                // Token expired, clean up
                let _ = Self::delete_token();
                Ok(None)
            }
            Err(e) => Err(e),
        }
    }
}
