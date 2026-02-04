/* separated_view.rs
 *
 * Copyright 2025-2026 Michail Krasnov <mskrasnov07@ya.ru>
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

use crate::messages::Message;
use iced::{
    Element, Length, Pixels,
    widget::{column, container, scrollable},
};

pub struct SeparatedView<'a> {
    pub first_pane: Element<'a, Message>,
    pub second_pane: Element<'a, Message>,
}

impl<'a> SeparatedView<'a> {
    pub fn new(f: impl Into<Element<'a, Message>>, s: impl Into<Element<'a, Message>>) -> Self {
        Self {
            first_pane: f.into(),
            second_pane: s.into(),
        }
    }

    pub fn view(self) -> Element<'a, Message> {
        let f = container(scrollable(self.first_pane).width(Length::Fill).spacing(5))
            .style(container::rounded_box);
        let s = container(scrollable(self.second_pane).width(Length::Fill).spacing(5))
            .style(container::rounded_box);

        container(column![f.height(Length::Shrink).max_height(Pixels(170.)), s,].spacing(5)).into()
    }
}
