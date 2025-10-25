//! Information about kernel modules

use anyhow::Result;
use async_std::task;
use ferrix_lib::sys::KModules;
use serde::{Serialize, Deserialize};
use std::process::Command;

use crate::DataLoadingState;

pub async fn get_kmodules() -> DataLoadingState<KResult> {
    let output = task::spawn_blocking(|| {
        Command::new("pkexec")
            .arg("/usr/bin/ferrix-polkit")
            .arg("kmods")
            .output()
    })
    .await;

    if let Err(why) = output {
        return DataLoadingState::Error(why.to_string());
    }
    let output = output.unwrap();
    if !output.status.success() {
        return DataLoadingState::Error(format!(
            "[ferrix-polkit] Non-zero return code:\n{}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let json_str = String::from_utf8_lossy(&output.stdout);
    let json_data = KResult::from_json(&json_str);

    match json_data {
        Ok(data) => DataLoadingState::Loaded(data),
        Err(why) => DataLoadingState::Error(why.to_string()),
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum KResult {
    Ok { data: KModules },
    Err { error: String },
}

impl KResult {
    pub fn new() -> Self {
        match KModules::new() {
            Ok(data) => Self::Ok { data, },
            Err(why) => Self::Err { error: why.to_string(), },
        }
    }

    pub fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string(&self)?)
    }

    pub fn from_json(json: &str) -> Result<Self> {
        Ok(serde_json::from_str(json)?)
    }
}
