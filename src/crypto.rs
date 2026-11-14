use std::fmt::Debug;

use derive_more::{Deref, DerefMut, Display};
use garde::Validate;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Validate, JsonSchema, Deref, DerefMut, Display, Serialize, Deserialize)]
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
