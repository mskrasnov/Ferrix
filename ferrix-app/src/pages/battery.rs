/* battery.rs
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

//! Battery page

use crate::{
    DataLoadingState, Message, fl,
    pages::{InfoRow, fmt_val, kv_info_table},
};
use ferrix_lib::battery::{BatInfo, Battery, Level, Status};

use iced::widget::{column, container, scrollable, text};

pub fn bat_page<'a>(bat_info: &'a DataLoadingState<BatInfo>) -> container::Container<'a, Message> {
    match bat_info {
        DataLoadingState::Loaded(bat_info) => {
            let mut bat_list = column![].spacing(5);
            for bat in &bat_info.bats {
                bat_list = bat_list.push(
                    text(fl!(
                        "bat-header",
                        name = match &bat.name {
                            Some(name) => name.to_string(),
                            None => fl!("bat-unknown-name"),
                        }
                    ))
                    .style(text::warning),
                );
                bat_list = bat_list.push(bat_table(bat));
            }
            container(scrollable(bat_list))
        }
        DataLoadingState::Error(why) => super::error_page(why),
        DataLoadingState::Loading => super::loading_page(),
    }
}

fn bat_table<'a>(bat: &'a Battery) -> container::Container<'a, Message> {
    let rows = vec![
        InfoRow::new(
            fl!("bat-status"),
            Some(match &bat.status {
                Some(Status::Full) => fl!("bat-status-ful"),
                Some(Status::Discharging) => fl!("bat-status-dis"),
                Some(Status::Charging) => fl!("bat-status-cha"),
                Some(Status::NotCharging) => fl!("bat-status-noc"),
                Some(Status::None) => fl!("bat-status-non"),
                Some(Status::Unknown(status)) => fl!("bat-status-unknown", status = status),
                None => fl!("bat-status-isnpresent"),
            }),
        ),
        InfoRow::new(
            fl!("bat-capacity"),
            Some(format!(
                "{} ({}%)",
                match &bat.capacity_level {
                    // Some(Level::Full) => "Full",
                    // Some(Level::Normal) => "Normal",
                    // Some(Level::High) => "High",
                    // Some(Level::Low) => "Low",
                    // Some(Level::Critical) => "Critical!",
                    // _ => "Unknown",
                    Some(Level::Full) => fl!("bat-lvl-ful"),
                    Some(Level::Normal) => fl!("bat-lvl-nor"),
                    Some(Level::High) => fl!("bat-lvl-hig"),
                    Some(Level::Low) => fl!("bat-lvl-low"),
                    Some(Level::Critical) => fl!("bat-lvl-cri"),
                    Some(Level::Unknown(lvl)) => fl!("bat-lvl-unk", lbl = lvl),
                    _ => fl!("bat-lvl-non"),
                },
                match bat.capacity {
                    Some(capacity) => format!("{capacity}"),
                    None => format!("none"),
                }
            )),
        ),
        InfoRow::new(fl!("bat-health"), fmt_val(bat.health)),
        InfoRow::new(fl!("bat-tech"), bat.technology.clone()),
        InfoRow::new(fl!("bat-cycle-cnt"), fmt_val(bat.cycle_count)),
        InfoRow::new(
            fl!("bat-volt-min-des"),
            fmt_val(bat.voltage_min_design),
        ),
        InfoRow::new(fl!("bat-volt-now"), fmt_val(bat.voltage_now)),
        InfoRow::new(fl!("bat-power-now"), fmt_val(bat.power_now)),
        InfoRow::new(fl!("bat-energy-full-des"), fmt_val(bat.energy_full_design)),
        InfoRow::new(fl!("bat-energy-full"), fmt_val(bat.energy_full)),
        InfoRow::new(fl!("bat-energy-now"), fmt_val(bat.energy_now)),
        InfoRow::new(fl!("bat-model"), bat.model_name.clone()),
        InfoRow::new(fl!("bat-manufact"), bat.manufacturer.clone()),
        InfoRow::new(fl!("bat-serial"), bat.serial_number.clone()),
    ];
    container(kv_info_table(rows)).style(container::rounded_box)
}
