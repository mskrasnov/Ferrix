/* ferrix.rs
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

use ferrix_lib::{battery::BatInfo, cpu::Processors, cpu_freq::CpuFreq, dmi::{Baseboard, Bios, Chassis, Processor}, drm::Video, init::SystemdServices, ram::{Swaps, RAM}, sys::{Groups, HostName, KModules, Kernel, LoadAVG, OsRelease, Shells, Uptime, Users}, vulnerabilities::Vulnerabilities};

use crate::load_state::LoadState;

#[derive(Debug, Clone)]
pub struct Ferrix {
    /********************************************
     *                CPU Info                  *
     ********************************************/
    pub processors: LoadState<Processors>,
    pub cpu_freq: LoadState<CpuFreq>,
    pub cpu_vulnerabilities: LoadState<Vulnerabilities>,

    /********************************************
     *                RAM Info                  *
     ********************************************/
    pub ram: LoadState<RAM>,
    pub swap: LoadState<Swaps>,

    /********************************************
     *              Screen Info                 *
     ********************************************/
    pub video: LoadState<Video>,

    /********************************************
     *              System Info                 *
     ********************************************/
    pub users: LoadState<Users>,
    pub groups: LoadState<Groups>,
    pub kernel: LoadState<Kernel>,
    pub kmods: LoadState<KModules>,
    pub load_avg: LoadState<LoadAVG>,
    pub os_release: LoadState<OsRelease>,
    pub uptime: LoadState<Uptime>,
    pub env_vars: LoadState<Vec<(String, String)>>,
    pub host_name: LoadState<HostName>,
    pub shells: LoadState<Shells>,
    pub language: LoadState<Option<String>>,
    pub current_desktop: LoadState<Option<String>>,
    pub systemd_services: LoadState<SystemdServices>,

    /********************************************
     *               Battery Info               *
     ********************************************/
    pub battery: LoadState<BatInfo>,

    /********************************************
     *                 DMI Info                 *
     ********************************************/
    pub dmi_bios: LoadState<Bios>,
    pub dmi_baseboard: LoadState<Baseboard>,
    pub dmi_chassis: LoadState<Chassis>,
    pub dmi_proc: LoadState<Processor>,
}
