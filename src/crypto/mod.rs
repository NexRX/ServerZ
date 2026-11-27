// mod encrypted;
mod secret;

// pub use encrypted::*;
pub use secret::*;

use anyhow::ensure;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;

fn store(value: impl AsRef<[u8]>, path: impl AsRef<Path>, allow_group: bool) -> anyhow::Result<()> {
    const OWNER_ONLY: u32 = 0o600;
    const OWNER_AND_GROUP: u32 = 0o660;
    let mode = if allow_group { OWNER_AND_GROUP } else { OWNER_ONLY };
    let path = path.as_ref();

    ensure!(!path.is_dir(), "Path must be a filepath not pointing to a existing directory");
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let mut options = OpenOptions::new();
    options.create(true).write(true).truncate(true);
    #[cfg(unix)]
    options.mode(mode);
    options.open(path)?.write_all(value.as_ref())?;

    Ok(())
}

#[allow(dead_code)]
fn load(path: impl AsRef<Path>) -> anyhow::Result<String> {
    fs::read_to_string(path).map_err(Into::into)
}

#[allow(dead_code)]
fn load_bytes(path: impl AsRef<Path>) -> anyhow::Result<Vec<u8>> {
    fs::read(path).map_err(Into::into)
}
