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

use crate::{Ferrix, Message, Page, fl};
use ferrix_lib::{battery::Status, utils::Size};

use iced::{
    Element, Font, Length, Theme, Vector, color, never,
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

pub fn dashboard<'a>(fx: &'a Ferrix) -> container::Container<'a, Message> {
    let (proc_name, proc_threads) = {
        match fx.proc_data.to_option() {
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
        match fx.ram_data.to_option() {
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
    let used_ram = Size::B(used_ram_bytes as u64)
        .round(2)
        .unwrap_or(Size::B(used_ram_bytes as u64));

    let swaps_usage = match fx.swap_data.to_option() {
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
        match fx.osrel_data.to_option() {
            Some(osr) => match &osr.pretty_name {
                Some(pname) => pname,
                None => &osr.name,
            },
            None => "Generic Linux",
        }
    };
    let hostname = match fx.system.to_option() {
        Some(system) => match &system.hostname {
            Some(hostname) => hostname as &str,
            None => "Unknown hostname",
        },
        None => "Unknown hostname",
    };
    let de = match fx.system.to_option() {
        Some(system) => match &system.desktop {
            Some(de) => de as &str,
            None => "Unknown desktop",
        },
        None => "Unknown desktop",
    };
    let (prev_stat, cur_stat) = (fx.prev_proc_stat.to_option(), fx.curr_proc_stat.to_option());
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

    let battery = match fx.bat_data.to_option() {
        Some(bat) => {
            let mut bats = Vec::with_capacity(bat.bats.len());
            for b in &bat.bats {
                let status = match &b.status {
                    Some(status) => match status {
                        Status::Charging => "⚡️",
                        Status::Discharging => "🔋️",
                        Status::NotCharging => "🚫️",
                        Status::Full => "🟢️",
                        _ => "❔️",
                    },
                    None => "❔️",
                };

                let name = match &b.name {
                    Some(name) => format!("{} {}", status, name),
                    None => fl!("dash-unk-bat"),
                };
                bats.push((name, b.capacity.unwrap_or(0)));
            }
            bats
        }
        None => vec![],
    };

    let mut items = vec![
        card(
            fl!("dash-proc"),
            fl!("dash-proc-info", name = proc_name, threads = proc_threads),
            Message::SelectPage(Page::Processors),
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
        card(fl!("misc-de"), de, Message::SelectPage(Page::SystemMisc)),
    ];

    /* 0 - CPU,
     * 1 - CPU Usage,
     * 2 - RAM Usage,
     * 3...n - Swap usage
     */
    let mut offset = 3;
    for swap in swaps_usage {
        items.insert(
            offset,
            widget_card(
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
            ),
        );
        offset += 1;
    }

    for bat in battery {
        items.push(widget_card(
            fl!("dash-bat"),
            column![
                text(format!("{}: {}%", bat.0, bat.1)),
                progress_bar(0.0..=100., bat.1 as f32)
            ]
            .spacing(5),
            Message::SelectPage(Page::Battery),
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
        .style(|t: &Theme| container::Style {
            shadow: iced::Shadow {
                color: {
                    if t.extended_palette().is_dark {
                        color!(0x1d2021)
                    } else {
                        color!(0xebdbb2)
                    }
                },
                offset: Vector::new(2., 2.),
                blur_radius: 2.,
            },
            ..container::rounded_box(t)
        })
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
        .style(|t| container::Style {
            shadow: iced::Shadow {
                color: {
                    if t.extended_palette().is_dark {
                        color!(0x1d2021)
                    } else {
                        color!(0xebdbb2)
                    }
                },
                offset: Vector::new(2., 2.),
                blur_radius: 2.,
            },
            ..container::rounded_box(t)
        })
        .padding(5),
    )
    .style(button::text)
    .padding(0)
    .on_press(on_press)
}
