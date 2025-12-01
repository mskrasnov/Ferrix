/* cpu_freq.rs
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

//! Get information about CPU frequency

use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::{
    ffi::OsString,
    fs::{read_dir, read_to_string},
    path::{Path, PathBuf},
};

use crate::traits::ToJson;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CpuFreq {
    pub policy: Vec<Policy>,
    pub boost: Option<bool>,
}

const CPU_FREQ_DIR: &str = "/sys/devices/system/cpu/cpufreq/";

impl CpuFreq {
    pub fn new() -> Result<Self> {
        let pth = Path::new(CPU_FREQ_DIR);
        let boost = match read_to_string(pth.join("boost")).ok() {
            Some(boost) => Some(&boost == "1"),
            None => None,
        };

        let mut policy = Vec::new();
        for dir in read_dir(CPU_FREQ_DIR)? {
            let dir = dir?;
            let fname = dir.file_name();
            if fname.to_string_lossy().contains("policy") {
                policy.push(Policy::new(fname)?);
            }
        }

        Ok(Self { policy, boost })
    }
}

impl ToJson for CpuFreq {}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct Policy {
    pub bios_limit: Option<u32>,
    pub boost: Option<bool>,
    pub cpb: Option<bool>,
    pub cpu_max_freq: Option<u32>,
    pub cpu_min_freq: Option<u32>,
    pub cpuinfo_transition_latency: Option<bool>,
    pub scaling_available_frequencies: Option<Vec<u32>>,
    pub scaling_available_governors: Option<String>,
    pub scaling_cur_freq: Option<u32>,
    pub scaling_driver: Option<String>,
    pub scaling_governor: Option<String>,
    pub scaling_max_freq: Option<u32>,
    pub scaling_min_freq: Option<u32>,
    pub scaling_setspeed: Option<String>,
}

impl Policy {
    pub fn new(policy: OsString) -> Result<Self> {
        let dir = Path::new(CPU_FREQ_DIR);
        let tgt = dir.join(policy);
        if !dir.exists() || !tgt.exists() {
            return Err(anyhow!("Directory {} does not exists!", dir.display()));
        }

        let read = |path: &PathBuf, name: &str| read_to_string(path.join(name));
        let get_bool = |num: Option<u8>| match num {
            Some(num) => Some(if num != 0 { true } else { false }),
            None => None,
        };

        Ok(Self {
            bios_limit: read(&tgt, "bios_limit")?.trim().parse().ok(),
            boost: get_bool(read(&tgt, "boost")?.trim().parse().ok()),
            cpb: get_bool(read(&tgt, "cpb")?.trim().parse().ok()),
            cpu_max_freq: read(&tgt, "cpu_max_freq")?.trim().parse().ok(),
            cpu_min_freq: read(&tgt, "cpu_min_freq")?.trim().parse().ok(),
            cpuinfo_transition_latency: get_bool(
                read(&tgt, "cpuinfo_transition_latency")?
                    .trim()
                    .parse()
                    .ok(),
            ),
            scaling_available_frequencies: Some(
                read(&tgt, "scaling_available_frequencies")?
                    .split_whitespace()
                    .map(|freq| freq.parse::<u32>().ok())
                    .filter(|freq| freq.is_some())
                    .map(|freq| freq.unwrap())
                    .collect::<Vec<_>>(),
            ),
            scaling_available_governors: Some(
                read(&tgt, "scaling_available_governors")?
                    .trim()
                    .split_whitespace()
                    .map(|gov| gov.to_string())
                    .collect(),
            ),
            scaling_cur_freq: read(&tgt, "scaling_cur_freq")?.trim().parse().ok(),
            scaling_driver: read(&tgt, "scaling_driver").ok(),
            scaling_governor: read(&tgt, "scaling_governor").ok(),
            scaling_max_freq: read(&tgt, "scaling_max_freq")?.trim().parse().ok(),
            scaling_min_freq: read(&tgt, "scaling_min_freq")?.trim().parse().ok(),
            scaling_setspeed: read(&tgt, "scaling_setspeed").ok(),
        })
    }
}
