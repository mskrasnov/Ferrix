/* cpu.rs
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

//! CPU page

use crate::{
    DataLoadingState, Message, fl,
    pages::{InfoRow, fmt_bool, fmt_val, fmt_vec, kv_info_table},
};
use ferrix_lib::cpu::{Processors, Stat};

use iced::{
    Alignment::Center,
    Padding,
    widget::{column, container, progress_bar, row, scrollable, text},
};

pub fn proc_page<'a>(
    processors: &'a DataLoadingState<Processors>,
    cur_stat: &'a DataLoadingState<Stat>,
    prev_stat: &'a DataLoadingState<Stat>,
) -> container::Container<'a, Message> {
    container(scrollable(
        column![
            proc_usage_charts(cur_stat, prev_stat),
            proc_info(processors),
        ]
        .padding(Padding::new(0.).right(15.))
        .spacing(5),
    ))
}

fn proc_usage_charts<'a>(
    cur_stat: &'a DataLoadingState<Stat>,
    prev_stat: &'a DataLoadingState<Stat>,
) -> container::Container<'a, Message> {
    if cur_stat.is_none() || prev_stat.is_none() {
        return super::error_page("CPU usage statistics are unknown!");
    }
    let cur_stat = cur_stat.to_option().unwrap();
    let prev_stat = prev_stat.to_option().unwrap();

    if cur_stat.cpus.len() != prev_stat.cpus.len() {
        return super::error_page("CPU usage statistics are broken!");
    }
    let i = cur_stat.cpus.len();
    let mut j = 0;
    let mut charts = column![].spacing(5);
    while j < i {
        let cur_usage = &cur_stat.cpus[j];
        let cpu_usage = cur_usage.usage_percentage(Some(prev_stat.cpus[j]));
        charts = charts.push(
            row![
                text(fl!("cpu-processor_no", proc_no = (j + 1))),
                progress_bar(0.0..=100., cpu_usage),
            ]
            .align_y(Center)
            .spacing(5),
        );
        j += 1;
    }

    container(
        column![
            text(fl!("dash-proc-usage")).style(text::warning),
            container(charts).style(container::rounded_box).padding(5),
        ]
        .spacing(5),
    )
}

fn proc_info<'a>(
    processors: &'a DataLoadingState<Processors>,
) -> container::Container<'a, Message> {
    match processors {
        DataLoadingState::Loaded(proc) => {
            let mut proc_list = column![].spacing(5);
            for proc in &proc.entries {
                let rows = vec![
                    InfoRow::new(fl!("cpu-vendor"), proc.vendor_id.clone()),
                    InfoRow::new(fl!("cpu-family"), fmt_val(proc.cpu_family)),
                    InfoRow::new(fl!("cpu-model"), proc.model_name.clone()),
                    InfoRow::new(fl!("cpu-stepping"), fmt_val(proc.stepping)),
                    InfoRow::new(fl!("cpu-microcode"), proc.microcode.clone()),
                    InfoRow::new(fl!("cpu-freq"), fmt_val(proc.cpu_mhz)),
                    InfoRow::new(fl!("cpu-cache"), fmt_val(proc.cache_size)),
                    InfoRow::new(fl!("cpu-physical-id"), fmt_val(proc.physical_id)),
                    InfoRow::new(fl!("cpu-siblings"), fmt_val(proc.siblings)),
                    InfoRow::new(fl!("cpu-core-id"), fmt_val(proc.core_id)),
                    InfoRow::new(fl!("cpu-cpu-cores"), fmt_val(proc.cpu_cores)),
                    InfoRow::new(fl!("cpu-apicid"), fmt_val(proc.apicid)),
                    InfoRow::new(fl!("cpu-iapicid"), fmt_val(proc.initial_apicid)),
                    InfoRow::new(fl!("cpu-fpu"), fmt_bool(proc.fpu)),
                    InfoRow::new(fl!("cpu-fpu-e"), fmt_bool(proc.fpu_exception)),
                    InfoRow::new(fl!("cpu-cpuid-lvl"), fmt_val(proc.cpuid_level)),
                    InfoRow::new(fl!("cpu-wp"), fmt_bool(proc.wp)),
                    InfoRow::new(fl!("cpu-flags"), fmt_vec(&proc.flags)),
                    InfoRow::new(fl!("cpu-bugs"), fmt_vec(&proc.bugs)),
                    InfoRow::new(fl!("cpu-bogomips"), fmt_val(proc.bogomips)),
                    InfoRow::new(fl!("cpu-clflush"), fmt_val(proc.clflush_size)),
                    InfoRow::new(fl!("cpu-cache-align"), fmt_val(proc.cache_alignment)),
                    InfoRow::new(fl!("cpu-address-size"), proc.address_sizes.clone()),
                    InfoRow::new(fl!("cpu-power"), proc.power_management.clone()),
                ];

                let proc_view = column![
                    text(fl!(
                        "cpu-processor_no",
                        proc_no = proc.processor.unwrap_or(0)
                    ))
                    .style(text::warning),
                    container(kv_info_table(rows)).style(container::rounded_box),
                ]
                .spacing(5);
                proc_list = proc_list.push(proc_view);
            }
            container(proc_list)
        }
        DataLoadingState::Error(why) => super::error_page(why),
        DataLoadingState::Loading => super::loading_page(),
    }
}
