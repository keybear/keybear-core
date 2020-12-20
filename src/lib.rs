#![forbid(unsafe_code)]

pub mod crypto;
pub mod data;

/// The required HTTP header containing the client ID.
pub const CLIENT_ID_HEADER: &str = "keybear-client-id";
