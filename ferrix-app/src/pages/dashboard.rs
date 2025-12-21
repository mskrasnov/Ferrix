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
    ram::{RAM, Swaps},
    sys::OsRelease,
    utils::Size,
};

use iced::{
    Element, Font, Length, never,
    widget::{
        button, column, container, grid, progress_bar, rich_text, scrollable, space, span, text,
    },
};

#[derive(Debug, Clone, Copy)]
struct SwapUsage<'a> {
    name: &'a str,
    size: Size,
    used: Size,
    size_b: f32,
    used_b: f32,
}

pub fn dashboard<'a>(
    proc: Option<&'a Processors>,
    stat: (Option<&'a Stat>, Option<&'a Stat>),
    ram: Option<&'a RAM>,
    swaps: Option<&'a Swaps>,
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
                ram.total.round(2).unwrap_or(Size::None),
                ram.available
                    .round(2)
                    .unwrap_or(ferrix_lib::utils::Size::None),
            ),
            None => (Size::None, Size::None),
        }
    };
    let total_ram_bytes = total_ram.get_bytes2().unwrap_or(0) as f32;
    let avail_ram_bytes = avail_ram.get_bytes2().unwrap_or(0) as f32;

    let used_ram_bytes = total_ram_bytes - avail_ram_bytes;
    let used_ram = Size::B(used_ram_bytes as usize)
        .round(2)
        .unwrap_or(Size::B(used_ram_bytes as usize));

    let swaps_usage = match swaps {
        Some(swaps) => {
            let mut usage = Vec::with_capacity(swaps.swaps.len());
            for swap in &swaps.swaps {
                let size_b = swap.size.get_bytes2().unwrap_or(0) as f32;
                let used_b = swap.used.get_bytes2().unwrap_or(0) as f32;
                usage.push(SwapUsage {
                    name: &swap.filename,
                    size: swap.size,
                    used: swap.used,
                    size_b,
                    used_b,
                });
            }
            usage
        }
        None => vec![],
    };

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
    let de = match system {
        Some(system) => match &system.desktop {
            Some(de) => de as &str,
            None => "Unknown desktop",
        },
        None => "Unknown desktop",
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

    let mut items = vec![
        card(
            fl!("dash-proc"),
            fl!("dash-proc-info", name = proc_name, threads = proc_threads),
            Message::SelectPage(Page::Processors),
        ),
        widget_card(
            fl!("dash-mem"),
            column![
                column![
                    text(fl!("dash-mem-used", used = used_ram.to_string())),
                    text(fl!("dash-mem-total", total = total_ram.to_string())),
                ],
                progress_bar(0.0..=total_ram_bytes, used_ram_bytes),
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
            Message::SelectPage(Page::SystemMonitor),
        ),
        card(fl!("misc-de"), de, Message::SelectPage(Page::SystemMisc)),
    ];

    for swap in swaps_usage {
        items.push(widget_card(
            fl!("dash-swap"),
            column![
                column![
                    text(swap.name),
                    text(fl!(
                        "dash-mem-used",
                        used = swap.used.round(2).unwrap_or_default().to_string()
                    )),
                    text(fl!(
                        "dash-mem-total",
                        total = swap.size.round(2).unwrap_or_default().to_string()
                    )),
                ],
                progress_bar(0.0..=swap.size_b, swap.used_b),
            ]
            .spacing(5),
            Message::SelectPage(Page::Memory),
        ));
    }

    let mut gr = grid([]).spacing(5).fluid(185.);
    for item in items {
        gr = gr.push(item);
    }
    container(scrollable(gr))
}

fn card<'a, H, C>(header: H, contents: C, on_press: Message) -> button::Button<'a, Message>
where
    H: text::IntoFragment<'a>,
    C: text::IntoFragment<'a>,
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
    H: text::IntoFragment<'a>,
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
