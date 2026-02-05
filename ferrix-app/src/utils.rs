/* styles.rs
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

//! Utilities and helper functions

use anyhow::Result;
use iced::Color;
use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

pub fn xdg_open<O: ToString>(object: O) -> Result<()> {
    Command::new("/usr/bin/xdg-open")
        .arg(object.to_string())
        .spawn()?;
    Ok(())
}

pub fn get_home() -> PathBuf {
    let home_env = env::var("HOME").unwrap_or("/tmp".to_string());

    Path::new(&home_env).to_path_buf()
}

pub trait ToColor {
    fn to_color(&self) -> Color;
}

impl ToColor for (u8, u8, u8) {
    fn to_color(&self) -> Color {
        Color::from_rgb8(self.0, self.1, self.2)
    }
}
