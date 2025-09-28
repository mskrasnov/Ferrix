//! RAM page

use crate::{
    Message,
    pages::{InfoRow, Page, fmt_val, kv_info_table},
};
use ferrix_lib::ram::RAM;

use iced::widget::{center, column, container, scrollable, text};

pub fn ram_page<'a>(ram: &'a Option<RAM>) -> container::Container<'a, Message> {
    match ram {
        None => container(center(text("Загрузка данных..."))),
        Some(ram) => {
            let mut ram_data = column![Page::Memory.title()].spacing(5);
            let rows = vec![
                InfoRow::new("Памяти всего", fmt_val(ram.total.round(2))),
                InfoRow::new("Памяти свободно", fmt_val(ram.free.round(2))),
                InfoRow::new("Памяти доступно", fmt_val(ram.available.round(2))),
                InfoRow::new("Буферы", fmt_val(ram.buffers.round(2))),
                InfoRow::new("В кеше", fmt_val(ram.cached.round(2))),
                InfoRow::new("В кеше подкачки", fmt_val(ram.swap_cached.round(2))),
                InfoRow::new("Активно", fmt_val(ram.active.round(2))),
                InfoRow::new("Неактивно", fmt_val(ram.inactive.round(2))),
                InfoRow::new("Активно (анонимные)", fmt_val(ram.active_anon.round(2))),
                InfoRow::new("Неактивно (анонимные)", fmt_val(ram.inactive_anon.round(2))),
                InfoRow::new("Активно (файл)", fmt_val(ram.active_file.round(2))),
                InfoRow::new("Неактивно (файл)", fmt_val(ram.inactive_file.round(2))),
                InfoRow::new("Невосполнимо", fmt_val(ram.unevictable.round(2))),
                InfoRow::new("Блокировано", fmt_val(ram.mlocked.round(2))),
                InfoRow::new("Подкачки всего", fmt_val(ram.swap_total.round(2))),
                InfoRow::new("Подкачки свободно", fmt_val(ram.swap_free.round(2))),
                InfoRow::new("ZSwap всего", fmt_val(ram.zswap.round(2))),
                InfoRow::new("ZSwap'пировано", fmt_val(ram.zswapped.round(2))),
                InfoRow::new("Ожидают записи на диск", fmt_val(ram.dirty.round(2))),
                InfoRow::new("Пишутся на диск", fmt_val(ram.writeback.round(2))),
            ];

            ram_data = ram_data.push(container(kv_info_table(rows)).style(container::rounded_box));
            container(scrollable(ram_data))
        }
    }
}
