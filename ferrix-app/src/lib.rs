/* lib.rs
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

pub mod export;
pub mod i18n;
pub mod icons;
pub mod load_state;
pub mod modals;
pub mod pages;
pub mod styles;
pub mod utils;
pub mod widgets;

pub mod dmi;
pub mod kernel;

// REFACTORED MODULES
pub mod messages;
// pub mod ferrix; // TODO!
use messages::*;

pub use load_state::DataLoadingState;
pub use pages::*;

use dmi::DMIResult;
use serde::{Deserialize, Serialize};
use widgets::{icon_button, sidebar_button};

use anyhow::Result;
use ferrix_lib::{
    battery::BatInfo,
    cpu::{Processors, Stat},
    drm::Video,
    init::SystemdServices,
    ram::{RAM, Swaps},
    sys::{
        Groups, LoadAVG, OsRelease, Uptime, Users, get_current_desktop, get_env_vars, get_hostname,
        get_lang,
    },
};
use iced::{
    Alignment::Center,
    Element, Length, Padding, Subscription, Task, Theme, time,
    widget::{column, container, row, scrollable, text},
};
use std::{collections::HashSet, fmt::Display, fs, path::Path, time::Duration};

use crate::{utils::get_home, widgets::line_charts::LineChart};

const SETTINGS_PATH: &str = "./ferrix.conf";

#[derive(Debug)]
pub struct Ferrix {
    pub current_page: Page,
    pub proc_data: DataLoadingState<Processors>,
    pub prev_proc_stat: DataLoadingState<Stat>,
    pub curr_proc_stat: DataLoadingState<Stat>,
    pub cpu_usage_chart: LineChart,
    pub show_cpus_chart: HashSet<usize>,
    pub show_chart_elements: usize,
    pub ram_data: DataLoadingState<RAM>,
    pub swap_data: DataLoadingState<Swaps>,
    pub show_mem_chart: HashSet<usize>,
    pub show_ram_chart: bool,
    pub ram_usage_chart: LineChart,
    pub dmi_data: DataLoadingState<DMIResult>,
    pub bat_data: DataLoadingState<BatInfo>,
    pub drm_data: DataLoadingState<Video>,
    pub osrel_data: DataLoadingState<OsRelease>,
    pub info_kernel: DataLoadingState<KernelData>,
    pub users_list: DataLoadingState<Users>,
    pub groups_list: DataLoadingState<Groups>,
    pub sysd_services_list: DataLoadingState<SystemdServices>,
    pub system: DataLoadingState<System>,
    pub settings: FXSettings,
    pub is_polkit: bool,
}

impl Default for Ferrix {
    fn default() -> Self {
        Self {
            current_page: Page::default(),
            proc_data: DataLoadingState::Loading,
            prev_proc_stat: DataLoadingState::Loading,
            curr_proc_stat: DataLoadingState::Loading,
            cpu_usage_chart: LineChart::new().legend(true).fill_alpha(0.25).animated(0.9),
            show_cpus_chart: HashSet::new(),
            show_chart_elements: 100,
            ram_data: DataLoadingState::Loading,
            swap_data: DataLoadingState::Loading,
            ram_usage_chart: LineChart::new().legend(true).fill_alpha(0.25).animated(0.9),
            show_mem_chart: HashSet::new(),
            show_ram_chart: true,
            dmi_data: DataLoadingState::Loading,
            bat_data: DataLoadingState::Loading,
            drm_data: DataLoadingState::Loading,
            osrel_data: DataLoadingState::Loading,
            info_kernel: DataLoadingState::Loading,
            users_list: DataLoadingState::Loading,
            groups_list: DataLoadingState::Loading,
            sysd_services_list: DataLoadingState::Loading,
            system: DataLoadingState::Loading,
            settings: FXSettings::read(get_home().join(".config").join(SETTINGS_PATH))
                .unwrap_or_default(),
            is_polkit: false,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct System {
    pub hostname: Option<String>,
    pub loadavg: Option<LoadAVG>,
    pub uptime: Option<Uptime>,
    pub desktop: Option<String>,
    pub language: Option<String>,
    pub env_vars: Vec<(String, String)>,
}

impl System {
    pub fn new() -> Result<Self> {
        Ok(Self {
            hostname: get_hostname(),
            loadavg: Some(LoadAVG::new()?),
            uptime: Some(Uptime::new()?),
            desktop: get_current_desktop(),
            language: get_lang(),
            env_vars: get_env_vars(),
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FXSettings {
    pub update_period: u8,
    pub style: Style,
}

impl FXSettings {
    pub fn read<P: AsRef<Path>>(pth: P) -> Result<Self> {
        let contents = fs::read_to_string(pth)?;
        let data = toml::from_str(&contents)?;
        Ok(data)
    }

    pub fn write<'a, P: AsRef<Path>>(&'a self, pth: P) -> Result<()> {
        let contents = toml::to_string(&self)?;
        fs::write(pth, contents)?;
        Ok(())
    }
}

impl Default for FXSettings {
    fn default() -> Self {
        Self {
            update_period: 1,
            style: Style::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, Default, PartialEq)]
pub enum Style {
    Light,
    #[default]
    Dark,
}

impl Style {
    pub const ALL: &[Self] = &[Self::Light, Self::Dark];

    pub fn to_theme(&self) -> Theme {
        match self {
            Self::Light => Theme::GruvboxLight,
            Self::Dark => Theme::GruvboxDark,
        }
    }
}

impl Display for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Light => fl!("style-light"),
                Self::Dark => fl!("style-dark"),
            }
        )
    }
}

impl Ferrix {
    pub fn theme(&self) -> Theme {
        self.settings.style.to_theme()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        message.update(self)
    }

    pub fn subscription(&self) -> Subscription<Message> {
        let mut scripts = vec![
            time::every(Duration::from_secs(self.settings.update_period as u64))
                .map(|_| Message::DataReceiver(DataReceiverMessage::GetCPUData)),
            time::every(Duration::from_secs(self.settings.update_period as u64))
                .map(|_| Message::DataReceiver(DataReceiverMessage::GetProcStat)),
            // Charts
            time::every(Duration::from_secs(self.settings.update_period as u64))
                .map(|_| Message::DataReceiver(DataReceiverMessage::AddCPUCoreLineSeries)),
            time::every(Duration::from_secs(self.settings.update_period as u64))
                .map(|_| Message::DataReceiver(DataReceiverMessage::AddTotalRAMUsage)),
            // Charts update
            iced::window::frames()
                .map(|inst| Message::DataReceiver(DataReceiverMessage::AnimationTick(inst))),
        ];

        if self.current_page == Page::Dashboard || self.current_page == Page::SystemMonitor {
            scripts.push(
                time::every(Duration::from_secs(self.settings.update_period as u64))
                    .map(|_| Message::DataReceiver(DataReceiverMessage::GetRAMData)),
            );
            scripts.push(
                time::every(Duration::from_secs(self.settings.update_period as u64))
                    .map(|_| Message::DataReceiver(DataReceiverMessage::GetSwapData)),
            );
        }

        if self.osrel_data.is_none()
            && (self.current_page == Page::Distro || self.current_page == Page::Dashboard)
        {
            scripts.push(
                time::every(Duration::from_millis(10))
                    .map(|_| Message::DataReceiver(DataReceiverMessage::GetOsReleaseData)),
            );
        }

        if self.drm_data.is_none() && self.current_page == Page::Screen {
            scripts.push(
                time::every(Duration::from_millis(10))
                    .map(|_| Message::DataReceiver(DataReceiverMessage::GetDRMData)),
            );
        } else if self.drm_data.is_some() && self.current_page == Page::Screen {
            scripts.push(
                time::every(Duration::from_secs(self.settings.update_period as u64))
                    .map(|_| Message::DataReceiver(DataReceiverMessage::GetDRMData)),
            );
        }

        if self.bat_data.is_none() && self.current_page == Page::Battery {
            scripts.push(
                time::every(Duration::from_millis(10))
                    .map(|_| Message::DataReceiver(DataReceiverMessage::GetBatInfo)),
            );
        } else if self.bat_data.is_some() && self.current_page == Page::Battery {
            scripts.push(
                time::every(Duration::from_secs(self.settings.update_period as u64))
                    .map(|_| Message::DataReceiver(DataReceiverMessage::GetBatInfo)),
            );
        }

        if self.info_kernel.is_none() && self.current_page == Page::Kernel {
            scripts.push(
                time::every(Duration::from_millis(10))
                    .map(|_| Message::DataReceiver(DataReceiverMessage::GetKernelData)),
            );
        }

        if self.users_list.is_none() && self.current_page == Page::Users {
            scripts.push(
                time::every(Duration::from_millis(10))
                    .map(|_| Message::DataReceiver(DataReceiverMessage::GetUsersData)),
            );
        }

        if self.system.is_none() {
            scripts.push(
                time::every(Duration::from_millis(10))
                    .map(|_| Message::DataReceiver(DataReceiverMessage::GetSystemData)),
            );
        }

        if self.groups_list.is_none() && self.current_page == Page::Groups {
            scripts.push(
                time::every(Duration::from_millis(10))
                    .map(|_| Message::DataReceiver(DataReceiverMessage::GetGroupsData)),
            );
        }

        if self.sysd_services_list.is_none() && self.current_page == Page::SystemManager {
            scripts.push(
                time::every(Duration::from_millis(10))
                    .map(|_| Message::DataReceiver(DataReceiverMessage::GetSystemdServices)),
            );
        } else if self.sysd_services_list.is_some() && self.current_page == Page::SystemManager {
            scripts.push(
                time::every(Duration::from_secs(
                    self.settings.update_period as u64 * 10u64,
                ))
                .map(|_| Message::DataReceiver(DataReceiverMessage::GetSystemdServices)),
            );
        }

        if self.system.is_none() && self.current_page == Page::SystemMisc {
            scripts.push(
                time::every(Duration::from_millis(10))
                    .map(|_| Message::DataReceiver(DataReceiverMessage::GetSystemData)),
            );
        } else if self.system.is_some() && self.current_page == Page::SystemMisc {
            scripts.push(
                time::every(Duration::from_secs(self.settings.update_period as u64))
                    .map(|_| Message::DataReceiver(DataReceiverMessage::GetSystemData)),
            );
        }

        if self.current_page == Page::DMI && !self.is_polkit && self.dmi_data.is_none() {
            scripts.push(
                time::every(Duration::from_secs(1))
                    .map(|_| Message::DataReceiver(DataReceiverMessage::GetDMIData)),
            );
        }

        Subscription::batch(scripts)
    }

    pub fn view<'a>(&'a self) -> Element<'a, Message> {
        row![sidebar(self.current_page), self.current_page.page(&self)]
            .spacing(5)
            .padding(5)
            .into()
    }
}

fn sidebar<'a>(cur_page: Page) -> container::Container<'a, Message> {
    let buttons_bar = row![
        icon_button("export", fl!("sidebar-export")).on_press(Message::SelectPage(Page::Export)),
        icon_button("settings", fl!("sidebar-settings"))
            .on_press(Message::SelectPage(Page::Settings)),
        icon_button("about", fl!("sidebar-about")).on_press(Message::SelectPage(Page::About)),
    ]
    .spacing(2)
    .align_y(Center);

    let pages_bar = column![
        text(fl!("sidebar-basic")).style(text::secondary),
        sidebar_button(Page::Dashboard, cur_page),
        sidebar_button(Page::SystemMonitor, cur_page),
        text(fl!("sidebar-hardware")).style(text::secondary),
        sidebar_button(Page::Processors, cur_page),
        sidebar_button(Page::Memory, cur_page),
        sidebar_button(Page::Storage, cur_page),
        sidebar_button(Page::DMI, cur_page),
        sidebar_button(Page::Battery, cur_page),
        sidebar_button(Page::Screen, cur_page),
        text(fl!("sidebar-admin")).style(text::secondary),
        sidebar_button(Page::Distro, cur_page),
        sidebar_button(Page::Users, cur_page),
        sidebar_button(Page::Groups, cur_page),
        sidebar_button(Page::SystemManager, cur_page),
        sidebar_button(Page::Software, cur_page),
        sidebar_button(Page::Environment, cur_page),
        sidebar_button(Page::Sensors, cur_page),
        text(fl!("sidebar-system")).style(text::secondary),
        sidebar_button(Page::Kernel, cur_page),
        sidebar_button(Page::KModules, cur_page),
        sidebar_button(Page::Development, cur_page),
        sidebar_button(Page::SystemMisc, cur_page),
        text(fl!("sidebar-manage")).style(text::secondary),
        sidebar_button(Page::Settings, cur_page),
        sidebar_button(Page::About, cur_page),
    ]
    .padding(Padding::new(0.).right(5.))
    .spacing(5);

    container(column![buttons_bar, scrollable(pages_bar)].spacing(5))
        .padding(5)
        .style(container::bordered_box)
        .height(Length::Fill)
}
