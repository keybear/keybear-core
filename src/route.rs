/// Define the routes by API version.
pub mod v1 {
    /// Nonce required for sending an encrypted request, unencrypted.
    pub const NONCE: &str = "v1/nonce";
    /// Registration of new devices, unencrypted.
    pub const REGISTER: &str = "/v1/register";
    /// Verification of newly registered devices.
    pub const VERIFY: &str = "/v1/verify";
    /// All the devices that need to be verified.
    pub const VERIFICATION_DEVICES: &str = "/v1/verification_devices";
    /// All the devices which have gone through the full verification process.
    pub const DEVICES: &str = "/v1/devices";
    /// All passwords.
    pub const PASSWORD: &str = "/v1/passwords";
}
