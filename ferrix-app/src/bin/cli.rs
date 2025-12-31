/* cli.rs
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

use anyhow::{Result, anyhow};
use ferrix_core::data::{DataType, FXData};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let arg = env::args().nth(1).unwrap_or("-j".to_string());
    let data = FXData::export_data(&[
        DataType::Processor,
        DataType::Vulnerabilities,
        DataType::CPUFrequency,
        DataType::Memory,
        DataType::Battery,
        DataType::Screen,
        DataType::Distro,
        DataType::Users,
        DataType::Groups,
        DataType::SystemMgr,
        DataType::Software,
        DataType::Kernel,
        DataType::KMods,
        DataType::SystemMisc,
    ])
    .await;
    let json = match &arg as &str {
        "-j" | "--json" => serde_json::to_string(&data),
        "-J" | "--json-pretty" => serde_json::to_string_pretty(&data),
        _ => {
            eprintln!(" * Error: argument '{arg}' is incorrect!");
            eprintln!(" * Supported args: '-j', '--json', '-J', '--json-pretty'");
            return Err(anyhow!("Incorrect usage"));
        }
    }?;
    println!("{json}");
    Ok(())
}
