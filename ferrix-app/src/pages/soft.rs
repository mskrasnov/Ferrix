/* soft.rs
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

use crate::{DataLoadingState, Message, fl, messages::ButtonsMessage, widgets::table::hdr_name};
use ferrix_lib::soft::{InstalledPackages, Package};

use iced::{
    Length,
    widget::{button, column, container, row as _row, scrollable, space::horizontal, table, text},
};

pub fn soft_page<'a>(
    software: &'a DataLoadingState<InstalledPackages>,
) -> container::Container<'a, Message> {
    container(scrollable(column![soft_list(software),].spacing(5)).spacing(5))
}

fn soft_list<'a>(
    software: &'a DataLoadingState<InstalledPackages>,
) -> container::Container<'a, Message> {
    match software {
        DataLoadingState::Loaded(soft) => {
            let pkgs = &soft.packages;
            let table = container(soft_table(pkgs)).style(container::rounded_box);
            let services_count = text(fl!("soft-total", total = pkgs.len()));

            let layout = column![services_count, table].spacing(5);
            container(scrollable(layout))
        }
        DataLoadingState::Error(why) => super::error_page(why),
        DataLoadingState::Loading => super::loading_page(),
    }
}

fn soft_table<'a>(rows: &'a [Package]) -> table::Table<'a, Message> {
    let columns = [
        table::column(
            _row![horizontal(), hdr_name(fl!("soft-hdr-name"))],
            |row: &'a Package| {
                _row![
                    horizontal(),
                    button(text(&row.name).wrapping(text::Wrapping::WordOrGlyph))
                        .style(button::text)
                        .padding(0)
                        .on_press(Message::Buttons(ButtonsMessage::CopyButtonPressed(
                            format!("{} {}", &row.name, &row.version)
                        ))),
                ]
            },
        )
        .width(Length::Shrink),
        table::column(hdr_name(fl!("soft-hdr-ver")), |row: &'a Package| {
            text(&row.version)
        })
        .width(Length::Shrink),
        table::column(hdr_name(fl!("soft-hdr-arch")), |row: &'a Package| {
            text(&row.arch)
        }),
        table::column(hdr_name(fl!("soft-hdr-type")), |row: &'a Package| {
            text(row.pkg_type.to_string())
        }),
    ];

    table(columns, rows).padding(2).width(Length::Fill)
}
