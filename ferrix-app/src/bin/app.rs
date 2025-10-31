/* app.rs
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

use ferrix_app::Ferrix;
use iced::{Size, window::Settings};

const APP_LOGO: &[u8] = include_bytes!("../../data/icons/hicolor/scalable/apps/win_logo.png");

pub fn main() -> iced::Result {
    iced::application(Ferrix::default, Ferrix::update, Ferrix::view)
        .settings(iced::Settings {
            default_text_size: iced::Pixels(12.),
            ..Default::default()
        })
        .window(Settings {
            icon: Some(iced::window::icon::from_file_data(APP_LOGO, None).unwrap()),
            min_size: Some(Size {
                width: 790.,
                height: 480.,
            }),
            ..Default::default()
        })
        .window_size((790., 480.))
        .antialiasing(true)
        .subscription(Ferrix::subscription)
        .theme(Ferrix::theme)
        .title("Ferrix System Monitor")
        .run()
}
