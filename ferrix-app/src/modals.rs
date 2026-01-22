/* modals.rs
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

use iced::Alignment::Center;
use iced::widget::text::IntoFragment;
use iced::widget::{button, column, container, row, space, stack, text};
use iced::{Color, Element, Padding};

use crate::fl;
use crate::messages::{ButtonsMessage, Message};

pub fn toast<'a>(
    base: impl Into<Element<'a, Message>>,
    content: impl IntoFragment<'a>,
) -> Element<'a, Message> {
    let toast = container(
        row![
            text(content).style(|t: &iced::Theme| text::Style {
                color: Some(if t.extended_palette().is_dark {
                    t.palette().text
                } else {
                    Color::WHITE
                })
            }),
            button(text(fl!("toast-close")))
                .on_press(Message::Buttons(ButtonsMessage::ShowToastToggle))
                .style(button::subtle)
                .padding(2),
        ]
        .align_y(Center)
        .spacing(8),
    )
    .style(|t| container::Style {
        background: Some(
            Color {
                a: 0.9,
                ..Color::BLACK
            }
            .into(),
        ),
        ..container::rounded_box(t)
    })
    .padding(5);

    stack![
        base.into(),
        column![
            space::vertical(),
            row![space::horizontal(), toast, space::horizontal()]
                .padding(Padding::new(0.).bottom(8.)),
        ],
    ]
    .into()
}
