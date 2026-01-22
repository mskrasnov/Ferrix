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
pub mod settings;
pub mod subscription;
use messages::*;

pub use load_state::DataLoadingState;
pub use pages::*;

use dmi::DMIData;
use serde::Serialize;
use widgets::{icon_button, sidebar_button};

use anyhow::Result;
use ferrix_lib::{
    battery::BatInfo,
    cpu::{Processors, Stat},
    cpu_freq::CpuFreq,
    drm::Video,
    init::SystemdServices,
    parts::Mounts,
    ram::{RAM, Swaps},
    soft::InstalledPackages,
    sys::{
        Groups, KModules, Kernel, LoadAVG, OsRelease, Uptime, Users, get_current_desktop,
        get_env_vars, get_hostname, get_lang,
    },
    vulnerabilities::Vulnerabilities,
};
use iced::{
    Alignment::Center,
    Element, Length, Task, Theme,
    widget::{column, container, row, scrollable, text},
};
use std::{collections::HashSet, env::args};

use crate::{settings::FXSettings, utils::get_home, widgets::line_charts::LineChart};

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
    pub cpu_freq: DataLoadingState<CpuFreq>,
    pub cpu_vulnerabilities: DataLoadingState<Vulnerabilities>,

    pub ram_data: DataLoadingState<RAM>,
    pub swap_data: DataLoadingState<Swaps>,
    pub show_mem_chart: HashSet<usize>,
    pub show_ram_chart: bool,
    pub ram_usage_chart: LineChart,

    pub storages: DataLoadingState<Mounts>,
    pub dmi_data: DataLoadingState<DMIData>,
    pub bat_data: DataLoadingState<BatInfo>,
    pub drm_data: DataLoadingState<Video>,
    pub osrel_data: DataLoadingState<OsRelease>,

    pub kernel_data: DataLoadingState<Kernel>,
    pub kmods_data: DataLoadingState<KModules>,

    pub users_list: DataLoadingState<Users>,
    pub groups_list: DataLoadingState<Groups>,
    pub sysd_services_list: DataLoadingState<SystemdServices>,
    pub installed_pkgs_list: DataLoadingState<InstalledPackages>,
    pub system: DataLoadingState<System>,

    pub settings: FXSettings,
    pub is_polkit: bool,
    pub show_copy_toast: bool,
}

impl Default for Ferrix {
    fn default() -> Self {
        let a = args().nth(1);
        let page = match &a {
            Some(a) => Page::from(a as &str),
            None => Page::default(),
        };

        Self {
            current_page: page,

            proc_data: DataLoadingState::Loading,
            prev_proc_stat: DataLoadingState::Loading,
            curr_proc_stat: DataLoadingState::Loading,
            cpu_usage_chart: LineChart::new().legend(true).fill_alpha(0.25).animated(1.),
            show_cpus_chart: HashSet::new(),
            show_chart_elements: 45,
            cpu_freq: DataLoadingState::Loading,
            cpu_vulnerabilities: DataLoadingState::Loading,
            ram_data: DataLoadingState::Loading,
            swap_data: DataLoadingState::Loading,
            ram_usage_chart: LineChart::new().legend(true).fill_alpha(0.25).animated(1.),
            show_mem_chart: HashSet::new(),
            show_ram_chart: true,
            storages: DataLoadingState::Loading,
            dmi_data: DataLoadingState::Loading,
            bat_data: DataLoadingState::Loading,
            drm_data: DataLoadingState::Loading,
            osrel_data: DataLoadingState::Loading,
            kernel_data: DataLoadingState::Loading,
            kmods_data: DataLoadingState::Loading,
            users_list: DataLoadingState::Loading,
            groups_list: DataLoadingState::Loading,
            sysd_services_list: DataLoadingState::Loading,
            installed_pkgs_list: DataLoadingState::Loading,
            system: DataLoadingState::Loading,
            settings: FXSettings::read(get_home().join(".config").join(SETTINGS_PATH))
                .unwrap_or_default(),
            is_polkit: false,
            show_copy_toast: false,
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

impl Ferrix {
    pub fn theme(&self) -> Theme {
        self.settings.style.to_theme()
    }

    pub fn title(&self) -> String {
        format!("Ferrix System Monitor — {}", self.current_page.title_str())
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        message.update(self)
    }

    pub fn view<'a>(&'a self) -> Element<'a, Message> {
        let win = row![sidebar(self.current_page), self.current_page.page(&self)]
            .spacing(5)
            .padding(5);

        if self.show_copy_toast {
            modals::toast(win, fl!("toast-copy")).into()
        } else {
            win.into()
        }
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
        sidebar_button(Page::CPUFrequency, cur_page),
        sidebar_button(Page::CPUVulnerabilities, cur_page),
        sidebar_button(Page::Memory, cur_page),
        sidebar_button(Page::FileSystems, cur_page),
        sidebar_button(Page::DMI, cur_page),
        sidebar_button(Page::Battery, cur_page),
        sidebar_button(Page::Screen, cur_page),
        sidebar_button(Page::Sensors, cur_page),
        text(fl!("sidebar-admin")).style(text::secondary),
        sidebar_button(Page::Distro, cur_page),
        sidebar_button(Page::Users, cur_page),
        sidebar_button(Page::Groups, cur_page),
        sidebar_button(Page::SystemManager, cur_page),
        sidebar_button(Page::Software, cur_page),
        sidebar_button(Page::Environment, cur_page),
        text(fl!("sidebar-system")).style(text::secondary),
        sidebar_button(Page::Kernel, cur_page),
        sidebar_button(Page::KModules, cur_page),
        sidebar_button(Page::Development, cur_page),
        sidebar_button(Page::SystemMisc, cur_page),
        text(fl!("sidebar-manage")).style(text::secondary),
        sidebar_button(Page::Settings, cur_page),
        sidebar_button(Page::About, cur_page),
    ]
    .spacing(3);

    container(column![buttons_bar, scrollable(pages_bar).spacing(5)].spacing(5))
        .padding(5)
        .style(container::bordered_box)
        .height(Length::Fill)
}
