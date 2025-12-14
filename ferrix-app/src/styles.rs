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

use iced::{Color, Theme, color, widget::button};

pub const CPU_CHARTS_COLORS: &'static [Color] = &[
    color!(0xe6194b),
    color!(0xF58231),
    color!(0xFFE119),
    color!(0xBFEF45),
    color!(0x3CB44B),
    color!(0x42D4F4),
    color!(0x4363D8),
    color!(0x911EB4),
    color!(0xff00e3),
    color!(0xffb5ba),
    color!(0x00a800),
    color!(0xfdffc5),
];

/// A link button
pub fn link_button(theme: &Theme, status: button::Status) -> button::Style {
    let palette = theme.extended_palette();

    let base = button::Style {
        text_color: palette.danger.strong.color,
        ..button::Style::default()
    };

    match status {
        button::Status::Active | button::Status::Pressed => base,
        button::Status::Hovered => button::Style {
            text_color: palette.danger.base.color.scale_alpha(0.8),
            ..base
        },
        button::Status::Disabled => button_disabled(base),
    }
}

pub fn button_disabled(style: button::Style) -> button::Style {
    button::Style {
        background: style.background.map(|b| b.scale_alpha(0.5)),
        text_color: style.text_color.scale_alpha(0.5),
        ..style
    }
}
