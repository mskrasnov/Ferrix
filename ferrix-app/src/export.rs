/* export.rs
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

//! Export manager

use std::fmt::Display;

use ferrix_lib::{
    battery::BatInfo,
    cpu::Processors,
    drm::Video,
    init::SystemdServices,
    ram::RAM,
    sys::{Groups, KModules, Kernel, OsRelease, Users},
    traits::ToJson,
};
use serde::Serialize;

use crate::DataLoadingState;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ExportStatus {
    LoadingData,
    ErrorLoadingData(String),
    SerializingStructure,
    ErrorSerializing(String),
    WritingData,
    ErrorWritingData(String),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ExportFormat {
    CompressedJson,
    HumanJson,
}

impl ExportFormat {
    pub const ALL: &[Self] = &[Self::CompressedJson, Self::HumanJson];
}

impl Display for ExportFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::CompressedJson => "Compressed JSON",
                Self::HumanJson => "Human-readable JSON",
            }
        )
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ExportMode {
    AllData,
    Selected,
}

impl ExportMode {
    pub const ALL: &[Self] = &[Self::AllData, Self::Selected];
}

impl Display for ExportMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::AllData => "All collected data",
                Self::Selected => "Selected data",
            }
        )
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ExportMember<'a, T> {
    Data { data: Option<&'a T> },
    Error { error_text: String },
}

impl<'a, T> From<&'a DataLoadingState<T>> for ExportMember<'a, T> {
    fn from(value: &'a DataLoadingState<T>) -> Self {
        match value {
            DataLoadingState::Loaded(data) => Self::Data { data: Some(data) },
            DataLoadingState::Loading => Self::Data { data: None },
            DataLoadingState::Error(why) => Self::Error {
                error_text: why.clone(),
            },
        }
    }
}

fn get_data<'a, T>(data: &'a DataLoadingState<T>) -> Option<ExportMember<'a, T>> {
    if let DataLoadingState::Loading = data {
        return None;
    }
    Some(ExportMember::from(data))
}

#[derive(Debug, Clone, Serialize)]
pub struct ExportData<'a> {
    pub cpu: Option<ExportMember<'a, Processors>>,
    pub ram: Option<ExportMember<'a, RAM>>,
    pub battery: Option<ExportMember<'a, BatInfo>>,
    pub drm: Option<ExportMember<'a, Video>>,
    pub os_release: Option<ExportMember<'a, OsRelease>>,
    pub kernel: Option<ExportMember<'a, Kernel>>,
    pub kmods: Option<ExportMember<'a, KModules>>,
    pub users: Option<ExportMember<'a, Users>>,
    pub groups: Option<ExportMember<'a, Groups>>,
    pub systemd: Option<ExportMember<'a, SystemdServices>>,
    pub misc: Option<ExportMember<'a, crate::System>>,
}

impl<'a> From<&'a mut crate::ferrix::Ferrix> for ExportData<'a> {
    fn from(value: &'a mut crate::ferrix::Ferrix) -> Self {
        Self {
            cpu: get_data(&value.data.proc_data),
            ram: get_data(&value.data.ram_data),
            battery: get_data(&value.data.bat_data),
            drm: get_data(&value.data.drm_data),
            os_release: get_data(&value.data.osrel_data),
            kernel: get_data(&value.data.kernel_data),
            kmods: get_data(&value.data.kmods_data),
            users: get_data(&value.data.users_list),
            groups: get_data(&value.data.groups_list),
            systemd: get_data(&value.data.sysd_services_list),
            misc: get_data(&value.data.system),
        }
    }
}

impl<'a> ToJson for ExportData<'a> {}
