use serde::{Deserialize, Serialize};

/// A password entry.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct PasswordRequest {
    /// Unique identifier.
    pub id: Option<String>,
    /// Name of the password as configured by the user.
    pub name: Option<String>,
    /// The e-mail associated.
    pub email: Option<String>,
    /// The website associated.
    pub website: Option<String>,
}

impl PasswordRequest {
    /// Create an request that must match the ID.
    pub fn from_id(id: &str) -> Self {
        Self {
            id: Some(id.to_string()),
            ..Default::default()
        }
    }

    /// Create an request that must match the name.
    pub fn from_name(name: &str) -> Self {
        Self {
            name: Some(name.to_string()),
            ..Default::default()
        }
    }

    /// Create an request that must match the email.
    pub fn from_email(email: &str) -> Self {
        Self {
            email: Some(email.to_string()),
            ..Default::default()
        }
    }

    /// Create an request that must match the website.
    pub fn from_website(website: &str) -> Self {
        Self {
            website: Some(website.to_string()),
            ..Default::default()
        }
    }
}

/// A password response.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct PasswordResponse {
    /// The password.
    password: String,
}

impl PasswordResponse {
    /// Get the password.
    pub fn password(&self) -> &str {
        &self.password
    }
}
