/* vulnerabilities.rs
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

//! CPU Vulnerabilities page

use crate::{DataLoadingState, Message, fl, widgets::table::hdr_name};
use ferrix_lib::vulnerabilities::Vulnerabilities;

use iced::{
    Length,
    widget::{container, scrollable, table, text},
};

pub fn vulnerabilities_page<'a>(
    vulnerabilities: &'a DataLoadingState<Vulnerabilities>,
) -> container::Container<'a, Message> {
    match vulnerabilities {
        DataLoadingState::Loaded(vulns) => {
            let vulns = &vulns.list;
            let table = container(vuln_table(vulns)).style(container::rounded_box);
            container(scrollable(table))
        }
        DataLoadingState::Error(why) => super::error_page(why),
        DataLoadingState::Loading => super::loading_page(),
    }
}

enum VulnType {
    Safe,
    Warning,
    Danger,
    Unknown,
}

impl VulnType {
    fn detect(descr: &str) -> Self {
        if descr.contains("vulnerable") {
            Self::Danger
        } else if descr.contains("Mitigation") {
            Self::Warning
        } else if descr.contains("Not affected") {
            Self::Safe
        } else {
            Self::Unknown
        }
    }

    fn get_emoji(&self) -> &str {
        match self {
            Self::Safe => "🟢️",
            Self::Warning => "🟠️",
            Self::Danger => "🔴️",
            Self::Unknown => "⚪️",
        }
    }

    fn format(&self, descr: &str) -> String {
        format!("{} {}", self.get_emoji(), descr)
    }
}

fn vuln_table<'a>(rows: &'a [(String, String)]) -> table::Table<'a, Message> {
    let columns = [
        table::column(
            hdr_name(fl!("vuln-hdr-name")),
            |row: &'a (String, String)| text(row.0.trim()).wrapping(text::Wrapping::Word),
        )
        .width(Length::FillPortion(1)),
        table::column(
            hdr_name(fl!("vuln-hdr-descr")),
            |row: &'a (String, String)| {
                let s = row.1.trim();
                let vuln_type = VulnType::detect(s);
                let vuln_str = vuln_type.format(s);

                text(vuln_str).wrapping(text::Wrapping::WordOrGlyph).style(
                    move |t: &iced::Theme| {
                        let p = t.palette();
                        text::Style {
                            color: Some(match vuln_type {
                                VulnType::Safe => p.success,
                                VulnType::Warning => p.warning,
                                VulnType::Danger => p.danger,
                                VulnType::Unknown => p.text,
                            }),
                        }
                    },
                )
            },
        )
        .width(Length::FillPortion(3)),
    ];
    table(columns, rows).padding(2).width(Length::Fill)
}
