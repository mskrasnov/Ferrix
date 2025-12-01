/* vulnerabilities.rs
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

//! Get information about CPU vulnerabilities

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::{read_dir, read_to_string};

use crate::traits::ToJson;

pub type Name = String;
pub type Description = String;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Vulnerabilities {
    pub list: Vec<(Name, Description)>,
}

const VULN_FILES_DIR: &str = "/sys/devices/system/cpu/vulnerabilities/";

impl Vulnerabilities {
    pub fn new() -> Result<Self> {
        let mut list = Vec::new();

        for file in read_dir(VULN_FILES_DIR)? {
            let file = file?.path();
            if file.is_file() {
                let name = match file.file_name() {
                    Some(name) => name.to_string_lossy(),
                    None => continue,
                };
                let description = read_to_string(&file)?;

                list.push((name.to_string(), description));
            }
        }
        list.sort_by_key(|l| l.0.clone());

        Ok(Self { list })
    }
}

impl ToJson for Vulnerabilities {}
