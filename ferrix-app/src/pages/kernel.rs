//! Kernel page

use crate::{
    Message,
    pages::{InfoRow, fmt_val, hdr_name, kv_info_table, text_fmt_val},
};
use ferrix_lib::sys::{KModules, Kernel, Module};

use iced::{
    Length,
    widget::{center, column, container, scrollable, table, text},
};

#[derive(Debug, Clone)]
pub struct KernelData {
    pub kernel: Kernel,
    pub mods: KModules,
}

impl KernelData {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            kernel: Kernel::new()?,
            mods: KModules::new()?,
        })
    }
}

fn modules_table<'a>(rows: &'a [Module]) -> table::Table<'a, Message> {
    let columns = [
        table::column(hdr_name("Имя"), |row: &'a Module| {
            text(&row.name).wrapping(text::Wrapping::WordOrGlyph)
        })
        .width(Length::FillPortion(1)),
        table::column(hdr_name("Размер"), |row: &'a Module| {
            text_fmt_val(row.size.round(2))
        }),
        table::column(hdr_name("Экз."), |row: &'a Module| text(row.instances)),
        table::column(hdr_name("Зависимости"), |row: &'a Module| {
            text(if &row.dependencies == "-" {
                ""
            } else {
                &row.dependencies
            })
            .wrapping(text::Wrapping::WordOrGlyph)
        })
        .width(Length::FillPortion(3)),
        table::column(hdr_name("Сост."), |row: &'a Module| {
            text(&row.state).style(if &row.state == "Live" {
                text::success
            } else {
                text::default
            })
        }),
        table::column(hdr_name("Адреса"), |row: &'a Module| {
            text(&row.memory_addrs)
        }),
    ];

    table(columns, rows).padding(2).width(Length::Fill)
}

pub fn kernel_page<'a>(
    kernel: &'a Option<Kernel>,
    modules: &'a Option<KModules>,
) -> container::Container<'a, Message> {
    match kernel {
        None => container(center(text("Загрузка данных..."))),
        Some(kern) => {
            let summary_rows = vec![
                InfoRow::new("Summary", kern.uname.clone()),
                InfoRow::new("Командная строка", kern.cmdline.clone()),
                InfoRow::new("Архитектура", kern.arch.clone()),
                InfoRow::new("Версия", kern.version.clone()),
                InfoRow::new("Сборка", kern.build_info.clone()),
                InfoRow::new("Макс. число процессов", fmt_val(Some(kern.pid_max))),
                InfoRow::new("Макс. число потоков", fmt_val(Some(kern.threads_max))),
                InfoRow::new("Макс. число user events", fmt_val(kern.user_events_max)),
                InfoRow::new("Доступная энтропия", fmt_val(kern.enthropy_avail)),
            ];

            let kern_summary_data =
                column![container(kv_info_table(summary_rows)).style(container::rounded_box)]
                    .spacing(5);

            let kern_modules = match modules {
                None => container(text("Загрузка информации о модулях ядра...")),
                Some(kmods) => container(modules_table(&kmods.modules)),
            }
            .style(container::rounded_box);

            let layout = column![
                text("Общая информация").style(text::warning),
                kern_summary_data,
                text("Загруженные модули ядра").style(text::warning),
                kern_modules,
            ]
            .spacing(5);

            container(scrollable(layout))
        }
    }
}
