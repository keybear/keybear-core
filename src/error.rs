use quick_error::quick_error;

quick_error! {
    /// Error while setting up a client.
    #[derive(Debug)]
    pub enum SetupError {
        /// Tor .onion url is not valid.
        InvalidServerUrl(descr: &'static str) {
            display("Server .onion URL is not valid: {}", descr)
        }
    }
}
