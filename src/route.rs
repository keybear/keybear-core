/// Define the routes by API version.
pub mod v1 {
    /// Registration of new devices.
    pub const REGISTER: &str = "/register";
    /// Verification of newly registered devices.
    pub const VERIFY: &str = "/verify";
    /// All the devices that need to be verified.
    pub const VERIFICATION_DEVICES: &str = "/verification_devices";
    /// All the devices which have gone through the full verification process.
    pub const DEVICES: &str = "/devices";
    /// All passwords.
    pub const PASSWORD: &str = "/passwords";
}
