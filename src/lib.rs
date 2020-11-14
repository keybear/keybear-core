#![forbid(unsafe_code)]

pub mod crypto;
pub mod error;

use error::SetupError;

/// State of a keybear client.
#[derive(Debug)]
pub struct Client {
    /// Tor .onion address of the server.
    server_url: String,
}

impl Client {
    /// Setup a new keybear client.
    ///
    /// Tor .onion address of the server.
    pub fn new<S>(server_url: S) -> Result<Self, SetupError>
    where
        S: Into<String>,
    {
        // Get the generic as the actual value so it's traits can be used
        let server_url = server_url.into();

        // Validate the URL
        if !server_url.contains(".onion") {
            return Err(SetupError::InvalidServerUrl("does not contain .onion"));
        }

        Ok(Self { server_url })
    }
}

#[cfg(test)]
mod tests {
    use crate::Client;

    #[test]
    fn validate_url() {
        // Valid URLs
        assert!(Client::new("abcdefgh.onion:1234").is_ok());

        // Invalid URLs
        assert!(Client::new("test.com").is_err());
    }
}
