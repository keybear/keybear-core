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
    pub fn new(name: &str, public_key: &PublicKey) -> Self {
        Self {
            name: name.to_string(),
            public_key: base64::encode(public_key.as_bytes()),
        }
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
    /// Get the public key of the server.
    pub fn server_public_key(&self) -> Result<PublicKey> {
        // Read exactly the bytes from the public key
        let bytes: [u8; 32] = base64::decode(self.server_public_key.clone())
            .context("Device public key is invalid")?
            .try_into()
            .map_err(|_| anyhow!("Device public key is invalid"))?;

        Ok(PublicKey::from(bytes))
    }

    /// Get the registered unique identifier.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Get the name.
    pub fn name(&self) -> &str {
        &self.name
    }
}
