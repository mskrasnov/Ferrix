//! Utilities and helper functions

use anyhow::Result;
use std::process::Command;

pub fn xdg_open<O: ToString>(object: O) -> Result<()> {
    Command::new("/usr/bin/xdg-open")
        .arg(object.to_string())
        .spawn()?;
    Ok(())
}
