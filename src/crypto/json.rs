use crate::crypto;
use anyhow::Result;
use serde::de::DeserializeOwned;
use std::{
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    ops::{Deref, DerefMut},
};
use x25519_dalek::SharedSecret;

/// A JSON payload that's encrypted by the client.
pub struct EncryptedJson<T>(T);

impl<T> EncryptedJson<T> {
    /// Construct a new object.
    pub fn new(obj: T) -> Self {
        Self(obj)
    }
}

impl<T> EncryptedJson<T>
where
    T: DeserializeOwned,
{
    /// Deconstruct to an inner value.
    pub fn into_inner(self) -> T {
        self.0
    }

    /// Construct the object from a string value.
    pub fn from_str(raw: &str) -> Result<Self> {
        Ok(Self(serde_json::from_str(raw)?))
    }
}

impl<T> Deref for EncryptedJson<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> DerefMut for EncryptedJson<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> Debug for EncryptedJson<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Encrypted JSON \"{:?}\"", self.0)
    }
}

impl<T> Display for EncryptedJson<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}
