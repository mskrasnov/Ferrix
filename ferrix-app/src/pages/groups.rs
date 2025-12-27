/* groups.rs
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

//! Groups list page

use crate::{
    Message, fl,
    load_state::DataLoadingState,
    widgets::table::{InfoRow, fmt_val, kv_info_table},
};
use ferrix_lib::sys::Groups;

use iced::widget::{column, container, scrollable, text};

pub fn groups_page<'a>(groups: &'a DataLoadingState<Groups>) -> container::Container<'a, Message> {
    match groups {
        DataLoadingState::Loaded(groups) => {
            let mut groups_list = column![].spacing(5);
            for grp in &groups.groups {
                let rows = vec![
                    InfoRow::new(fl!("groups-name"), Some(grp.name.clone())),
                    InfoRow::new(fl!("groups-id"), fmt_val(Some(grp.gid))),
                    InfoRow::new(fl!("groups-members"), Some(format!("{:?}", &grp.users))),
                ];
                let grp_view = column![
                    text(fl!("groups-group", group_no = grp.gid)).style(text::warning),
                    container(kv_info_table(rows)).style(container::rounded_box),
                ]
                .spacing(5);
                groups_list = groups_list.push(grp_view);
            }
            container(scrollable(groups_list))
        }
        DataLoadingState::Error(why) => super::error_page(why),
        DataLoadingState::Loading => super::loading_page(),
    }
}
