#![forbid(unsafe_code)]

pub mod crypto;
pub mod types;

/// The required HTTP header containing the client ID.
pub const CLIENT_ID_HEADER: &str = "keybear-client-id";
