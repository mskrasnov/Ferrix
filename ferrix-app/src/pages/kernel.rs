//! Kernel page

use crate::{
    Message,
    pages::{InfoRow, fmt_val, kv_info_table},
};
use ferrix_lib::sys::Kernel;

use iced::widget::{center, column, container, scrollable, text};

pub fn kernel_page<'a>(kernel: &'a Option<Kernel>) -> container::Container<'a, Message> {
    match kernel {
        None => container(center(text("Загрузка данных..."))),
        Some(kern) => {
            let rows = vec![
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
            let kern_data =
                column![container(kv_info_table(rows)).style(container::rounded_box)].spacing(5);
            container(scrollable(kern_data))
        }
    }
}
