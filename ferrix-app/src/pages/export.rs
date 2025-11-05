/* export.rs
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

//! Export Manager page

use crate::{
    export::{ExportFormat, ExportMode},
    messages::{ExportManagerMessage, Message},
};
use iced::widget::{button, column, container, pick_list, row, text};

pub fn export_page<'a>() -> container::Container<'a, Message> {
    container(
        column![
            text("На данный момент будут экспортированы только собранные данные!"),
            row![
                column![text("Формат экспорта:"), text("Экспортируемые данные:"),].spacing(7),
                column![
                    pick_list(
                        ExportFormat::ALL,
                        Some(ExportFormat::CompressedJson),
                        |fmt| Message::ExportManager(ExportManagerMessage::ExportFormatSelected(
                            fmt
                        )),
                    )
                    .padding(2),
                    pick_list(ExportMode::ALL, Some(ExportMode::AllData), |mode| {
                        Message::ExportManager(ExportManagerMessage::ExportModeSelected(mode))
                    },)
                    .padding(2),
                ]
                .spacing(5),
            ]
            .spacing(5),
            button("Экспорт").on_press(Message::ExportManager(ExportManagerMessage::ExportData(
                "export.json".to_string()
            ))),
        ]
        .spacing(5),
    )
}
