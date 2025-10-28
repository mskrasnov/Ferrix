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

pub use load_state::DataLoadingState;
pub use pages::*;

use dmi::DMIResult;
use serde::{Deserialize, Serialize};
use widgets::{icon_button, sidebar_button};

use anyhow::Result;
use ferrix_lib::{
    cpu::{Processors, Stat},
    drm::Video,
    init::{Connection, SystemdServices},
    ram::RAM,
    sys::{Groups, LoadAVG, OsRelease, Uptime, Users, get_hostname},
};
use iced::{
    Alignment::Center,
    Element, Length, Subscription, Task, Theme, time,
    widget::{column, container, row, scrollable, text},
};
use std::{fmt::Display, fs, path::Path, time::Duration};

use crate::{dmi::get_dmi_data, utils::get_home};

const SETTINGS_PATH: &str = "./ferrix.conf";

#[derive(Debug, Clone)]
pub enum Message {
    GetCPUData,
    CPUDataReceived(DataLoadingState<Processors>),

    GetProcStat,
    ProcStatReceived(DataLoadingState<Stat>),

    GetRAMData,
    RAMDataReceived(DataLoadingState<RAM>),

    GetDMIData,
    DMIDataReceived(DataLoadingState<DMIResult>),

    GetDRMData,
    DRMDataReceived(DataLoadingState<Video>),

    GetOsReleaseData,
    OsReleaseDataReceived(DataLoadingState<OsRelease>),

    GetKernelData,
    KernelDataReceived(DataLoadingState<KernelData>),

    GetUsersData,
    UsersDataReceived(DataLoadingState<Users>),

    GetGroupsData,
    GroupsDataReceived(DataLoadingState<Groups>),

    GetSystemdServices,
    SystemdServicesReceived(DataLoadingState<SystemdServices>),

    GetSystemData,
    SystemDataReceived(DataLoadingState<System>),

    Dummy,
    ChangeTheme(Style),
    SelectPage(Page),
    ChangeUpdatePeriod(u8),
    LinkButtonPressed(String),
    SaveSettingsButtonPressed,
}

#[derive(Debug)]
pub struct Ferrix {
    pub current_page: Page,
    pub proc_data: DataLoadingState<Processors>,
    pub prev_proc_stat: DataLoadingState<Stat>,
    pub curr_proc_stat: DataLoadingState<Stat>,
    pub ram_data: DataLoadingState<RAM>,
    pub dmi_data: DataLoadingState<DMIResult>,
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
            ram_data: DataLoadingState::Loading,
            dmi_data: DataLoadingState::Loading,
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

#[derive(Debug, Clone)]
pub struct System {
    pub hostname: Option<String>,
    pub loadavg: Option<LoadAVG>,
    pub uptime: Option<Uptime>,
}

impl System {
    pub fn new() -> Result<Self> {
        Ok(Self {
            hostname: get_hostname(),
            loadavg: Some(LoadAVG::new()?),
            uptime: Some(Uptime::new()?),
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
        match message {
            Message::CPUDataReceived(state) => {
                self.proc_data = state;
                Task::none()
            }
            Message::GetCPUData => Task::perform(
                async move {
                    let proc = Processors::new();
                    match proc {
                        Ok(proc) => DataLoadingState::Loaded(proc),
                        Err(why) => DataLoadingState::Error(why.to_string()),
                    }
                },
                |val| Message::CPUDataReceived(val),
            ),
            Message::ProcStatReceived(state) => {
                if self.curr_proc_stat.is_some() {
                    self.prev_proc_stat = self.curr_proc_stat.clone();
                } else if self.curr_proc_stat.is_none() && self.prev_proc_stat.is_none() {
                    self.prev_proc_stat = state.clone();
                }
                self.curr_proc_stat = state;
                Task::none()
            }
            Message::GetProcStat => Task::perform(
                async move {
                    let stat = Stat::new();
                    match stat {
                        Ok(stat) => DataLoadingState::Loaded(stat),
                        Err(why) => DataLoadingState::Error(why.to_string()),
                    }
                },
                |val| Message::ProcStatReceived(val),
            ),
            Message::DMIDataReceived(state) => {
                self.is_polkit = true;
                if state.is_some() && self.is_polkit {
                    self.dmi_data = state;
                } else if !self.is_polkit {
                    self.dmi_data = state;
                }
                Task::none()
            }
            Message::GetDMIData => {
                if self.dmi_data.is_none() && self.current_page == Page::DMI && !self.is_polkit {
                    Task::perform(async move { get_dmi_data().await }, |val| {
                        Message::DMIDataReceived(val)
                    })
                } else {
                    Task::none()
                }
            }
            Message::DRMDataReceived(state) => {
                self.drm_data = state;
                Task::none()
            }
            Message::GetDRMData => Task::perform(
                async move {
                    let drm = Video::new();
                    match drm {
                        Ok(drm) => DataLoadingState::Loaded(drm),
                        Err(why) => DataLoadingState::Error(why.to_string()),
                    }
                },
                |val| Message::DRMDataReceived(val),
            ),
            Message::RAMDataReceived(state) => {
                self.ram_data = state;
                Task::none()
            }
            Message::GetRAMData => Task::perform(
                async move {
                    let ram = RAM::new();
                    match ram {
                        Ok(ram) => DataLoadingState::Loaded(ram),
                        Err(why) => DataLoadingState::Error(why.to_string()),
                    }
                },
                |val| Message::RAMDataReceived(val),
            ),
            Message::OsReleaseDataReceived(state) => {
                self.osrel_data = state;
                Task::none()
            }
            Message::GetOsReleaseData => Task::perform(
                async move {
                    let osrel = OsRelease::new();
                    match osrel {
                        Ok(osrel) => DataLoadingState::Loaded(osrel),
                        Err(why) => DataLoadingState::Error(why.to_string()),
                    }
                },
                |val| Message::OsReleaseDataReceived(val),
            ),
            Message::KernelDataReceived(state) => {
                self.info_kernel = state;
                Task::none()
            }
            Message::GetKernelData => Task::perform(
                async move {
                    let kern = KernelData::new();
                    match kern {
                        Ok(mut kern) => {
                            kern.mods.modules.sort_by_key(|md| md.name.clone());
                            DataLoadingState::Loaded(kern)
                        }
                        Err(why) => DataLoadingState::Error(why.to_string()),
                    }
                },
                |val| Message::KernelDataReceived(val),
            ),
            Message::UsersDataReceived(state) => {
                self.users_list = state;
                Task::none()
            }
            Message::GetUsersData => Task::perform(
                async move {
                    let users = Users::new();
                    match users {
                        Ok(mut users) => {
                            users.users.sort_by_key(|usr| usr.uid);
                            DataLoadingState::Loaded(users)
                        }
                        Err(why) => DataLoadingState::Error(why.to_string()),
                    }
                },
                |val| Message::UsersDataReceived(val),
            ),
            Message::GroupsDataReceived(state) => {
                self.groups_list = state;
                Task::none()
            }
            Message::GetGroupsData => Task::perform(
                async move {
                    let groups = Groups::new();
                    match groups {
                        Ok(mut groups) => {
                            groups.groups.sort_by_key(|grp| grp.gid);
                            DataLoadingState::Loaded(groups)
                        }
                        Err(why) => DataLoadingState::Error(why.to_string()),
                    }
                },
                |val| Message::GroupsDataReceived(val),
            ),
            Message::SystemdServicesReceived(state) => {
                self.sysd_services_list = state;
                Task::none()
            }
            Message::GetSystemdServices => Task::perform(
                async move {
                    let conn = Connection::session().await;
                    if let Err(why) = conn {
                        return DataLoadingState::Error(why.to_string());
                    }
                    let conn = conn.unwrap();

                    let srv_list = SystemdServices::new_from_connection(&conn).await;
                    match srv_list {
                        Ok(srv_list) => DataLoadingState::Loaded(srv_list),
                        Err(why) => DataLoadingState::Error(why.to_string()),
                    }
                },
                |val| Message::SystemdServicesReceived(val),
            ),
            Message::SystemDataReceived(state) => {
                self.system = state;
                Task::none()
            }
            Message::GetSystemData => Task::perform(
                async move {
                    let sys = System::new();
                    match sys {
                        Ok(sys) => DataLoadingState::Loaded(sys),
                        Err(why) => DataLoadingState::Error(why.to_string()),
                    }
                },
                |val| Message::SystemDataReceived(val),
            ),
            Message::SelectPage(page) => {
                self.current_page = page;
                Task::none()
            }
            Message::ChangeTheme(style) => {
                self.settings.style = style;
                Task::none()
            }
            Message::ChangeUpdatePeriod(period) => {
                self.settings.update_period = period;
                Task::none()
            }
            Message::LinkButtonPressed(url) => {
                // TODO: add error handling
                let _ = utils::xdg_open(url);
                Task::none()
            }
            Message::SaveSettingsButtonPressed => {
                // TODO: add error handling
                let _ = self
                    .settings
                    .write(get_home().join(".config").join(SETTINGS_PATH));
                Task::none()
            }
            _ => Task::none(),
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        let mut scripts = vec![
            time::every(Duration::from_secs(self.settings.update_period as u64))
                .map(|_| Message::GetCPUData),
            time::every(Duration::from_secs(self.settings.update_period as u64))
                .map(|_| Message::GetProcStat),
        ];

        if self.current_page == Page::Dashboard {
            scripts.push(
                time::every(Duration::from_secs(self.settings.update_period as u64))
                    .map(|_| Message::GetRAMData),
            );
        }

        if self.osrel_data.is_none()
            && (self.current_page == Page::Distro || self.current_page == Page::Dashboard)
        {
            scripts.push(time::every(Duration::from_millis(10)).map(|_| Message::GetOsReleaseData));
        }

        if self.drm_data.is_none() && self.current_page == Page::Screen {
            scripts.push(time::every(Duration::from_millis(10)).map(|_| Message::GetDRMData));
        } else if self.drm_data.is_some() && self.current_page == Page::Screen {
            scripts.push(
                time::every(Duration::from_secs(self.settings.update_period as u64))
                    .map(|_| Message::GetDRMData),
            );
        }

        if self.info_kernel.is_none() && self.current_page == Page::Kernel {
            scripts.push(time::every(Duration::from_millis(10)).map(|_| Message::GetKernelData));
        }

        if self.users_list.is_none() && self.current_page == Page::Users {
            scripts.push(time::every(Duration::from_millis(10)).map(|_| Message::GetUsersData));
        }

        if self.system.is_none() {
            scripts.push(time::every(Duration::from_millis(10)).map(|_| Message::GetSystemData));
        }

        if self.groups_list.is_none() && self.current_page == Page::Groups {
            scripts.push(time::every(Duration::from_millis(10)).map(|_| Message::GetGroupsData));
        }

        if self.sysd_services_list.is_none() && self.current_page == Page::SystemManager {
            scripts
                .push(time::every(Duration::from_millis(10)).map(|_| Message::GetSystemdServices));
        } else if self.sysd_services_list.is_some() && self.current_page == Page::SystemManager {
            scripts.push(
                time::every(Duration::from_secs(
                    self.settings.update_period as u64 * 10u64,
                ))
                .map(|_| Message::GetSystemdServices),
            );
        }

        if self.system.is_none() && self.current_page == Page::SystemMisc {
            scripts.push(time::every(Duration::from_millis(10)).map(|_| Message::GetSystemData));
        } else if self.system.is_some() && self.current_page == Page::SystemMisc {
            scripts.push(
                time::every(Duration::from_secs(self.settings.update_period as u64))
                    .map(|_| Message::GetSystemData),
            );
        }

        if self.current_page == Page::DMI && !self.is_polkit && self.dmi_data.is_none() {
            scripts.push(time::every(Duration::from_secs(1)).map(|_| Message::GetDMIData));
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
        icon_button("export", fl!("sidebar-export")).on_press(Message::Dummy),
        icon_button("settings", fl!("sidebar-settings"))
            .on_press(Message::SelectPage(Page::Settings)),
        icon_button("about", fl!("sidebar-about")).on_press(Message::SelectPage(Page::About)),
    ]
    .spacing(2)
    .align_y(Center);

    let pages_bar = column![
        text(fl!("sidebar-hardware")).style(text::secondary),
        sidebar_button(Page::Dashboard, cur_page),
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
    .spacing(5);

    container(column![buttons_bar, scrollable(pages_bar)].spacing(5))
        .padding(5)
        .style(container::bordered_box)
        .height(Length::Fill)
}
