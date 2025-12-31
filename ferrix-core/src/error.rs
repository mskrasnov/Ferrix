/* error.rs
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

//! Error handling

use serde::{Deserialize, Serialize};
use std::{error, fmt::Display, result};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Error {
    description: Option<String>,
    kind: ErrorKind,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum ErrorKind {
    Unknown,
    PermissionDenied,
    NotAFile,
    NotADir,
    ObjectNotFound,
    IncorrectFormat,
}

impl Error {
    pub fn new(kind: ErrorKind) -> Self {
        Self {
            description: None,
            kind,
        }
    }

    pub fn description(mut self, descr: impl Into<String>) -> Self {
        self.description = Some(descr.into());
        self
    }

    pub fn set_description(&mut self, descr: impl Into<String>) {
        self.description = Some(descr.into());
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match &self.description {
            Some(descr) => descr as &str,
            None => "<error description not provided>",
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.description {
            Some(descr) => write!(f, "{} (kind: {})", descr, self.kind),
            None => write!(f, "<error description not provided> (kind: {})", self.kind),
        }
    }
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Unknown => "unknown",
                Self::PermissionDenied => "permission denied",
                Self::NotAFile => "not a file",
                Self::NotADir => "not a directory",
                Self::ObjectNotFound => "requested object not found",
                Self::IncorrectFormat => "object format is incorrect",
            }
        )
    }
}
