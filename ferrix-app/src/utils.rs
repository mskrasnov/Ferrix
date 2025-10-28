//! Utilities and helper functions

use anyhow::Result;
use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

pub fn xdg_open<O: ToString>(object: O) -> Result<()> {
    Command::new("/usr/bin/xdg-open")
        .arg(object.to_string())
        .spawn()?;
    Ok(())
}

pub fn get_home() -> PathBuf {
    let home_env = env::var("HOME").unwrap_or("/tmp".to_string());

    Path::new(&home_env).to_path_buf()
}
