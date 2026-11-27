use argon2::Argon2;
use derive_more::{Deref, DerefMut, Display};
use garde::Validate;
use password_hash::{PasswordHasher, PasswordVerifier, phc::PasswordHash};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, path::Path};

#[derive(Clone, Validate, JsonSchema, Deref, DerefMut, Display, Serialize, Deserialize, Hash)]
#[repr(transparent)]
pub struct Secret(#[garde(length(min = 3, max = 255))] pub String);

impl Debug for Secret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        SecretRef::from(self).fmt(f)
    }
}

impl From<String> for Secret {
    fn from(value: String) -> Self {
        Self(value)
    }
}

#[derive(Clone, Validate, JsonSchema, Deref, DerefMut, Display, Serialize, Deserialize)]
#[repr(transparent)]
pub struct SecretRef<'a>(#[garde(length(min = 3, max = 255))] pub &'a str);

impl<'a> From<&'a str> for SecretRef<'a> {
    fn from(value: &'a str) -> Self {
        Self(value)
    }
}

impl<'a> From<&'a Secret> for SecretRef<'a> {
    fn from(value: &'a Secret) -> Self {
        SecretRef(value)
    }
}

impl<'a> Debug for SecretRef<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", "*".repeat(self.len()))
    }
}

impl Secret {
    pub fn hash(&self) -> Result<String, password_hash::Error> {
        let hashed = Argon2::default().hash_password(self.0.as_bytes())?.to_string();

        Ok(hashed)
    }

    pub fn verify_hash(&self, encoded_hash: &str) -> bool {
        let Ok(parsed_hash) = PasswordHash::new(encoded_hash) else {
            return false;
        };

        Argon2::default().verify_password(self.0.as_bytes(), &parsed_hash).is_ok()
    }
}

impl Secret {
    #[allow(dead_code)]
    pub fn store(&self, path: impl AsRef<Path>, allow_group: bool) -> anyhow::Result<()> {
        super::store(&self.0, path, allow_group)
    }

    #[allow(dead_code)]
    pub fn load(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        super::load(path).map(Self)
    }
}
