use serde::{Deserialize, Serialize};

/// Device information that's accessible to other devices.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PublicDevice {
    /// Unique identifier.
    pub id: String,
    /// Name of the device.
    pub name: String,
}
