use serde::{Deserialize, Serialize};

/// Device information that's accessible to other devices.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PublicDevice {
    /// Unique identifier.
    id: String,
    /// Name of the device.
    name: String,
}

impl PublicDevice {
    /// Create a new response.
    ///
    /// Used by the server applications.
    #[doc(hidden)]
    pub fn new<S1, S2>(id: S1, name: S2) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        Self {
            id: id.into(),
            name: name.into(),
        }
    }

    /// Unique identifier.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Name of the device.
    pub fn name(&self) -> &str {
        &self.name
    }
}
