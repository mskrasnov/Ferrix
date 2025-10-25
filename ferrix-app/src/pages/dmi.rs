//! DMI table viewer page

#![allow(unused)]

use crate::{
    DataLoadingState, Message, fl,
    pages::{InfoRow, fmt_bool, fmt_val, fmt_vec, kv_info_table},
};
use ferrix_lib::{cpu::Processors, dmi::Chassis};

use iced::widget::{column, container, scrollable, text};

pub fn chassis_page<'a>(
    chassis: &'a DataLoadingState<Chassis>,
) -> container::Container<'a, Message> {
    match chassis {
        DataLoadingState::Loaded(chassis) => {
            let rows = vec![
                InfoRow::new("Производитель", chassis.manufacturer.clone()),
                InfoRow::new("Версия", chassis.version.clone()),
                InfoRow::new("SKU number", chassis.sku_number.clone()),
            ];

            let chassis_view = column![
                text("Корпус/шасси").style(text::warning),
                container(kv_info_table(rows)).style(container::rounded_box),
            ]
            .spacing(5);

            container(scrollable(chassis_view))
        }
        DataLoadingState::Error(why) => super::error_page(why),
        DataLoadingState::Loading => super::loading_page(),
    }
}
