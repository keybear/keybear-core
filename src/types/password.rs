use serde::{Deserialize, Serialize};

/// A password entry.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct PasswordRequest {
    /// Unique identifier.
    pub id: Option<String>,
    /// Name of the password as configured by the user.
    pub name: Option<String>,
    /// Associated e-mail.
    pub email: Option<String>,
    /// Associated website.
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

    /// Unique identifier.
    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    /// Name of the password as configured by the user.
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Associated e-mail.
    pub fn email(&self) -> Option<&str> {
        self.email.as_deref()
    }

    /// Associated website.
    pub fn website(&self) -> Option<&str> {
        self.website.as_deref()
    }
}

/// A password response.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct PasswordResponse {
    /// The password.
    password: String,
}

impl PasswordResponse {
    /// Create a new response.
    ///
    /// Used by the server applications.
    #[doc(hidden)]
    pub fn new<S>(password: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            password: password.into(),
        }
    }

    /// Password.
    pub fn password(&self) -> &str {
        &self.password
    }
}
