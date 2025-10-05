//! RAM page

use crate::{
    Message,
    pages::{InfoRow, fmt_val, kv_info_table},
};
use ferrix_lib::ram::RAM;

use iced::widget::{center, column, container, scrollable, text};

pub fn ram_page<'a>(ram: &'a Option<RAM>) -> container::Container<'a, Message> {
    match ram {
        None => container(center(text("Загрузка данных..."))),
        Some(ram) => {
            let mut ram_data = column![].spacing(5);
            let rows = vec![
                InfoRow::new("Общий объём", fmt_val(ram.total.round(2))),
                InfoRow::new("Свободно", fmt_val(ram.free.round(2))),
                InfoRow::new("Доступно", fmt_val(ram.available.round(2))),
                InfoRow::new("Буферы", fmt_val(ram.buffers.round(2))),
                InfoRow::new("Кеш", fmt_val(ram.cached.round(2))),
                InfoRow::new("Кеш подкачки", fmt_val(ram.swap_cached.round(2))),
                InfoRow::new("Активная", fmt_val(ram.active.round(2))),
                InfoRow::new("Неактивная", fmt_val(ram.inactive.round(2))),
                InfoRow::new("Активная (анонимные)", fmt_val(ram.active_anon.round(2))),
                InfoRow::new("Неактивная (анонимные)", fmt_val(ram.inactive_anon.round(2))),
                InfoRow::new("Активная (файл)", fmt_val(ram.active_file.round(2))),
                InfoRow::new("Неактивная (файл)", fmt_val(ram.inactive_file.round(2))),
                InfoRow::new("Невыгружаемая", fmt_val(ram.unevictable.round(2))),
                InfoRow::new("Заблокированная", fmt_val(ram.mlocked.round(2))),
                InfoRow::new("Подкачки всего", fmt_val(ram.swap_total.round(2))),
                InfoRow::new("Подкачки свободно", fmt_val(ram.swap_free.round(2))),
                InfoRow::new("ZSwap всего", fmt_val(ram.zswap.round(2))),
                InfoRow::new("ZSwap'пировано", fmt_val(ram.zswapped.round(2))),
                InfoRow::new("Грязные страницы", fmt_val(ram.dirty.round(2))),
                InfoRow::new("Ожидание записи", fmt_val(ram.writeback.round(2))),
                InfoRow::new("Анонимные страницы", fmt_val(ram.anon_pages.round(2))),
                InfoRow::new("Отображённая", fmt_val(ram.mapped.round(2))),
                InfoRow::new("Разделяемая", fmt_val(ram.shmem.round(2))),
                InfoRow::new("Восстанавливаемая ядром", fmt_val(ram.kreclaimable.round(2))),
                InfoRow::new("slab", fmt_val(ram.slab.round(2))),
                InfoRow::new("Восстанавливаемый slab", fmt_val(ram.sreclaimable.round(2))),
                InfoRow::new("Невостанавливаемый slab", fmt_val(ram.sunreclaim.round(2))),
                InfoRow::new("Стек ядра", fmt_val(ram.kernel_stack.round(2))),
                InfoRow::new("Таблицы страниц", fmt_val(ram.page_tables.round(2))),
                InfoRow::new("Доп. таблицы страниц", fmt_val(ram.sec_page_tables.round(2))),
                InfoRow::new("Нестабильный NFS", fmt_val(ram.nfs_unstable.round(2))),
                InfoRow::new("Bounce буферы", fmt_val(ram.bounce.round(2))),
                InfoRow::new(
                    "Временные буферы (для FUSE)",
                    fmt_val(ram.writeback_tmp.round(2)),
                ),
                InfoRow::new("Можно выделить (max.)", fmt_val(ram.commit_limit.round(2))),
            ];

            ram_data = ram_data.push(container(kv_info_table(rows)).style(container::rounded_box));
            container(scrollable(ram_data))
        }
    }
}
