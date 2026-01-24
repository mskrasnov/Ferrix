/* settings.rs
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

//! Program settings page

use crate::{
    ferrix::Ferrix,
    fl,
    messages::{ButtonsMessage, Message, SettingsMessage},
    settings::Style,
    widgets::icon_tooltip,
};
use iced::{
    Alignment::Center,
    widget::{button, center, column, container, pick_list, row, rule, slider, text},
};

pub fn settings_page<'a>(state: &'a Ferrix) -> container::Container<'a, Message> {
    let update_sec = slider(1..=10, state.settings.update_period, |per| {
        Message::Settings(SettingsMessage::ChangeUpdatePeriod(per))
    });
    let update_changer = column![
        row![
            text(fl!("settings-update-period")).size(16),
            icon_tooltip("about", fl!("settings-uperiod-tip"),),
            rule::horizontal(1.)
        ]
        .spacing(5)
        .align_y(Center),
        row![
            update_sec,
            container(center(text(state.settings.update_period).size(14)))
                .style(container::rounded_box)
                .width(22)
                .height(22),
        ]
        .spacing(5)
        .align_y(Center),
    ]
    .spacing(5);

    let theme_selector = pick_list(Style::ALL, Some(state.settings.style), |style| {
        Message::Settings(SettingsMessage::ChangeStyle(style))
    });
    let theme_changer = column![
        row![
            text(fl!("settings-look")).size(16),
            icon_tooltip("about", fl!("settings-look-tip")),
            rule::horizontal(1.)
        ]
        .spacing(5)
        .align_y(Center),
        row![text(fl!("settings-look-select")), theme_selector]
            .spacing(5)
            .align_y(Center),
    ]
    .spacing(5);

    let layout = container(
        column![
            update_changer,
            theme_changer,
            button(text(fl!("settings-save")))
                .on_press(Message::Buttons(ButtonsMessage::SaveSettingsButtonPressed)),
        ]
        .spacing(5),
    );
    layout
}
