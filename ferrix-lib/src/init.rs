/* init.rs
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

//! Get information about `systemd` services

use std::fmt::Display;

use anyhow::Result;
use serde::Serialize;
use zbus::{Connection, zvariant::OwnedObjectPath};
use zbus_systemd::systemd1::ManagerProxy;

use crate::traits::*;

/// A structure containing information about `systemd` services
#[derive(Debug, Serialize)]
pub struct SystemdServices {
    pub units: Vec<ServiceInfo>,
}

impl SystemdServices {
    pub async fn new_from_connection(conn: &Connection) -> Result<Self> {
        let mgr = ManagerProxy::new(conn).await?;
        let mut units = vec![];

        for unit in mgr.list_units().await? {
            units.push(ServiceInfo::from(unit));
        }

        Ok(Self { units })
    }
}

impl ToJson for SystemdServices {}

impl ToPlainText for SystemdServices {
    fn to_plain(&self) -> String {
        let mut s = format!("\nSystemd services list:");
        for service in &self.units {
            s += &service.to_plain();
        }

        s
    }
}

fn unescape(s: &str) -> String {
    s.replace("\\x20", " ")
        .replace("\\x5c", "\\")
        .replace("\\x2f", "/")
        .replace("\\x2d", "-")
}

type ServiceTuple = (
    String,
    String,
    String,
    String,
    String,
    String,
    OwnedObjectPath,
    u32,
    String,
    OwnedObjectPath,
);

#[derive(Debug, Serialize)]
pub struct ServiceInfo {
    /// Unit name (e.g. `hibernate.target`)
    pub name: String,

    /// Unit description (e.g. `System Hibernation`)
    pub description: String,

    /// Load state
    pub load_state: LoadState,

    /// Active state
    pub active_state: ActiveState,

    /// Work state
    pub work_state: WorkState,

    /// Daemon path
    pub daemon_path: String,

    /// Job ID
    pub job_id: u32,

    /// Unit type
    pub unit_type: UnitType,
}

impl ToPlainText for ServiceInfo {
    fn to_plain(&self) -> String {
        let mut s = format!("\nService \"{}\"\n", &self.name);
        s += &print_val("Description", &self.description);
        s += &print_val("Load state", &self.load_state);
        s += &print_val("Active state", &self.active_state);
        s += &print_val("Work state", &self.work_state);
        s += &print_val("Daemon path", &self.daemon_path);
        s += &print_val("Job ID", &self.job_id);
        s += &print_val("Unit type", &self.unit_type);

        s
    }
}

impl ToJson for ServiceInfo {}

impl From<ServiceTuple> for ServiceInfo {
    fn from(value: ServiceTuple) -> Self {
        Self {
            name: unescape(&value.0),
            description: unescape(&value.1),
            load_state: LoadState::from(&value.2),
            active_state: ActiveState::from(&value.3),
            work_state: WorkState::from(&value.4),
            daemon_path: unescape(&value.5),
            job_id: value.7,
            unit_type: UnitType::from(&value.8),
        }
    }
}

#[derive(Debug, Serialize)]
pub enum LoadState {
    Loaded,
    Stub,
    Masked,
    NotFound,
    Unknown(String),
}

impl Display for LoadState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Loaded => "Loaded",
                Self::Stub => "Stub",
                Self::Masked => "Masked",
                Self::NotFound => "Not found",
                _ => "Unknown",
            }
        )
    }
}

impl From<&String> for LoadState {
    fn from(value: &String) -> Self {
        match value as &str {
            "loaded" => Self::Loaded,
            "stub" => Self::Stub,
            "masked" => Self::Masked,
            "not-found" => Self::NotFound,
            _ => Self::Unknown(value.to_string()),
        }
    }
}

#[derive(Debug, Serialize)]
pub enum ActiveState {
    Active,
    Inactive,
    Activating,
    Deactivating,
    Failed,
    Unknown(String),
}

impl Display for ActiveState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Active => "Active",
                Self::Inactive => "Inactive",
                Self::Activating => "Activating",
                Self::Deactivating => "Deactivating",
                Self::Failed => "Failed",
                _ => "Unknown",
            }
        )
    }
}

impl From<&String> for ActiveState {
    fn from(value: &String) -> Self {
        match value as &str {
            "active" => Self::Active,
            "inactive" => Self::Inactive,
            "activating" => Self::Activating,
            "deactivating" => Self::Deactivating,
            "failed" => Self::Failed,
            _ => Self::Unknown(value.to_string()),
        }
    }
}

#[derive(Debug, Serialize)]
pub enum WorkState {
    Active,
    Running,
    Exited,
    Dead,
    Mounted,
    Mounting,
    Plugged,
    Listening,
    Waiting,
    Failed,
    Unknown(String),
}

impl Display for WorkState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Active => "Active",
                Self::Running => "Running",
                Self::Exited => "Exited",
                Self::Dead => "Dead",
                Self::Mounted => "Mounted",
                Self::Mounting => "Mounting",
                Self::Plugged => "Plugged",
                Self::Listening => "Listening",
                Self::Waiting => "Waiting",
                Self::Failed => "Failed",
                _ => "Unknown",
            }
        )
    }
}

impl From<&String> for WorkState {
    fn from(value: &String) -> Self {
        match value as &str {
            "active" => Self::Active,
            "running" => Self::Running,
            "exited" => Self::Exited,
            "dead" => Self::Dead,
            "mounted" => Self::Mounted,
            "mounting" => Self::Mounting,
            "plugged" => Self::Plugged,
            "listening" => Self::Listening,
            "waiting" => Self::Waiting,
            "failed" => Self::Failed,
            _ => Self::Unknown(value.to_string()),
        }
    }
}

#[derive(Debug, Serialize)]
pub enum UnitType {
    Target,
    Service,
    Mount,
    Swap,
    None,
    Unknown(String),
}

impl Display for UnitType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Target => "Target",
                Self::Service => "Service",
                Self::Mount => "Mount",
                Self::Swap => "Swap",
                Self::None => "None-type",
                _ => "Unknown",
            }
        )
    }
}

impl From<&String> for UnitType {
    fn from(value: &String) -> Self {
        match value as &str {
            "target" => Self::Target,
            "service" => Self::Service,
            "mount" => Self::Mount,
            "swap" => Self::Swap,
            "" => Self::None,
            _ => Self::Unknown(value.to_string()),
        }
    }
}
