/* dmi.rs
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

//! DMI table viewer page

use crate::{
    DataLoadingState, Message,
    dmi::DMIResult,
    fl,
    pages::{InfoRow, fmt_bool, fmt_val, fmt_vec, hdr_name, text_fmt_val},
};
use ferrix_lib::dmi::{Baseboard, Chassis, ChassisStateData, Processor};

use iced::{
    Element, Length,
    widget::{column, container, rule, scrollable, table, text},
};

pub fn dmi_page<'a>(dmi: &'a DataLoadingState<DMIResult>) -> container::Container<'a, Message> {
    match dmi {
        DataLoadingState::Loaded(dmi) => match dmi {
            DMIResult::Ok { data } => {
                let baseboard = baseboard_table(&data.baseboard);
                let chassis = chassis_table(&data.chassis);
                let proc = processor_table(&data.processor);

                container(scrollable(
                    column![
                        baseboard,
                        rule::horizontal(1.),
                        chassis,
                        rule::horizontal(1.),
                        proc,
                    ]
                    .spacing(5),
                ))
            }
            DMIResult::Error { error } => super::error_page(error),
        },
        DataLoadingState::Error(why) => super::error_page(why),
        DataLoadingState::Loading => super::loading_page(),
    }
}

fn baseboard_table<'a>(bb: &'a Baseboard) -> container::Container<'a, Message> {
    let rows = vec![
        InfoRow::new("Manufacturer", bb.manufacturer.clone()),
        InfoRow::new("Product", bb.product.clone()),
        InfoRow::new("Serial number", bb.serial_number.clone()),
        InfoRow::new("Asset tag", bb.asset_tag.clone()),
        InfoRow::new("Location in chassis", bb.location_in_chassis.clone()),
        InfoRow::new("Chassis handle", fmt_val(bb.chassis_handle)),
    ];

    let features = match &bb.feature_flags {
        Some(bf) => {
            let rows = vec![
                InfoRow::new("Hosting board", fmt_bool(Some(bf.hosting_board))),
                InfoRow::new(
                    "Requires daughter board",
                    fmt_bool(Some(bf.requires_daughterboard)),
                ),
                InfoRow::new("Removable?", fmt_bool(Some(bf.is_removable))),
                InfoRow::new("Replaceable?", fmt_bool(Some(bf.is_replaceable))),
                InfoRow::new("Hot swappable?", fmt_bool(Some(bf.is_hot_swappable))),
            ];

            container(
                column![
                    text("Baseboard features").style(text::warning),
                    container(kv_info_table(rows)).style(container::rounded_box),
                ]
                .spacing(5),
            )
        }
        None => container(text("Baseboard features is empty!").style(text::danger)),
    };

    let btype = match &bb.board_type {
        Some(bt) => {
            let rows = vec![
                InfoRow::new("Raw value", Some(format!("{}", bt.raw))),
                InfoRow::new("Type", Some(bt.value.to_string())),
            ];

            container(
                column![
                    text("Baseboard type").style(text::warning),
                    container(kv_info_table(rows)).style(container::rounded_box),
                ]
                .spacing(5),
            )
        }
        None => container(text("Unknown baseboard type!").style(text::danger)),
    };

    let bb_view = column![
        text("Baseboard").style(text::warning),
        container(kv_info_table(rows)).style(container::rounded_box),
        features,
        btype,
    ]
    .spacing(5);

    container(bb_view)
}

fn chassis_table<'a>(c: &'a Chassis) -> container::Container<'a, Message> {
    let rows = vec![
        InfoRow::new("Manufacturer", c.manufacturer.clone()),
        InfoRow::new("Version", c.version.clone()),
        InfoRow::new("Serial number", c.serial_number.clone()),
        InfoRow::new("Asset tag", c.asset_tag_number.clone()),
        InfoRow::new("OEM Defined", fmt_val(c.oem_defined)),
        InfoRow::new("Contained elements", fmt_val(c.contained_element_count)),
        InfoRow::new(
            "Contained elements record length",
            fmt_val(c.contained_element_record_length),
        ),
        InfoRow::new("SKU Number", c.sku_number.clone()),
    ];

    let chassis_type = match &c.chassis_type {
        Some(ct) => {
            let rows = vec![
                InfoRow::new("Raw", fmt_val(Some(ct.raw))),
                InfoRow::new("Type", Some(ct.value.to_string())),
                InfoRow::new("Lock presence", Some(ct.lock_presence.to_string())),
            ];
            container(
                column![
                    text("Chassis type").style(text::warning),
                    container(kv_info_table(rows)).style(container::rounded_box)
                ]
                .spacing(5),
            )
        }
        None => container(text("Unknown chassis type").style(text::danger)),
    };

    let bootup_state = match &c.bootup_state {
        Some(bs) => chassis_state(bs, "Bootup state"),
        None => container(text("Unknown bootup state").style(text::danger)),
    };
    let ps_state = match &c.power_supply_state {
        Some(pss) => chassis_state(pss, "Power Supply state"),
        None => container(text("Unknown power supply state").style(text::danger)),
    };
    let t_state = match &c.thermal_state {
        Some(ts) => chassis_state(ts, "Thermal state"),
        None => container(text("Unknown thermal state").style(text::danger)),
    };

    let security_status = match &c.security_status {
        Some(ss) => {
            let rows = vec![
                InfoRow::new("Raw", fmt_val(Some(ss.raw))),
                InfoRow::new("Status", Some(ss.value.to_string())),
            ];
            container(
                column![
                    text("Security status").style(text::warning),
                    container(kv_info_table(rows)).style(container::rounded_box)
                ]
                .spacing(5),
            )
        }
        None => container(text("Unknown security status!").style(text::danger)),
    };

    let chassis_view = column![
        text("Chassis").style(text::warning),
        container(kv_info_table(rows)).style(container::rounded_box),
        chassis_type,
        bootup_state,
        ps_state,
        t_state,
        security_status,
    ]
    .spacing(5);

    container(chassis_view)
}

fn chassis_state<'a>(
    state: &'a ChassisStateData,
    hdr: &'a str,
) -> container::Container<'a, Message> {
    let rows = vec![
        InfoRow::new("Raw", fmt_val(Some(state.raw))),
        InfoRow::new("State", Some(state.value.to_string())),
    ];

    container(
        column![
            text(hdr).style(text::warning),
            container(kv_info_table(rows)).style(container::rounded_box)
        ]
        .spacing(5),
    )
}

fn processor_table<'a>(p: &'a Processor) -> container::Container<'a, Message> {
    let rows = vec![
        InfoRow::new(
            "Raw Processor ID",
            match p.processor_id {
                Some(pid) => fmt_vec(&Some(pid.to_vec())),
                None => None,
            },
        ),
        InfoRow::new("Socket reference designation", p.socked_designation.clone()),
        InfoRow::new(
            "Processor type",
            match &p.processor_type {
                Some(pt) => Some(pt.value.to_string()),
                None => None,
            },
        ),
        InfoRow::new(
            "Processor family",
            match &p.processor_family {
                Some(pf) => Some(pf.value.to_string()),
                None => None,
            },
        ),
        InfoRow::new(
            "Processor family #2",
            match &p.processor_family_2 {
                Some(pf) => Some(pf.value.to_string()),
                None => None,
            },
        ),
        InfoRow::new("Processor manufacturer", p.processor_manufacturer.clone()),
        InfoRow::new("Processor version", p.processor_version.clone()),
        InfoRow::new(
            "External clock",
            match &p.external_clock {
                Some(ec) => Some(ec.to_string()),
                None => None,
            },
        ),
        InfoRow::new(
            "Max speed",
            match &p.max_speed {
                Some(ms) => Some(ms.to_string()),
                None => None,
            },
        ),
        InfoRow::new(
            "Current speed",
            match &p.current_speed {
                Some(cs) => Some(cs.to_string()),
                None => None,
            },
        ),
        InfoRow::new(
            "Socket populated",
            match &p.status {
                Some(ps) => fmt_bool(Some(ps.socket_populated)),
                None => None,
            },
        ),
        InfoRow::new(
            "CPU Status",
            match &p.status {
                Some(ps) => Some(ps.cpu_status.to_string()),
                None => None,
            },
        ),
        InfoRow::new(
            "Processor Upgrade",
            match &p.processor_upgrade {
                Some(pu) => Some(pu.value.to_string()),
                None => None,
            },
        ),
        InfoRow::new("Serial number", p.serial_number.clone()),
        InfoRow::new("Asset tag", p.asset_tag.clone()),
        InfoRow::new("Part number", p.part_number.clone()),
        InfoRow::new(
            "Core count",
            match &p.core_count {
                Some(cc) => Some(cc.to_string()),
                None => None,
            },
        ),
        InfoRow::new(
            "Cores enabled",
            match &p.cores_enabled {
                Some(ce) => Some(ce.to_string()),
                None => None,
            },
        ),
        InfoRow::new(
            "Thread count",
            match &p.thread_count {
                Some(tc) => Some(tc.to_string()),
                None => None,
            },
        ),
        InfoRow::new(
            "Core count #2",
            match &p.core_count_2 {
                Some(cc) => Some(cc.to_string()),
                None => None,
            },
        ),
        InfoRow::new(
            "Cores enabled #2",
            match &p.cores_enabled_2 {
                Some(ce) => Some(ce.to_string()),
                None => None,
            },
        ),
        InfoRow::new(
            "Thread count #2",
            match &p.thread_count_2 {
                Some(tc) => Some(tc.to_string()),
                None => None,
            },
        ),
    ];

    container(
        column![
            text("Processor").style(text::warning),
            container(kv_info_table(rows)).style(container::rounded_box),
            processor_characteristics_table(p),
            processor_voltage_table(p),
        ]
        .spacing(5),
    )
}

fn processor_voltage_table<'a>(p: &'a Processor) -> container::Container<'a, Message> {
    let voltage = &p.voltage;
    match voltage {
        None => container(text("Unknown processor voltage!").style(text::danger)),
        Some(v) => {
            let mut rows = vec![];
            match v {
                ferrix_lib::dmi::ProcessorVoltage::CurrentVolts(volts) => {
                    rows.push(InfoRow::new("Current voltage", fmt_val(Some(volts))))
                }
                ferrix_lib::dmi::ProcessorVoltage::SupportedVolts(volts) => {
                    rows.push(InfoRow::new(
                        "5.0V Supported",
                        fmt_bool(Some(volts.volts_5_0)),
                    ));
                    rows.push(InfoRow::new(
                        "3.3V Supported",
                        fmt_bool(Some(volts.volts_3_3)),
                    ));
                    rows.push(InfoRow::new(
                        "2.9V Supported",
                        fmt_bool(Some(volts.volts_2_9)),
                    ));
                    rows.push(InfoRow::new(
                        "Other supported voltages",
                        fmt_vec(&Some(volts.voltages.clone())),
                    ));
                }
            }

            container(
                column![
                    text("Processor voltage").style(text::warning),
                    container(kv_info_table(rows)).style(container::rounded_box),
                ]
                .spacing(5),
            )
        }
    }
}

fn processor_characteristics_table<'a>(p: &'a Processor) -> container::Container<'a, Message> {
    let chars = &p.processors_characteristics;
    match chars {
        None => container(text("Processor characteristics is not present!").style(text::danger)),
        Some(c) => {
            let rows = vec![
                InfoRow::new("64-bit capable", fmt_bool(Some(c.bit_64capable))),
                InfoRow::new("128-bit capable", fmt_bool(Some(c.bit_128capable))),
                InfoRow::new("Multi core", fmt_bool(Some(c.multi_core))),
                InfoRow::new("Hardware thread", fmt_bool(Some(c.hardware_thread))),
                InfoRow::new("Execute protection", fmt_bool(Some(c.execute_protection))),
                InfoRow::new(
                    "Enhanced Virtualization",
                    fmt_bool(Some(c.enhanced_virtualization)),
                ),
                InfoRow::new(
                    "Power/performance control",
                    fmt_bool(Some(c.power_perfomance_control)),
                ),
                InfoRow::new("ARM64 SoC ID", fmt_bool(Some(c.arm_64soc_id))),
            ];

            container(
                column![
                    text("Processor characteristics").style(text::warning),
                    container(kv_info_table(rows)).style(container::rounded_box),
                ]
                .spacing(5),
            )
        }
    }
}

/*******************************************************
 *******************************************************/

fn kv_info_table<'a, V>(rows: Vec<InfoRow<V>>) -> Element<'a, Message>
where
    V: ToString + Clone + 'a,
{
    let columns = [
        table::column(hdr_name(fl!("hdr-param")), |row: InfoRow<V>| {
            text(row.param_header).wrapping(text::Wrapping::WordOrGlyph)
        })
        .width(Length::FillPortion(1)),
        table::column(hdr_name(fl!("hdr-value")), |row: InfoRow<V>| {
            text_fmt_val(row.value)
        })
        .width(Length::FillPortion(4)),
    ];

    table(columns, rows).padding(2).width(Length::Fill).into()
}
