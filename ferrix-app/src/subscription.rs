/* subscription.rs
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

use crate::{
    Ferrix,
    load_state::LoadState,
    messages::{DataReceiverMessage, Message},
    pages::Page,
};
use iced::{Subscription, time};
use std::time::Duration;

type Script<T> = Subscription<T>;
type OScript<T> = Option<Script<T>>;

const START_UPERIOD: u64 = 10;

trait PushMaybe<T> {
    fn push_maybe(&mut self, data: Option<T>);
}

impl<T> PushMaybe<T> for Vec<T> {
    fn push_maybe(&mut self, data: Option<T>) {
        if let Some(data) = data {
            self.push(data);
        }
    }
}

impl Ferrix {
    pub fn subscription(&self) -> Script<Message> {
        let u = self.settings.update_period as u64;
        let mut scripts = vec![
            // Charts
            time::every(Duration::from_secs(u))
                .map(|_| Message::DataReceiver(DataReceiverMessage::AddCPUCoreLineSeries)),
            time::every(Duration::from_secs(u))
                .map(|_| Message::DataReceiver(DataReceiverMessage::AddTotalRAMUsage)),
            // Charts update
            iced::window::frames()
                .map(|inst| Message::DataReceiver(DataReceiverMessage::AnimationTick(inst))),
        ];
        let oscripts = [
            self.cpu_basic_data(),
            self.cpu_stat_data(),
            self.ram_data(),
            self.swap_data(),
            self.cpu_freq_subscription(),
            self.cpu_vuln_subscription(),
            self.storage_subscription(),
            self.dmi_subscription(),
            self.battery_subscription(),
            self.drm_subscription(),
            self.osrel_subscription(),
            self.users_subscription(),
            self.groups_subscription(),
            self.sysd_subscription(),
            self.soft_subscription(),
            self.env_and_sys_subscription(),
            self.kernel_subscription(),
            self.kmods_subscription(),
        ];
        for oscr in oscripts {
            scripts.push_maybe(oscr);
        }
        Subscription::batch(scripts)
    }

    fn u(&self) -> u64 {
        self.settings.update_period as u64
    }

    fn cpu_basic_data(&self) -> OScript<Message> {
        if (self.current_page == Page::Dashboard || self.current_page == Page::Processors)
            && self.proc_data.is_none()
        {
            Some(
                time::every(Duration::from_millis(START_UPERIOD))
                    .map(|_| Message::DataReceiver(DataReceiverMessage::GetCPUData)),
            )
        } else {
            Some(
                time::every(Duration::from_secs(self.u()))
                    .map(|_| Message::DataReceiver(DataReceiverMessage::GetCPUData)),
            )
        }
    }

    fn cpu_stat_data(&self) -> OScript<Message> {
        if (self.current_page == Page::Dashboard || self.current_page == Page::SystemMonitor)
            && self.curr_proc_stat.is_none()
        {
            Some(
                time::every(Duration::from_millis(START_UPERIOD))
                    .map(|_| Message::DataReceiver(DataReceiverMessage::GetProcStat)),
            )
        } else {
            Some(
                time::every(Duration::from_secs(self.u()))
                    .map(|_| Message::DataReceiver(DataReceiverMessage::GetProcStat)),
            )
        }
    }

    fn ram_data(&self) -> OScript<Message> {
        if (self.current_page == Page::Dashboard || self.current_page == Page::Memory)
            && self.ram_data.is_none()
        {
            Some(
                time::every(Duration::from_millis(START_UPERIOD))
                    .map(|_| Message::DataReceiver(DataReceiverMessage::GetRAMData)),
            )
        } else {
            Some(
                time::every(Duration::from_secs(self.u()))
                    .map(|_| Message::DataReceiver(DataReceiverMessage::GetRAMData)),
            )
        }
    }

    fn swap_data(&self) -> OScript<Message> {
        if (self.current_page == Page::Dashboard || self.current_page == Page::Memory)
            && self.swap_data.is_none()
        {
            Some(
                time::every(Duration::from_millis(START_UPERIOD))
                    .map(|_| Message::DataReceiver(DataReceiverMessage::GetSwapData)),
            )
        } else {
            Some(
                time::every(Duration::from_secs(self.u()))
                    .map(|_| Message::DataReceiver(DataReceiverMessage::GetSwapData)),
            )
        }
    }

    fn cpu_freq_subscription(&self) -> OScript<Message> {
        if self.current_page == Page::CPUFrequency && self.cpu_freq.is_none() {
            Some(
                time::every(Duration::from_millis(START_UPERIOD))
                    .map(|_| Message::DataReceiver(DataReceiverMessage::GetCPUFrequency)),
            )
        } else if self.current_page == Page::CPUFrequency {
            Some(
                time::every(Duration::from_secs(self.u()))
                    .map(|_| Message::DataReceiver(DataReceiverMessage::GetCPUFrequency)),
            )
        } else {
            None
        }
    }

    fn cpu_vuln_subscription(&self) -> OScript<Message> {
        if self.current_page == Page::CPUVulnerabilities && self.cpu_vulnerabilities.is_none() {
            Some(
                time::every(Duration::from_millis(START_UPERIOD))
                    .map(|_| Message::DataReceiver(DataReceiverMessage::GetCPUVulnerabilities)),
            )
        } else {
            None
        }
    }

    fn storage_subscription(&self) -> OScript<Message> {
        None
    }

    fn dmi_subscription(&self) -> OScript<Message> {
        if self.current_page == Page::DMI && !self.is_polkit && self.dmi_data.is_none() {
            Some(
                time::every(Duration::from_secs(1))
                    .map(|_| Message::DataReceiver(DataReceiverMessage::GetDMIData)),
            )
        } else {
            None
        }
    }

    fn battery_subscription(&self) -> OScript<Message> {
        if self.current_page == Page::Dashboard || self.current_page == Page::Battery {
            match self.bat_data {
                LoadState::Loaded(_) => Some(
                    time::every(Duration::from_secs(self.u()))
                        .map(|_| Message::DataReceiver(DataReceiverMessage::GetBatInfo)),
                ),
                _ => Some(
                    time::every(Duration::from_millis(START_UPERIOD))
                        .map(|_| Message::DataReceiver(DataReceiverMessage::GetBatInfo)),
                ),
            }
        } else {
            None
        }
    }

    fn drm_subscription(&self) -> OScript<Message> {
        if self.current_page == Page::Screen && self.drm_data.is_none() {
            Some(
                time::every(Duration::from_millis(START_UPERIOD))
                    .map(|_| Message::DataReceiver(DataReceiverMessage::GetDRMData)),
            )
        } else if self.current_page == Page::Screen && self.drm_data.is_some() {
            Some(
                time::every(Duration::from_secs(self.u()))
                    .map(|_| Message::DataReceiver(DataReceiverMessage::GetDRMData)),
            )
        } else {
            None
        }
    }

    fn osrel_subscription(&self) -> OScript<Message> {
        if (self.current_page == Page::Dashboard || self.current_page == Page::Distro)
            && self.osrel_data.is_none()
        {
            Some(
                time::every(Duration::from_millis(START_UPERIOD))
                    .map(|_| Message::DataReceiver(DataReceiverMessage::GetOsReleaseData)),
            )
        } else {
            None
        }
    }

    fn users_subscription(&self) -> OScript<Message> {
        if self.current_page == Page::Users && self.users_list.is_none() {
            Some(
                time::every(Duration::from_millis(START_UPERIOD))
                    .map(|_| Message::DataReceiver(DataReceiverMessage::GetUsersData)),
            )
        } else {
            None
        }
    }

    fn groups_subscription(&self) -> OScript<Message> {
        if self.current_page == Page::Groups && self.groups_list.is_none() {
            Some(
                time::every(Duration::from_millis(START_UPERIOD))
                    .map(|_| Message::DataReceiver(DataReceiverMessage::GetGroupsData)),
            )
        } else {
            None
        }
    }

    fn sysd_subscription(&self) -> OScript<Message> {
        if self.current_page == Page::SystemManager && self.sysd_services_list.is_none() {
            Some(
                time::every(Duration::from_millis(START_UPERIOD))
                    .map(|_| Message::DataReceiver(DataReceiverMessage::GetSystemdServices)),
            )
        } else if self.current_page == Page::SystemManager && self.sysd_services_list.is_some() {
            Some(
                time::every(Duration::from_secs(self.u() * 10))
                    .map(|_| Message::DataReceiver(DataReceiverMessage::GetSystemdServices)),
            )
        } else {
            None
        }
    }

    fn soft_subscription(&self) -> OScript<Message> {
        if self.current_page == Page::Software && self.installed_pkgs_list.is_none() {
            Some(
                time::every(Duration::from_millis(START_UPERIOD))
                    .map(|_| Message::DataReceiver(DataReceiverMessage::GetPackagesList)),
            )
        } else {
            None
        }
    }

    fn env_and_sys_subscription(&self) -> OScript<Message> {
        if self.current_page == Page::Environment
            || self.current_page == Page::SystemMisc
            || self.current_page == Page::Dashboard
        {
            if self.system.is_none() {
                Some(
                    time::every(Duration::from_millis(START_UPERIOD))
                        .map(|_| Message::DataReceiver(DataReceiverMessage::GetSystemData)),
                )
            } else {
                match self.current_page {
                    Page::SystemMisc => Some(
                        time::every(Duration::from_secs(self.u()))
                            .map(|_| Message::DataReceiver(DataReceiverMessage::GetSystemData)),
                    ),
                    _ => None,
                }
            }
        } else {
            None
        }
    }

    fn kernel_subscription(&self) -> OScript<Message> {
        if self.current_page == Page::Kernel && self.kernel_data.is_none() {
            Some(
                time::every(Duration::from_millis(START_UPERIOD))
                    .map(|_| Message::DataReceiver(DataReceiverMessage::GetKernelData)),
            )
        } else {
            None
        }
    }

    fn kmods_subscription(&self) -> OScript<Message> {
        if self.current_page == Page::KModules && self.kmods_data.is_none() {
            Some(
                time::every(Duration::from_millis(START_UPERIOD))
                    .map(|_| Message::DataReceiver(DataReceiverMessage::GetKModsData)),
            )
        } else {
            None
        }
    }
}
