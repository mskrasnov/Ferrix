/* data.rs
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

//! Get information from `ferrix-lib` crate

use ferrix_lib::{
    battery::BatInfo,
    cpu::{Processors, Stat},
    dmi::{Baseboard, Bios, Chassis, Processor},
    drm::Video,
    init::{Connection, SystemdServices},
    ram::{RAM, Swaps},
    soft::InstalledPackages,
    sys::{Groups, KModules, Kernel, LoadAVG, OsRelease, Uptime, Users},
    vulnerabilities::Vulnerabilities,
};
use serde::{Deserialize, Serialize};

use crate::load_state::LoadState;

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum DataType {
    Overview,
    Processor,
    Vulnerabilities,
    Memory,
    Storage,
    DMITable0,
    DMITable2,
    DMITable3,
    DMITable4,
    Battery,
    Screen,
    Distro,
    Users,
    Groups,
    SystemMgr,
    Software,
    Kernel,
    KMods,
    SystemMisc,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct FXData {
    // Processor info
    pub processors: LoadState<Processors>,
    pub prev_proc_stat: LoadState<Stat>,
    pub curr_proc_stat: LoadState<Stat>,
    pub cpu_vulnerabilities: LoadState<Vulnerabilities>,

    // Memory info
    pub memory_ram: LoadState<RAM>,
    pub memory_swaps: LoadState<Swaps>,

    // DMI info
    pub dmi_bios: LoadState<Bios>,
    pub dmi_baseboard: LoadState<Baseboard>,
    pub dmi_chassis: LoadState<Chassis>,
    pub dmi_proc: LoadState<Processor>,

    // Other info
    pub battery: LoadState<BatInfo>,
    pub video: LoadState<Video>,
    pub distro: LoadState<OsRelease>,
    pub kernel: LoadState<Kernel>,
    pub kmods: LoadState<KModules>,
    pub users: LoadState<Users>,
    pub groups: LoadState<Groups>,
    pub systemd: LoadState<SystemdServices>,
    pub installed_pkgs: LoadState<InstalledPackages>,
    pub system: LoadState<SystemMisc>,
}

impl FXData {
    pub fn new() -> Self {
        FXData {
            ..Default::default()
        }
    }

    pub async fn get(&mut self, data_type: DataType) {
        match data_type {
            DataType::Overview => {
                self.get_proc_stat();
                self.get_memory();
                self.get_distro();
                self.get_battery();
                self.get_system_misc();
            }
            DataType::Processor => self.get_proc_stat(),
            DataType::Vulnerabilities => {
                self.cpu_vulnerabilities = Vulnerabilities::new().to_load_state();
            }
            DataType::Memory => self.get_memory(),
            DataType::Battery => self.get_battery(),
            DataType::Screen => self.video = Video::new().to_load_state(),
            DataType::Distro => self.get_distro(),
            DataType::Users => self.users = Users::new().to_load_state(),
            DataType::Groups => self.groups = Groups::new().to_load_state(),
            DataType::SystemMgr => {
                let conn = match Connection::session().await {
                    Ok(conn) => conn,
                    Err(why) => {
                        self.systemd = LoadState::Error(why.to_string());
                        return;
                    }
                };

                self.systemd = match SystemdServices::new_from_connection(&conn).await {
                    Ok(systemd) => LoadState::Loaded(systemd),
                    Err(why) => LoadState::Error(why.to_string()),
                }
            }
            DataType::Software => self.installed_pkgs = InstalledPackages::get().to_load_state(),
            DataType::Kernel => self.kernel = Kernel::new().to_load_state(),
            DataType::KMods => self.kmods = KModules::new().to_load_state(),
            _ => panic!(),
        }
    }

    fn get_proc_stat(&mut self) {
        self.processors = Processors::new().to_load_state();
        self.update_proc_stat();
    }

    fn update_proc_stat(&mut self) {
        if self.prev_proc_stat.is_none() {
            self.prev_proc_stat = Stat::new().to_load_state();
        } else {
            self.prev_proc_stat = self.curr_proc_stat.clone();
            self.curr_proc_stat = Stat::new().to_load_state();
        }
    }

    fn get_memory(&mut self) {
        self.memory_ram = RAM::new().to_load_state();
        self.memory_swaps = Swaps::new().to_load_state();
    }

    fn get_distro(&mut self) {
        self.distro = OsRelease::new().to_load_state();
    }

    fn get_battery(&mut self) {
        self.battery = BatInfo::new().to_load_state();
    }

    fn get_system_misc(&mut self) {}
}

#[derive(Debug, Clone, Serialize)]
pub struct SystemMisc {
    pub hostname: Option<String>,
    pub loadavg: Option<LoadAVG>,
    pub uptime: Option<Uptime>,
    pub desktop: Option<String>,
    pub language: Option<String>,
    pub env_vars: Vec<(String, String)>,
}

trait ToLoadState<T> {
    fn to_load_state(self) -> LoadState<T>;
}

impl<T> ToLoadState<T> for anyhow::Result<T> {
    fn to_load_state(self) -> LoadState<T> {
        match self {
            Ok(t) => LoadState::Loaded(t),
            Err(why) => LoadState::Error(why.to_string()),
        }
    }
}
