//! Kernel page

use crate::{
    Message, fl,
    load_state::DataLoadingState,
    pages::{InfoRow, fmt_val, hdr_name, kv_info_table, text_fmt_val},
};
use ferrix_lib::sys::{KModules, Kernel, Module};

use iced::{
    Length,
    widget::{column, container, scrollable, table, text},
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
        table::column(hdr_name(fl!("kmod-name")), |row: &'a Module| {
            text(&row.name).wrapping(text::Wrapping::WordOrGlyph)
        })
        .width(Length::FillPortion(1)),
        table::column(hdr_name(fl!("kmod-size")), |row: &'a Module| {
            text_fmt_val(row.size.round(2))
        }),
        table::column(hdr_name(fl!("kmod-instances")), |row: &'a Module| {
            text(row.instances)
        }),
        table::column(hdr_name(fl!("kmod-depends")), |row: &'a Module| {
            text(if &row.dependencies == "-" {
                ""
            } else {
                &row.dependencies
            })
            .wrapping(text::Wrapping::WordOrGlyph)
        })
        .width(Length::FillPortion(3)),
        table::column(hdr_name(fl!("kmod-state")), |row: &'a Module| {
            text(&row.state).style(if &row.state == "Live" {
                text::success
            } else {
                text::default
            })
        }),
        table::column(hdr_name(fl!("kmod-addrs")), |row: &'a Module| {
            text(&row.memory_addrs)
        }),
    ];

    table(columns, rows).padding(2).width(Length::Fill)
}

pub fn kernel_page<'a>(
    kernel_data: &'a DataLoadingState<KernelData>,
) -> container::Container<'a, Message> {
    match kernel_data {
        DataLoadingState::Loaded(kernel_data) => {
            let kern = &kernel_data.kernel;
            let summary_rows = vec![
                InfoRow::new(fl!("kernel-summary"), kern.uname.clone()),
                InfoRow::new(fl!("kernel-cmdline"), kern.cmdline.clone()),
                InfoRow::new(fl!("kernel-arch"), kern.arch.clone()),
                InfoRow::new(fl!("kernel-version"), kern.version.clone()),
                InfoRow::new(fl!("kernel-build"), kern.build_info.clone()),
                InfoRow::new(fl!("kernel-pid-max"), fmt_val(Some(kern.pid_max))),
                InfoRow::new(fl!("kernel-threads-max"), fmt_val(Some(kern.threads_max))),
                InfoRow::new(fl!("kernel-user-evs"), fmt_val(kern.user_events_max)),
                InfoRow::new(fl!("kernel-avail-enthropy"), fmt_val(kern.enthropy_avail)),
            ];

            let kern_summary_data =
                column![container(kv_info_table(summary_rows)).style(container::rounded_box)]
                    .spacing(5);

            let kern_modules =
                container(modules_table(&kernel_data.mods.modules)).style(container::rounded_box);

            let layout = column![
                text(fl!("kernel-summary-hdr")).style(text::warning),
                kern_summary_data,
                text(fl!("kernel-mods-hdr")).style(text::warning),
                kern_modules,
            ]
            .spacing(5);

            container(scrollable(layout))
        }
        DataLoadingState::Error(why) => super::error_page(why),
        DataLoadingState::Loading => super::loading_page(),
    }
}
