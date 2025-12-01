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

//! Application data

use std::time::Duration;

use ferrix_lib::{
    battery::BatInfo,
    cpu::{Processors, Stat},
    drm::Video,
    init::SystemdServices,
    ram::RAM,
    sys::{Groups, KModules, Kernel, OsRelease, Users},
};
use iced::{Subscription, Task, Theme, time};

use crate::{
    DataLoadingState, FXSettings, Page, SETTINGS_PATH, System,
    dmi::DMIResult,
    messages::{DataReceiverMessage, Message},
    utils::get_home,
};

#[derive(Debug, Clone)]
pub struct AppState {
    pub current_page: Page,
    pub settings: FXSettings,
    pub request_polkit: bool,
    pub data: FXData,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            current_page: Page::default(),
            settings: FXSettings::read(get_home().join(".config").join(SETTINGS_PATH))
                .unwrap_or_default(),
            request_polkit: false,
            data: FXData::default(),
        }
    }
}

impl AppState {
    pub fn theme(&self) -> Theme {
        self.settings.style.to_theme()
    }

    // pub fn update(&mut self, message: Message) -> Task<Message> {
    //     message.update(self)
    // }

    pub fn subscription(&self) -> Subscription<Message> {
        let update_period = self.settings.update_period as u64;
        let page = self.current_page;
        let mut scripts = vec![];

        if page == Page::Dashboard || page == Page::Processors {
            scripts.push(
                time::every(Duration::from_secs(update_period))
                    .map(|_| Message::DataReceiver(DataReceiverMessage::GetCPUData)),
            );
            scripts.push(
                time::every(Duration::from_secs(update_period))
                    .map(|_| Message::DataReceiver(DataReceiverMessage::GetProcStat)),
            );
        }
        if page == Page::Dashboard || page == Page::Memory {
            scripts.push(
                time::every(Duration::from_secs(update_period))
                    .map(|_| Message::DataReceiver(DataReceiverMessage::GetRAMData)),
            );
        }

        match self.current_page {
            Page::Dashboard => {
                let mut subscripts = vec![
                    time::every(Duration::from_secs(update_period))
                        .map(|_| Message::DataReceiver(DataReceiverMessage::GetRAMData)),
                ];
                scripts.append(&mut subscripts);
            }
            _ => {}
        }

        todo!()
    }
}

#[derive(Debug, Clone, Default)]
pub struct FXData {
    pub processors: DataLoadingState<Processors>,
    pub current_proc_stat: DataLoadingState<Stat>,
    pub previous_proc_stat: DataLoadingState<Stat>,
    pub ram: DataLoadingState<RAM>,
    pub dmi: DataLoadingState<DMIResult>,
    pub battery: DataLoadingState<BatInfo>,
    pub video: DataLoadingState<Video>,
    pub os_release: DataLoadingState<OsRelease>,
    pub kernel_summary: DataLoadingState<Kernel>,
    pub kernel_modules: DataLoadingState<KModules>,
    pub users_list: DataLoadingState<Users>,
    pub groups_list: DataLoadingState<Groups>,
    pub systemd_services: DataLoadingState<SystemdServices>,
    pub system_misc: DataLoadingState<System>,
}
