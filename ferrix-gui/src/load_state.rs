/* load_state.rs
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

#[derive(Debug, Clone, Default)]
pub enum LoadState<T> {
    #[default]
    Loading,
    Error(String),
    Loaded(T),
}

impl<T> LoadState<T> {
    pub fn to_option<'a>(&'a self) -> Option<&'a T> {
        match self {
            Self::Loaded(data) => Some(data),
            _ => None,
        }
    }

    pub fn is_some(&self) -> bool {
        match self {
            Self::Loaded(_) => true,
            _ => false,
        }
    }

    pub fn is_none(&self) -> bool {
        !self.is_some()
    }

    pub fn is_error(&self) -> bool {
        match self {
            Self::Error(_) => true,
            _ => false,
        }
    }

    pub fn is_ok(&self) -> bool {
        self.is_some() && !self.is_error()
    }
}

impl<T: Clone> LoadState<T> {
    pub fn unwrap(self) -> T {
        match self {
            Self::Loaded(data) => data,
            _ => panic!("No data available"),
        }
    }
}
