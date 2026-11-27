use crate::crypto::{ENCRYPTION_KEY_PATH, load_bytes};
use anyhow::{anyhow, ensure};
use chacha20poly1305::{
    ChaCha20Poly1305, Key, KeyInit as _, Nonce,
    aead::{Aead as _, Generate},
};
use std::{fmt::Debug, fs, io::ErrorKind, path::Path};
use toasty::Embed;

pub const ENCRYPTION_KEY_PATH: &str = concat!(crate::DATA_FOLDER, "/encryption.key");

#[derive(Clone, Embed)]
pub struct Encrypted {
    ciphertext: Vec<u8>,
    nonce: Vec<u8>,
}

impl Encrypted {
    pub fn encrypt(value: Secret) -> anyhow::Result<Self> {
        let key = encryption_key()?;
        let cipher = ChaCha20Poly1305::new(&key);
        let nonce = Nonce::generate();
        let ciphertext = cipher.encrypt(&nonce, value.0.as_bytes())?;

        Ok(Self { ciphertext, nonce: nonce.to_vec() })
    }

    pub fn decrypt(self) -> anyhow::Result<Secret> {
        let key = encryption_key()?;
        let cipher = ChaCha20Poly1305::new(&key);
        let plaintext = cipher.decrypt(&self.nonce()?, self.ciphertext.as_ref())?;
        Ok(Secret(String::from_utf8(plaintext)?))
    }

    pub fn nonce(&self) -> anyhow::Result<Nonce> {
        Nonce::try_from(self.nonce.as_slice()).map_err(|err| anyhow!("Failed to convert nonce: {err:?}"))
    }

    pub fn store(&self, path: impl AsRef<Path>, allow_group: bool) -> anyhow::Result<()> {
        let mut bytes = self.nonce.clone();
        bytes.extend_from_slice(&self.ciphertext);

        store(bytes, path, allow_group)
    }

    pub fn load(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        const NONCE_LEN: usize = 12;

        let bytes = load_bytes(path)?;
        ensure!(bytes.len() > NONCE_LEN, "File is too short to be encrypted");

        let (nonce, ciphertext) = bytes.split_at(NONCE_LEN);
        Ok(Self { ciphertext: ciphertext.to_vec(), nonce: nonce.to_vec() })
    }
}

impl Debug for Encrypted {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(r#"Encrypted("<redacted>")"#)
    }
}

impl std::fmt::Display for Encrypted {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(r#""<encrypted>""#)
    }
}

/// Returns the stored encryption key or generates and stores one
fn encryption_key() -> anyhow::Result<Key> {
    match fs::read(ENCRYPTION_KEY_PATH) {
        Ok(bytes) => Ok(Key::try_from(bytes.as_slice())?),
        Err(err) if err.kind() == ErrorKind::NotFound => {
            let key = Key::generate();
            store(key, ENCRYPTION_KEY_PATH, false)?;
            Ok(key)
        }
        Err(err) => Err(err)?,
    }
}
