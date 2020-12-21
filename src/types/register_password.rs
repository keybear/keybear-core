use serde::{Deserialize, Serialize};

/// A password registration struct.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegisterPasswordRequest {
    /// Name of the password as configured by the user.
    pub name: String,
    /// The actual password.
    pub password: String,
    /// The e-mail associated.
    pub email: Option<String>,
    /// The website associated.
    pub website: Option<String>,
}

impl RegisterPasswordRequest {
    /// Construct a new password.
    pub fn new(name: &str, password: &str, email: Option<&str>, website: Option<&str>) -> Self {
        Self {
            name: name.to_string(),
            password: password.to_string(),
            email: email.map(|email| email.to_string()),
            website: website.map(|website| website.to_string()),
        }
    }
}
