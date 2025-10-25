//! DMI table viewer page

use crate::{
    dmi::DMIResult, pages::{fmt_bool, fmt_val, kv_info_table, InfoRow}, DataLoadingState, Message
};
use ferrix_lib::dmi::Baseboard;

use iced::widget::{column, container, scrollable, text};

pub fn chassis_page<'a>(dmi: &'a DataLoadingState<DMIResult>) -> container::Container<'a, Message> {
    match dmi {
        DataLoadingState::Loaded(dmi) => match dmi {
            DMIResult::Ok { data } => baseboard_table(&data.baseboard),
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

    container(scrollable(bb_view))
}
