/* settings.rs
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

//! Program settings page

use crate::{
    ferrix::Ferrix,
    fl,
    messages::{ButtonsMessage, Message, SettingsMessage},
    settings::{ChartLineThickness, Style},
    widgets::icon_tooltip,
};
use iced::{
    Alignment::Center,
    Element, Pixels,
    widget::{button, center, column, container, pick_list, row, rule, slider, space, text},
};
use std::ops::RangeInclusive;

pub fn settings_page<'a>(state: &'a Ferrix) -> Element<'a, Message> {
    let update_changer = settings_group(
        column![
            settings_item(
                fl!("settings-uper-main"),
                time_slider(
                    1..=15,
                    state.settings.update_period,
                    state.settings.update_period,
                    |per| { Message::Settings(SettingsMessage::ChangeUpdatePeriod(per)) }
                ),
            ),
            settings_item(
                fl!("page-sysmon"),
                time_slider(
                    1..=15,
                    state.settings.charts_update_period_nsecs,
                    format!(
                        "{:.1}",
                        state.settings.charts_update_period_nsecs as f32 * 0.1,
                    ),
                    |per| { Message::Settings(SettingsMessage::ChangeChartsUpdatePeriod(per)) },
                ),
            ),
        ]
        .spacing(5),
    );

    let theme_selector = pick_list(Style::ALL, Some(state.settings.style), |style| {
        Message::Settings(SettingsMessage::ChangeStyle(style))
    })
    .padding(3);
    let chart_line_thick_selector = pick_list(
        ChartLineThickness::ALL,
        Some(state.settings.chart_line_thickness),
        |thickness| Message::Settings(SettingsMessage::ChangeChartLineThickness(thickness)),
    )
    .padding(3);

    let theme_changer = settings_group(
        column![
            settings_item(fl!("settings-look-select"), theme_selector),
            settings_item(fl!("settings-look-thick"), chart_line_thick_selector),
        ]
        .spacing(5),
    );

    let layout = settings_container(
        column![
            settings_header(fl!("settings-update-period"), fl!("settings-uperiod-tip")),
            update_changer,
            settings_header(fl!("settings-look"), fl!("settings-look-tip")),
            theme_changer,
            row![
                space::horizontal(),
                button(text(fl!("settings-save")))
                    .on_press(Message::Buttons(ButtonsMessage::SaveSettingsButtonPressed)),
            ],
        ]
        .spacing(5),
    );
    layout
}

fn settings_container<'a, Message: Clone + 'a>(
    contents: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    row![
        space::horizontal(),
        container(contents.into())
            .width(450)
            .max_width(Pixels(550.)),
        space::horizontal(),
    ]
    .align_y(Center)
    .into()
}

fn settings_group<'a, Message: Clone + 'a>(
    contents: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    container(contents.into())
        .style(container::bordered_box)
        .padding(5)
        .into()
}

fn settings_item<'a, T, C, Message>(header: T, contents: C) -> Element<'a, Message>
where
    T: text::IntoFragment<'a>,
    C: Into<Element<'a, Message>>,
    Message: Clone + 'a,
{
    row![text(header), space::horizontal(), contents.into()]
        .align_y(Center)
        .into()
}

fn settings_header<'a, T>(header: T, tooltip: T) -> row::Row<'a, Message>
where
    T: text::IntoFragment<'a>,
{
    row![
        text(header).size(16),
        icon_tooltip("about", tooltip),
        rule::horizontal(1.)
    ]
    .spacing(5)
    .align_y(Center)
}

fn time_slider<'a, D, Message>(
    range: RangeInclusive<u8>,
    val: u8,
    txt: D,
    on_change: impl Fn(u8) -> Message + 'a,
) -> Element<'a, Message>
where
    D: text::IntoFragment<'a>,
    Message: Clone + 'a,
{
    row![
        slider(range, val, on_change).width(250),
        container(center(text(txt).size(12)))
            .style(container::rounded_box)
            .width(25)
            .height(22),
    ]
    .align_y(Center)
    .spacing(5)
    .into()
}
