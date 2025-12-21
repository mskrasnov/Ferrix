/* pages.rs
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

//! Pages with information about hardware and software

use iced::widget::text::IntoFragment;
use iced::{
    Alignment::{self, Center},
    Element, Length,
    widget::{Column, center, column, container, row, rule, svg::Handle, table, text},
};

use crate::{Ferrix, Message, fl, icons::ERROR_ICON, widgets::link_button};

mod battery;
mod cpu;
mod dashboard;
mod distro;
mod dmi;
mod drm;
mod env;
mod export;
mod groups;
mod kernel;
mod ram;
mod settings;
mod sysmon;
mod system;
mod systemd;
mod users;

pub use kernel::KernelData;
pub use sysmon::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
pub enum Page {
    /************************************
     *       Hardware & dashboard       *
     ************************************/
    #[default]
    Dashboard,
    Processors,
    SystemMonitor,
    Memory,
    Storage,
    DMI,
    Battery,
    Screen,

    /************************************
     *          Administration          *
     ************************************/
    Distro,
    SystemMisc,
    Users,
    Groups,
    SystemManager,
    Software,
    Environment,
    Sensors,

    /************************************
     *               Kernel             *
     ************************************/
    Kernel,
    KModules,
    Development,

    /************************************
     *              Service             *
     ************************************/
    Settings,
    About,
    Export,
    Todo,
}

impl<'a> Page {
    pub fn title(&'a self) -> iced::widget::Column<'a, Message> {
        header_text(self.title_str())
    }

    pub fn title_str(&self) -> String {
        match self {
            Self::Dashboard => fl!("page-dashboard"),
            Self::Processors => fl!("page-procs"),
            Self::SystemMonitor => fl!("page-sysmon"),
            Self::Memory => fl!("page-memory"),
            Self::Storage => fl!("page-storage"),
            Self::DMI => fl!("page-dmi"),
            Self::Battery => fl!("page-battery"),
            Self::Screen => fl!("page-screen"),
            Self::Distro => fl!("page-distro"),
            Self::Users => fl!("page-users"),
            Self::Groups => fl!("page-groups"),
            Self::SystemManager => fl!("page-sysmgr"),
            Self::Software => fl!("page-software"),
            Self::Environment => fl!("page-env"),
            Self::Sensors => fl!("page-sensors"),
            Self::Kernel => fl!("page-kernel"),
            Self::KModules => fl!("page-kmods"),
            Self::Development => fl!("page-dev"),
            Self::SystemMisc => fl!("page-sysmisc"),
            Self::Settings => fl!("page-settings"),
            Self::About => fl!("page-about"),
            Self::Export => fl!("page-export"),
            Self::Todo => fl!("page-todo"),
        }
    }

    pub fn page(&'a self, state: &'a Ferrix) -> Element<'a, Message> {
        let page = match self {
            Self::Dashboard => dashboard::dashboard(
                state.proc_data.to_option(),
                (
                    state.prev_proc_stat.to_option(),
                    state.curr_proc_stat.to_option(),
                ),
                state.ram_data.to_option(),
                state.swap_data.to_option(),
                state.osrel_data.to_option(),
                state.system.to_option(),
            )
            .into(),
            Self::Processors => cpu::proc_page(&state.proc_data).into(),
            Self::SystemMonitor => {
                sysmon::usage_charts_page(&state, &state.curr_proc_stat, &state.prev_proc_stat)
                    .into()
            }
            Self::Memory => ram::ram_page(&state.ram_data).into(),
            Self::DMI => dmi::dmi_page(&state.dmi_data).into(),
            Self::Battery => battery::bat_page(&state.bat_data).into(),
            Self::Screen => drm::drm_page(&state.drm_data).into(),
            Self::Distro => distro::distro_page(&state.osrel_data).into(),
            Self::Kernel => kernel::kernel_page(&state.info_kernel).into(),
            Self::SystemMisc => system::system_page(&state.system).into(),
            Self::Users => users::users_page(&state.users_list).into(),
            Self::Groups => groups::groups_page(&state.groups_list).into(),
            Self::SystemManager => systemd::services_page(&state.sysd_services_list).into(),
            Self::Environment => env::env_page(&state.system).into(),
            Self::Settings => settings::settings_page(&state).into(),
            Self::Export => export::export_page().into(),
            Self::About => self.about_page().into(),
            _ => self.todo_page(),
        };

        column![self.title(), page,].spacing(5).into()
    }

    fn todo_page(&self) -> Element<'a, Message> {
        container(center(
            text(fl!("page-todo-msg")).size(16).style(text::secondary),
        ))
        .into()
    }

    fn about_page(&'a self) -> container::Container<'a, Message> {
        let img = iced::widget::svg("/usr/share/Ferrix/com.mskrasnov.Ferrix.svg")
            .width(128)
            .height(128);
        let header = row![
            img,
            column![
                text(fl!("about-hdr")).size(24),
                text(format!(
                    "{}: {}, {}: {}",
                    fl!("about-ferrix"),
                    env!("CARGO_PKG_VERSION"),
                    fl!("about-flib"),
                    ferrix_lib::FX_LIB_VERSION,
                ))
                .size(14),
            ]
            .spacing(5),
        ]
        .align_y(Center)
        .spacing(5);

        let about_info = row![
            column![
                text(fl!("about-author-hdr")).style(text::secondary),
                text(fl!("about-feedback-hdr")).style(text::secondary),
                text(fl!("about-source-hdr")).style(text::secondary),
                text("crates.io:").style(text::secondary),
                text(fl!("about-blog")).style(text::secondary),
            ]
            .align_x(Alignment::End)
            .spacing(3),
            column![
                row![
                    text(fl!("about-author")),
                    link_button("(GitHub)", "https://github.com/mskrasnov"),
                ]
                .spacing(5),
                link_button("mskrasnov07 at ya dot ru", "mailto:mskrasnov07@ya.ru"),
                link_button("GitHub", "https://github.com/mskrasnov/Ferrix"),
                row![
                    link_button("ferrix-app", "https://crates.io/crates/ferrix-app"),
                    text(", "),
                    link_button("ferrix-lib", "https://crates.io/crates/ferrix-lib"),
                ],
                link_button("mskrasnov", "https://boosty.to/mskrasnov"),
            ]
            .spacing(3),
        ]
        .spacing(5);

        let donate = column![
            text(fl!("about-donate")),
            link_button(fl!("about-donate-lbl"), "https://boosty.to/mskrasnov"),
        ]
        .spacing(5);

        let contents = column![
            column![header, rule::horizontal(1)].spacing(2),
            about_info,
            row![
                text(fl!("about-support")).style(text::warning).size(16),
                rule::horizontal(1)
            ]
            .align_y(Center)
            .spacing(5),
            donate,
        ]
        .spacing(5);

        container(contents)
    }
}

#[derive(Debug, Clone)]
pub struct InfoRow<V> {
    pub param_header: String,
    pub value: Option<V>,
}

impl<V> InfoRow<V> {
    pub fn new<P>(param: P, value: Option<V>) -> Self
    where
        P: Into<String>,
        V: ToString,
    {
        Self {
            param_header: param.into(),
            value,
        }
    }
}

fn text_fmt_val<'a, V>(val: Option<V>) -> Element<'a, Message>
where
    V: ToString + 'a,
{
    match val {
        Some(val) if !val.to_string().is_empty() && !val.to_string().contains("http") => {
            text(val.to_string()).into()
        }
        Some(val) if !val.to_string().is_empty() && val.to_string().contains("http") => {
            link_button(val.to_string(), val.to_string()).into()
        }
        Some(_) => text("N/A").into(),
        None => text("").into(),
    }
}

pub fn kv_info_table<'a, V>(rows: Vec<InfoRow<V>>) -> Element<'a, Message>
where
    V: ToString + Clone + 'a,
{
    let columns = [
        table::column(hdr_name(fl!("hdr-param")), |row: InfoRow<V>| {
            text(row.param_header)
        }),
        table::column(hdr_name(fl!("hdr-value")), |row: InfoRow<V>| {
            text_fmt_val(row.value)
        })
        .width(Length::Fill),
    ];

    table(columns, rows).padding(2).width(Length::Fill).into()
}

fn hdr_name<'a, S: IntoFragment<'a>>(s: S) -> text::Text<'a> {
    text(s).style(text::secondary)
}

fn header_text<'a>(txt: String) -> Column<'a, Message> {
    column![text(txt).size(22), rule::horizontal(1)].spacing(2)
}

fn fmt_val<T>(val: Option<T>) -> Option<String>
where
    T: ToString + Copy,
{
    match val {
        Some(val) => Some(val.to_string()),
        None => None,
    }
}

fn fmt_vec<T>(val: &Option<Vec<T>>) -> Option<String>
where
    T: ToString + Clone,
{
    match val {
        Some(val) => {
            let mut s = String::new();
            for i in val {
                s = format!("{s}{} ", i.to_string());
            }
            Some(s)
        }
        None => None,
    }
}

fn fmt_bool(val: Option<bool>) -> Option<String> {
    match val {
        Some(val) => match val {
            true => Some(fl!("bool-true")),
            false => Some(fl!("bool-false")),
        },
        None => None,
    }
}

fn loading_page<'a>() -> container::Container<'a, Message> {
    container(center(
        text(fl!("ldr-page-tooltip"))
            .style(text::secondary)
            .size(14),
    ))
}

fn error_page<'a>(etext: &'a str) -> container::Container<'a, Message> {
    container(center(
        column![
            row![
                iced::widget::svg(Handle::from_memory(ERROR_ICON))
                    .width(20)
                    .height(20),
                text(fl!("err-page-tooltip")).size(20),
            ]
            .align_y(Center)
            .spacing(5),
            text(etext).style(text::secondary).size(14),
        ]
        .align_x(Center)
        .spacing(5),
    ))
}
