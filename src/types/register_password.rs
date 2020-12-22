use serde::{Deserialize, Serialize};

/// A password registration struct.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegisterPasswordRequest {
    /// Name of the password as configured by the user.
    name: String,
    /// Actual password.
    password: String,
    /// Associated e-mail.
    email: Option<String>,
    /// Associated website.
    website: Option<String>,
}

impl RegisterPasswordRequest {
    /// Construct a new password.
    pub fn new<S1, S2, S3, S4>(
        name: S1,
        password: S2,
        email: Option<S3>,
        website: Option<S4>,
    ) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<String>,
        S4: Into<String>,
    {
        Self {
            name: name.into(),
            password: password.into(),
            email: email.map(|email| email.into()),
            website: website.map(|website| website.into()),
        }
    }

    /// Name of the password as configured by the user.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Actual password.
    pub fn password(&self) -> &str {
        &self.password
    }

    /// Associated e-mail.
    pub fn email(&self) -> Option<&str> {
        self.email.as_ref().map(|s| s.as_str())
    }

    /// Associated website.
    pub fn website(&self) -> Option<&str> {
        self.website.as_ref().map(|s| s.as_str())
    }
}
