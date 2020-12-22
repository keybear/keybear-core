use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::convert::TryInto;
use x25519_dalek::PublicKey;

/// A device registration struct.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegisterDeviceRequest {
    /// Name of the device as configured by the user.
    name: String,
    /// The public key of the device encoded as base64.
    public_key: String,
}

impl RegisterDeviceRequest {
    /// Construct a new device.
    pub fn new<S>(name: S, public_key: &PublicKey) -> Self
    where
        S: Into<String>,
    {
        Self {
            name: name.into(),
            public_key: base64::encode(public_key.as_bytes()),
        }
    }

    /// Name of the device as configured by the user.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The public key of the device encoded as base64.
    pub fn public_key(&self) -> &str {
        &self.public_key
    }
}

/// The result from successfully registering a device.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegisterDeviceResponse {
    /// Unique identifier.
    id: String,
    /// Name of the device as configured by the user.
    name: String,
    /// The public key of the server.
    server_public_key: String,
}

impl RegisterDeviceResponse {
    /// Create a new response.
    ///
    /// Used by the server applications.
    #[doc(hidden)]
    pub fn new<S1, S2>(id: S1, name: S2, server_public_key: &PublicKey) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        Self {
            id: id.into(),
            name: name.into(),
            server_public_key: base64::encode(server_public_key.as_bytes()),
        }
    }

    /// Unique identifier.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Name of the device as configured by the user.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the public key of the server.
    pub fn server_public_key(&self) -> Result<PublicKey> {
        // Read exactly the bytes from the public key
        let bytes: [u8; 32] = base64::decode(self.server_public_key.clone())
            .context("Device public key is invalid")?
            .try_into()
            .map_err(|_| anyhow!("Device public key is invalid"))?;

        Ok(PublicKey::from(bytes))
    }
}
