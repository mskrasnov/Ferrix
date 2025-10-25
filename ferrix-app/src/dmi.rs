//! DMI Service Provider

use anyhow::Result;
use async_std::task;
use ferrix_lib::dmi::{Baseboard, Chassis};
use serde::{Deserialize, Serialize};
use std::process::Command;

use crate::DataLoadingState;

pub async fn get_dmi_data() -> DataLoadingState<DMIResult> {
    let output = task::spawn_blocking(|| {
        Command::new("pkexec")
            .arg("/usr/bin/ferrix-polkit")
            .arg("dmi")
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
    let json_data = DMIResult::from_json(&json_str);

    match json_data {
        Ok(data) => DataLoadingState::Loaded(data),
        Err(why) => DataLoadingState::Error(why.to_string()),
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum DMIResult {
    Ok { data: DMIData },
    Error { error: String },
}

impl DMIResult {
    pub fn new() -> Self {
        match DMIData::new() {
            Ok(data) => Self::Ok { data },
            Err(why) => Self::Error {
                error: why.to_string(),
            },
        }
    }

    pub fn to_json(&self) -> Result<String> {
        let contents = serde_json::to_string(&self)?;
        Ok(contents)
    }

    pub fn from_json(json: &str) -> Result<Self> {
        Ok(serde_json::from_str(json)?)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DMIData {
    pub baseboard: Baseboard,
    pub chassis: Chassis,
}

impl DMIData {
    pub fn new() -> Result<Self> {
        Ok(Self {
            baseboard: Baseboard::new()?,
            chassis: Chassis::new()?,
        })
    }
}
