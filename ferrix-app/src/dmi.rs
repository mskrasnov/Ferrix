/* dmi.rs
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

//! DMI Service Provider

use anyhow::Result;
use async_std::task;
use ferrix_lib::dmi::{Baseboard, Bios, Chassis, Processor};
use serde::{Deserialize, Serialize};
use std::process::Command;

use crate::load_state::{LoadState, ToLoadState};

pub async fn get_dmi_data() -> LoadState<DMIData> {
    let output = task::spawn_blocking(|| {
        Command::new("pkexec")
            .arg("ferrix-polkit")
            .arg("dmi")
            .output()
    })
    .await;

    if let Err(why) = output {
        return LoadState::Error(why.to_string());
    }
    let output = output.unwrap();
    if output.status.code().unwrap_or(0) != 0 {
        return LoadState::Error(format!(
            "[ferrix-polkit] Non-zero return code:\n{}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let json_str = String::from_utf8_lossy(&output.stdout);
    let json_data = DMIData::from_json(&json_str);

    match json_data {
        Ok(data) => LoadState::Loaded(data),
        Err(why) => LoadState::Error(why.to_string()),
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DMIData {
    pub bios: LoadState<Bios>,
    pub baseboard: LoadState<Baseboard>,
    pub chassis: LoadState<Chassis>,
    pub processor: LoadState<Processor>,
}

impl DMIData {
    pub fn new() -> Self {
        Self {
            bios: Bios::new().to_load_state(),
            baseboard: Baseboard::new().to_load_state(),
            chassis: Chassis::new().to_load_state(),
            processor: Processor::new().to_load_state(),
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
