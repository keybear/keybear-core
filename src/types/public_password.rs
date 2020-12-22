use serde::{Deserialize, Serialize};

/// Password information without the actual password.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PublicPassword {
    /// Unique identifier.
    id: String,
    /// Name of the password.
    name: String,
    /// Associated e-mail.
    email: Option<String>,
    /// Associated website.
    website: Option<String>,
}

impl PublicPassword {
    /// Create a new response.
    ///
    /// Used by the server applications.
    #[doc(hidden)]
    pub fn new<S1, S2, S3, S4>(id: S1, name: S2, email: Option<S3>, website: Option<S4>) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<String>,
        S4: Into<String>,
    {
        Self {
            id: id.into(),
            name: name.into(),
            email: email.map(|s| s.into()),
            website: website.map(|s| s.into()),
        }
    }

    /// Unique identifier.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Name of the password.
    pub fn name(&self) -> &str {
        &self.name
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
