/* dashboard.rs
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

//! Dashboard page

use crate::{Message, Page, fl};
use ferrix_lib::{
    cpu::{Processors, Stat},
    ram::RAM,
    sys::OsRelease,
};

use iced::{
    Element, Font, Length, never,
    widget::text,
    widget::{
        button, column, container, progress_bar, rich_text, row, space, span, text::IntoFragment,
    },
};

pub fn dashboard<'a>(
    proc: Option<&'a Processors>,
    stat: (Option<&'a Stat>, Option<&'a Stat>),
    ram: Option<&'a RAM>,
    osr: Option<&'a OsRelease>,
    system: Option<&'a crate::System>,
) -> container::Container<'a, Message> {
    let (proc_name, proc_threads) = {
        match proc {
            Some(proc) => {
                let model = &proc.entries[0].model_name;
                let vendor = match model {
                    Some(model) => model,
                    None => "N/A",
                };
                let threads = proc.entries.len();

                (vendor, threads)
            }
            None => ("N/A", 0),
        }
    };
    let (total_ram, avail_ram) = {
        match ram {
            Some(ram) => (
                ram.total.round(2).unwrap_or(ferrix_lib::utils::Size::None),
                ram.available
                    .round(2)
                    .unwrap_or(ferrix_lib::utils::Size::None),
            ),
            None => (ferrix_lib::utils::Size::None, ferrix_lib::utils::Size::None),
        }
    };
    let total_ram_bytes = total_ram.get_bytes2().unwrap_or(0) as f32;
    let avail_ram_bytes = avail_ram.get_bytes2().unwrap_or(0) as f32;
    let os_name = {
        match osr {
            Some(osr) => match &osr.pretty_name {
                Some(pname) => pname,
                None => &osr.name,
            },
            None => "Generic Linux",
        }
    };
    let hostname = match system {
        Some(system) => match &system.hostname {
            Some(hostname) => hostname as &str,
            None => "Unknown hostname",
        },
        None => "Unknown hostname",
    };
    let (prev_stat, cur_stat) = stat;
    let cpu_usage = if prev_stat.is_none() || cur_stat.is_none() {
        0.0
    } else {
        let prev_stat = prev_stat.unwrap();
        let cur_stat = cur_stat.unwrap();

        match &cur_stat.cpu {
            Some(cpu) => cpu.usage_percentage(prev_stat.cpu),
            None => 0.0,
        }
    };

    container(
        column![
            row![
                card(
                    fl!("dash-proc"),
                    fl!("dash-proc-info", name = proc_name, threads = proc_threads),
                    Message::SelectPage(Page::Processors),
                ),
                widget_card(
                    fl!("dash-mem"),
                    column![
                        text(format!("{}/{}", avail_ram, total_ram)),
                        progress_bar(0.0..=total_ram_bytes, total_ram_bytes - avail_ram_bytes),
                    ]
                    .spacing(5),
                    Message::SelectPage(Page::Memory),
                ),
                card(fl!("dash-sys"), os_name, Message::SelectPage(Page::Distro)),
                card(
                    fl!("dash-host"),
                    hostname,
                    Message::SelectPage(Page::SystemMisc),
                ),
            ]
            .spacing(5),
            widget_card(
                fl!("dash-proc-usage"),
                column![
                    text(fl!(
                        "dash-proc-usg_label",
                        usage = format!("{cpu_usage:.2}")
                    )),
                    progress_bar(0.0..=100., cpu_usage),
                ]
                .spacing(5),
                Message::SelectPage(Page::Processors)
            ),
        ]
        .spacing(5),
    )
}

fn card<'a, H, C>(header: H, contents: C, on_press: Message) -> button::Button<'a, Message>
where
    H: IntoFragment<'a>,
    C: IntoFragment<'a>,
{
    button(
        container(
            column![
                rich_text![
                    span(header)
                        .font(Font {
                            weight: iced::font::Weight::Bold,
                            ..Default::default()
                        })
                        .size(16),
                ]
                .on_link_click(never),
                space().width(Length::Fill).height(Length::Fill),
                iced::widget::text(contents),
            ]
            .spacing(5),
        )
        .width(135)
        .max_width(135)
        .height(135)
        .max_height(135)
        .style(container::rounded_box)
        .padding(5),
    )
    .style(button::text)
    .padding(0)
    .on_press(on_press)
}

fn widget_card<'a, H, C>(header: H, contents: C, on_press: Message) -> button::Button<'a, Message>
where
    H: IntoFragment<'a>,
    C: Into<Element<'a, Message>>,
{
    button(
        container(
            column![
                rich_text![
                    span(header)
                        .font(Font {
                            weight: iced::font::Weight::Bold,
                            ..Default::default()
                        })
                        .size(16),
                ]
                .on_link_click(never),
                space().width(Length::Fill).height(Length::Fill),
                contents.into(),
            ]
            .spacing(5),
        )
        .width(135)
        .max_width(135)
        .height(135)
        .max_height(135)
        .style(container::rounded_box)
        .padding(5),
    )
    .style(button::text)
    .padding(0)
    .on_press(on_press)
}
