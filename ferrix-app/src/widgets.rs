/* widgets.rs
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

//! Custom widgets for UI

use iced::widget::svg::Handle;
use iced::widget::text::IntoFragment;
use iced::widget::tooltip;
use iced::{
    Color,
    widget::{button, container, svg, text, tooltip::Position},
};

use crate::{
    Message,
    icons::{ABOUT_ICON, ERROR_ICON, EXPORT_ICON, SETTINGS_ICON},
    pages::Page,
};

pub fn icon_tooltip<'a, T>(icon_name: &'a str, tooltip: T) -> container::Container<'a, Message>
where
    T: IntoFragment<'a>,
{
    let svg_bytes = match icon_name {
        "about" => ABOUT_ICON,
        "error" => ERROR_ICON,
        "export" => EXPORT_ICON,
        "settings" => SETTINGS_ICON,
        _ => &[],
    };
    let icon = svg(Handle::from_memory(svg_bytes))
        .style(|theme: &iced::Theme, _| svg::Style {
            color: Some(theme.palette().text),
        })
        .width(16)
        .height(16);

    container(iced::widget::tooltip(
        icon,
        container(text(tooltip).style(|s: &iced::Theme| text::Style {
            color: Some(if s.extended_palette().is_dark {
                s.palette().text
            } else {
                Color::WHITE
            }),
        }))
        .padding(2)
        .style(|_| container::Style {
            background: Some(iced::Background::Color(Color::from_rgba8(0, 0, 0, 0.71))),
            border: iced::Border {
                radius: iced::border::Radius::from(2),
                ..iced::Border::default()
            },
            ..Default::default()
        }),
        Position::Bottom,
    ))
    .width(16)
    .height(16)
}

pub fn icon_button<'a>(icon_name: &'a str, tooltip: String) -> button::Button<'a, Message> {
    let svg_bytes = match icon_name {
        "about" => ABOUT_ICON,
        "error" => ERROR_ICON,
        "export" => EXPORT_ICON,
        "settings" => SETTINGS_ICON,
        _ => &[],
    };
    let icon = svg(Handle::from_memory(svg_bytes)).style(|theme: &iced::Theme, _| svg::Style {
        color: Some(theme.palette().text),
    });

    button(iced::widget::tooltip(
        icon.width(16).height(16),
        container(text(tooltip).size(11).style(|s: &iced::Theme| text::Style {
            color: Some(if s.extended_palette().is_dark {
                s.palette().text
            } else {
                Color::WHITE
            }),
        }))
        .padding(2)
        .style(|_| container::Style {
            background: Some(iced::Background::Color(Color::from_rgba8(0, 0, 0, 0.71))),
            border: iced::Border {
                radius: iced::border::Radius::from(2),
                ..iced::Border::default()
            },
            ..Default::default()
        }),
        Position::Bottom,
    ))
    .style(button::subtle)
    .padding(2)
}

pub fn sidebar_button<'a>(page: Page, cur_page: Page) -> button::Button<'a, Message> {
    button(text(page.title_str()))
        .style(if page != cur_page {
            button::subtle
        } else {
            button::secondary
        })
        .on_press(Message::SelectPage(page))
}

pub fn link_button<'a, P>(placeholder: P, link: &'a str) -> tooltip::Tooltip<'a, Message>
where
    P: IntoFragment<'a>,
{
    tooltip(
        button(text(placeholder))
            .style(super::styles::link_button)
            .padding(0)
            .on_press(Message::LinkButtonPressed(link.to_string())),
        container(text(link).size(11).style(|s: &iced::Theme| text::Style {
            color: Some(if s.extended_palette().is_dark {
                s.palette().text
            } else {
                Color::WHITE
            }),
        }))
        .padding(2)
        .style(|_| container::Style {
            background: Some(iced::Background::Color(Color::from_rgba8(0, 0, 0, 0.71))),
            border: iced::Border {
                radius: iced::border::Radius::from(2),
                ..iced::Border::default()
            },
            ..Default::default()
        }),
        Position::Bottom,
    )
}
