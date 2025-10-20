//! systemd services list

use crate::{Message, load_state::DataLoadingState, pages::hdr_name};
use ferrix_lib::init::{ActiveState, LoadState, ServiceInfo, SystemdServices, WorkState};

use iced::{
    Length,
    widget::{column, container, row, scrollable, table, text},
};

fn srv_table<'a>(rows: &'a [ServiceInfo]) -> table::Table<'a, Message> {
    let columns = [
        table::column(hdr_name("Имя"), |row: &'a ServiceInfo| {
            // If the window has a standard size, then some names and
            // descriptions of services will not fit within the limits
            // of one cell of the table, which will lead to an excessive
            // increase in the "Description" column and the almost
            // complete disappearance of the remaining columns. So we
            // change the minimum size of the two largest columns and
            // change the character wrapping logic./There are enough
            // words in the `text` widget so that everything fits,
            // regardless of the size of the window and the table cell.
            text(&row.name).wrapping(text::Wrapping::WordOrGlyph)
        })
        .width(Length::FillPortion(2)),
        table::column(hdr_name("Описание"), |row: &ServiceInfo| {
            text(&row.description).wrapping(text::Wrapping::WordOrGlyph)
        })
        .width(Length::FillPortion(3)),
        table::column(hdr_name("Загружен"), |row: &ServiceInfo| {
            text(format!("{}", row.load_state)).style(match row.load_state {
                LoadState::Loaded => text::success,
                LoadState::Stub | LoadState::Masked => text::warning,
                LoadState::NotFound => text::danger,
                _ => text::secondary,
            })
        }),
        table::column(hdr_name("Активен"), |row: &ServiceInfo| {
            text(format!("{}", row.active_state)).style(match row.active_state {
                ActiveState::Failed => text::danger,
                ActiveState::Deactivating => text::warning,
                ActiveState::Activating => text::primary,
                ActiveState::Active => text::success,
                _ => text::secondary,
            })
        }),
        table::column(hdr_name("Работает"), |row: &ServiceInfo| {
            text(format!("{}", row.work_state)).style(match row.work_state {
                WorkState::Active
                | WorkState::Running
                | WorkState::Mounted
                | WorkState::Plugged => text::success,
                WorkState::Exited | WorkState::Dead | WorkState::Failed => text::danger,
                WorkState::Mounting | WorkState::Listening | WorkState::Waiting => text::warning,
                _ => text::secondary,
            })
        }),
    ];

    table(columns, rows).padding(2).width(Length::Fill)
}

pub fn services_page<'a>(
    services: &'a DataLoadingState<SystemdServices>,
) -> container::Container<'a, Message> {
    match services {
        DataLoadingState::Loaded(services) => {
            let units = &services.units;
            let table = container(srv_table(units)).style(container::rounded_box);
            let warn_txt = {
                let hdr = text("Внимание:").style(text::warning);
                let body =
                    text("Увеличьте размер окна для более корректного отображения ряда строк!");

                row![hdr, body].spacing(5)
            };
            let services_count = text(format!("Всего сервисов: {}", units.len()));

            let layout = column![warn_txt, services_count, table,].spacing(5);
            container(scrollable(layout))
        }
        DataLoadingState::Error(why) => super::error_page(why),
        DataLoadingState::Loading => super::loading_page(),
    }
}
