// Export the encryption primitives here so no extra libraries need to be included
pub use chacha20poly1305::Nonce;
pub use x25519_dalek::{PublicKey, SharedSecret, StaticSecret};

use anyhow::{anyhow, bail, Result};
use chacha20poly1305::{
    aead::{Aead, NewAead},
    ChaCha20Poly1305, Key,
};
use log::{debug, trace};
use rand_core::OsRng;
use serde::{de::DeserializeOwned, Serialize};
use std::{
    any,
    fs::{self, File},
    io::Read,
    path::Path,
};

/// Add functions to the crypto secret key to make it easier to use.
pub trait StaticSecretExt {
    /// Check whether there is a file containing the crypto keys.
    fn verify_file<P>(file: P) -> bool
    where
        P: AsRef<Path>;

    /// Generate a new secret key with the OS random number generator.
    fn new_with_os_rand() -> StaticSecret;

    /// Try to load the crypto keys from our file on the disk.
    fn from_file<P>(file: P) -> Result<StaticSecret>
    where
        P: AsRef<Path>;

    /// Save the crypto keys to the file on the disk.
    fn save<P>(&self, file: P) -> Result<()>
    where
        P: AsRef<Path>;

    /// Try to load the crypto key or generate a new one.
    fn from_file_or_generate<P>(file: P) -> Result<StaticSecret>
    where
        P: AsRef<Path>,
    {
        if Self::verify_file(&file) {
            // The file exists, open it
            Self::from_file(file)
        } else {
            // The file doesn't exist, generate a new one and save it
            let key = Self::new_with_os_rand();
            key.save(file)?;

            Ok(key)
        }
    }
}

impl StaticSecretExt for StaticSecret {
    fn verify_file<P>(file: P) -> bool
    where
        P: AsRef<Path>,
    {
        // Get the generic as the actual reference so it's traits can be used
        let file = file.as_ref();

        debug!("Verifying file \"{}\"", file.display());

        // TODO: add more checks
        file.is_file()
    }

    fn new_with_os_rand() -> StaticSecret {
        // Get the generic as the actual reference so it's traits can be used
        debug!("Generating new secret key");

        // Generate a secret key
        StaticSecret::new(OsRng)
    }

    fn from_file<P>(file: P) -> Result<StaticSecret>
    where
        P: AsRef<Path>,
    {
        // Get the generic as the actual reference so it's traits can be used
        let file = file.as_ref();

        debug!("Loading secret key from file \"{}\"", file.display());

        // Cannot load from disk if the file is not a valid one
        if !Self::verify_file(file) {
            bail!("Reading crypto keys from file {:?} failed", file);
        }

        // Read the file
        let mut f = File::open(file)
            .map_err(|err| anyhow!("Reading crypto keys from file {:?} failed: {}", file, err))?;

        // Read exactly the bytes from the file
        let mut bytes = [0; 32];
        f.read_exact(&mut bytes).map_err(|err| {
            anyhow!(
                "Crypto keys file {:?} has wrong size, it might be corrupt: {}",
                file,
                err
            )
        })?;

        // Try to construct the secret key from the bytes
        Ok(StaticSecret::from(bytes))
    }

    fn save<P>(&self, file: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        // Get the generic as the actual reference so it's traits can be used
        let file = file.as_ref();

        debug!("Saving secret key to file \"{}\"", file.display());

        // Try to write the keys as raw bytes to the disk
        fs::write(file, self.to_bytes())
            .map_err(|err| anyhow!("Could not write crypto keys to file {:?}: {}", file, err))
    }
}

/// Encrypt a serializable object into a chacha20poly1305 encoded JSON string.
pub fn encrypt<T>(shared_secret_key: &SharedSecret, nonce: &Nonce, obj: &T) -> Result<Vec<u8>>
where
    T: Serialize,
{
    trace!("Encrypting \"{}\" into bytes", any::type_name::<T>());

    // Serialize the object into a JSON byte array
    let json = serde_json::to_vec(obj)?;

    cipher(shared_secret_key)
        // Encrypt the message
        .encrypt(nonce, json.as_slice())
        .map_err(|err| anyhow!("Encrypting message: {}", err))
}

/// Decrypt a chacha20poly1305 encoded JSON string into an object.
pub fn decrypt<T>(shared_secret_key: &SharedSecret, nonce: &Nonce, cipher_bytes: &[u8]) -> Result<T>
where
    T: DeserializeOwned,
{
    trace!("Trying to decrypt bytes into \"{}\"", any::type_name::<T>());

    cipher(shared_secret_key)
        // Decrypt the message
        .decrypt(nonce, cipher_bytes)
        .map_err(|err| anyhow!("Decrypting message: {}", err))
        // Try to convert it to a JSON object
        .map(|bytes| {
            serde_json::from_slice(&bytes).map_err(|err| {
                trace!(
                    "JSON resulting in error \"{}\":\n{}",
                    err,
                    String::from_utf8_lossy(&bytes)
                );

                anyhow!("Decrypted JSON is invalid: {}", err)
            })
        })?
}

/// Create a cipher from the shared secret key of a client and the server.
fn cipher(shared_secret_key: &SharedSecret) -> ChaCha20Poly1305 {
    let key = Key::from_slice(shared_secret_key.as_bytes());

    ChaCha20Poly1305::new(key)
}

#[cfg(test)]
mod tests {
    use crate::crypto::{self, StaticSecretExt};
    use anyhow::Result;
    use chacha20poly1305::Nonce;
    use rand_core::OsRng;
    use serde::{Deserialize, Serialize};
    use x25519_dalek::{EphemeralSecret, PublicKey, StaticSecret};

    #[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
    struct TestObject {
        string: String,
        int: i64,
        vec: Vec<String>,
    }

    #[test]
    fn default() -> Result<()> {
        // Create a temporary directory for the test database
        let dir = tempfile::tempdir()?;
        // Create the temporary file to save the key in
        let file = dir.path().join("key");

        // Try to load the file, which will fail and generate a new file
        StaticSecret::from_file_or_generate(file)?;

        Ok(())
    }

    #[test]
    fn verify() {
        // A non-existing file means it's not a valid file for the keys
        assert_eq!(
            StaticSecret::verify_file("/definitily/should/not/exist"),
            false
        );
    }

    #[test]
    fn save_and_load() -> Result<()> {
        // Create a temporary directory for the test database
        let dir = tempfile::tempdir()?;
        // Create the temporary file to save the key in
        let file = dir.path().join("key");

        // Generate a new pair of keys.
        let secret = StaticSecret::new_with_os_rand();

        // Save the secret key
        secret.save(&file)?;

        // Load the saved secret key from disk
        let disk_secret = StaticSecret::from_file(file)?;

        // Check if they are the same
        assert_eq!(secret.to_bytes(), disk_secret.to_bytes());

        // Close the directory
        dir.close()?;

        Ok(())
    }

    #[test]
    fn encrypt_decrypt() -> Result<()> {
        // Generate a new shared key
        let alice_secret = EphemeralSecret::new(OsRng);
        let bob_secret = EphemeralSecret::new(OsRng);
        let bob_public = PublicKey::from(&bob_secret);
        let shared_secret = alice_secret.diffie_hellman(&bob_public);

        let obj = TestObject {
            string: "HI".to_string(),
            int: 1234,
            vec: vec!["Hi!".to_string(), "there".to_string()],
        };

        // Create a random nonce
        let nonce = Nonce::from_slice(b"abcdefghijkl");

        // Encrypt the object
        let cipher_bytes = crypto::encrypt(&shared_secret, &nonce, &obj)?;

        // Decrypt the encrypted string
        let decrypted: TestObject = crypto::decrypt(&shared_secret, &nonce, &cipher_bytes)?;

        assert_eq!(obj, decrypted);

        Ok(())
    }
}
