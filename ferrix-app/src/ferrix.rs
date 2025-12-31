/* ferrix.rs
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

//! Application data

use std::time::Duration;

use ferrix_core::data::FXData;
use iced::{Subscription, Theme, time};

use crate::{
    FXSettings, Page, SETTINGS_PATH,
    messages::{DataReceiverMessage, Message},
    utils::get_home,
};

#[derive(Debug, Clone)]
pub struct AppState {
    pub current_page: Page,
    pub settings: FXSettings,
    pub request_polkit: bool,
    pub data: FXData,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            current_page: Page::default(),
            settings: FXSettings::read(get_home().join(".config").join(SETTINGS_PATH))
                .unwrap_or_default(),
            request_polkit: false,
            data: FXData::default(),
        }
    }
}

impl AppState {
    pub fn theme(&self) -> Theme {
        self.settings.style.to_theme()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        let update_period = self.settings.update_period as u64;
        let mut scripts = vec![];
        scripts.push(
            time::every(Duration::from_secs(update_period))
                .map(|_| Message::DataReceiver(DataReceiverMessage::GetAllData)),
        );
        Subscription::batch(scripts)
    }
}
