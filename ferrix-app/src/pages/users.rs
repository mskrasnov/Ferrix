/* users.rs
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

//! Users list page

use crate::{
    Message, fl,
    load_state::DataLoadingState,
    pages::{InfoRow, fmt_val, kv_info_table},
};
use ferrix_lib::sys::Users;

use iced::widget::{column, container, scrollable, text};

pub fn users_page<'a>(users: &'a DataLoadingState<Users>) -> container::Container<'a, Message> {
    match users {
        DataLoadingState::Loaded(users) => {
            let mut users_list = column![].spacing(5);
            for usr in &users.users {
                let rows = vec![
                    InfoRow::new(fl!("users-name"), Some(usr.name.clone())),
                    InfoRow::new(fl!("users-id"), fmt_val(Some(usr.uid))),
                    InfoRow::new(fl!("users-gid"), fmt_val(Some(usr.gid))),
                    InfoRow::new(fl!("users-gecos"), usr.gecos.clone()),
                    InfoRow::new(fl!("users-home"), Some(usr.home_dir.clone())),
                    InfoRow::new(fl!("users-shell"), Some(usr.login_shell.clone())),
                ];
                let usr_view = column![
                    text(fl!("users-hdr", id = usr.uid)).style(text::warning),
                    container(kv_info_table(rows)).style(container::rounded_box),
                ]
                .spacing(5);
                users_list = users_list.push(usr_view);
            }
            container(scrollable(users_list))
        }
        DataLoadingState::Error(why) => super::error_page(why),
        DataLoadingState::Loading => super::loading_page(),
    }
}
