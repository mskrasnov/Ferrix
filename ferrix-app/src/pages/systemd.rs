/* systemd.rs
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

//! systemd services list

use crate::{Message, fl, load_state::DataLoadingState, pages::hdr_name};
use ferrix_lib::init::{ActiveState, LoadState, ServiceInfo, SystemdServices, WorkState};

use iced::{
    Length, Padding,
    widget::{column, container, row, scrollable, table, text},
};

fn srv_table<'a>(rows: &'a [ServiceInfo]) -> table::Table<'a, Message> {
    let columns = [
        table::column(hdr_name(fl!("sysd-hdr-name")), |row: &'a ServiceInfo| {
            // If the window has a standard size, then some names and
            // descriptions of services will not fit within the limits
            // of one cell of the table, which will lead to an excessive
            // increase in the "Description" column and the almost
            // complete disappearance of the remaining columns. So we
            // change the minimum size of the two largest columns and
            // change the character wrapping logic./There are enough
            // words in the `text` widget so that everything fits,
            // regardless of the size of the window and the table cell.
            text(&row.name).wrapping(text::Wrapping::WordOrGlyph)
        })
        .width(Length::FillPortion(2)),
        table::column(hdr_name(fl!("sysd-hdr-descr")), |row: &ServiceInfo| {
            text(&row.description).wrapping(text::Wrapping::WordOrGlyph)
        })
        .width(Length::FillPortion(3)),
        table::column(hdr_name(fl!("sysd-hdr-load")), |row: &ServiceInfo| {
            text(format!("{}", row.load_state)).style(match row.load_state {
                LoadState::Loaded => text::success,
                LoadState::Stub | LoadState::Masked => text::warning,
                LoadState::NotFound => text::danger,
                _ => text::secondary,
            })
        }),
        table::column(hdr_name(fl!("sysd-hdr-actv")), |row: &ServiceInfo| {
            text(format!("{}", row.active_state)).style(match row.active_state {
                ActiveState::Failed => text::danger,
                ActiveState::Deactivating => text::warning,
                ActiveState::Activating => text::primary,
                ActiveState::Active => text::success,
                _ => text::secondary,
            })
        }),
        table::column(hdr_name(fl!("sysd-hdr-work")), |row: &ServiceInfo| {
            text(format!("{}", row.work_state)).style(match row.work_state {
                WorkState::Active
                | WorkState::Running
                | WorkState::Mounted
                | WorkState::Plugged => text::success,
                WorkState::Exited | WorkState::Dead | WorkState::Failed => text::danger,
                WorkState::Mounting | WorkState::Listening | WorkState::Waiting => text::warning,
                _ => text::secondary,
            })
        }),
    ];

    table(columns, rows).padding(2).width(Length::Fill)
}

pub fn services_page<'a>(
    services: &'a DataLoadingState<SystemdServices>,
) -> container::Container<'a, Message> {
    match services {
        DataLoadingState::Loaded(services) => {
            let units = &services.units;
            let table = container(srv_table(units)).style(container::rounded_box);
            let warn_txt = {
                let hdr = text(fl!("sysd-warning")).style(text::warning);
                let body = text(fl!("sysd-warn-txt"));

                row![hdr, body].spacing(5)
            };
            let services_count = text(fl!("sysd-total", total = units.len()));

            let layout = column![warn_txt, services_count, table,]
                .spacing(5)
                .padding(Padding::new(0.).right(15.));
            container(scrollable(layout))
        }
        DataLoadingState::Error(why) => super::error_page(why),
        DataLoadingState::Loading => super::loading_page(),
    }
}
