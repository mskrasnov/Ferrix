/* traits.rs
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

//! Custom trait objects

use anyhow::Result;
use serde::Serialize;
use std::fmt::Display;

/// A trait for converting structure data to Plain Text for writing
/// to `*.txt` or output to `stdout`
pub trait ToPlainText {
    /// Convert structure fields to `String`
    fn to_plain(&self) -> String;
}

pub fn print_opt_val<T: Display>(param: &str, value: &Option<T>) -> String {
    if let Some(value) = value {
        format!("\t{param}: {value}\n")
    } else {
        format!("")
    }
}

pub fn print_val<T: Display>(param: &str, value: &T) -> String {
    format!("\t{param}: {value}\n")
}

/// A trate with functions for converting data from a structure or
/// other object to the JSON format
pub trait ToJson {
    /// Convert object data to machine-readable JSON format (without
    /// unnecessary indentation and newline transitions)
    fn to_json(&self) -> Result<String>
    where
        Self: Serialize,
    {
        let s = serde_json::to_string(&self)?;
        Ok(s)
    }

    /// Convert object data to human-readable JSON format ("pretty";
    /// with additional newline transitions and indentation)
    fn to_json_pretty(&self) -> Result<String>
    where
        Self: Serialize,
    {
        let s = serde_json::to_string_pretty(&self)?;
        Ok(s)
    }
}
