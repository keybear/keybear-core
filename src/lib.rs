#![forbid(unsafe_code)]

/// Primitives for encrypting/decrypting message bodies.
pub mod crypto;
/// Route URL paths.
pub mod route;
/// Serialization types that can be sent between clients and hosts.
pub mod types;

/// The required HTTP header containing the client ID.
pub const CLIENT_ID_HEADER: &str = "keybear-client-id";
