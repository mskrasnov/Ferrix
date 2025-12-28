/* kernel.rs
 *
 * Copyright 2025 Michail Krasnov <mskrasnov07@ya.ru>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

//! Information about kernel modules

// TODO v0.5.0: use this module to get modules info!

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
