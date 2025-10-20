//! CPU page

use crate::{
    DataLoadingState, Message,
    pages::{InfoRow, fmt_bool, fmt_val, fmt_vec, kv_info_table},
};
use ferrix_lib::cpu::Processors;

use iced::widget::{column, container, scrollable, text};

pub fn proc_page<'a>(
    processors: &'a DataLoadingState<Processors>,
) -> container::Container<'a, Message> {
    match processors {
        DataLoadingState::Loaded(proc) => {
            let mut proc_list = column![].spacing(5);
            for proc in &proc.entries {
                let rows = vec![
                    InfoRow::new("Производитель", proc.vendor_id.clone()),
                    InfoRow::new("Семейство", fmt_val(proc.cpu_family)),
                    InfoRow::new("Модель", proc.model_name.clone()),
                    InfoRow::new("Stepping", fmt_val(proc.stepping)),
                    InfoRow::new("Микрокод", proc.microcode.clone()),
                    InfoRow::new("Частота", fmt_val(proc.cpu_mhz)),
                    InfoRow::new("Размер L3 кеша", fmt_val(proc.cache_size)),
                    InfoRow::new("Физический ID", fmt_val(proc.physical_id)),
                    InfoRow::new("Siblings", fmt_val(proc.siblings)),
                    InfoRow::new("ID ядра", fmt_val(proc.core_id)),
                    InfoRow::new("Число ядер", fmt_val(proc.cpu_cores)),
                    InfoRow::new("APIC ID", fmt_val(proc.apicid)),
                    InfoRow::new("Initial APIC ID", fmt_val(proc.initial_apicid)),
                    InfoRow::new("FPU", fmt_bool(proc.fpu)),
                    InfoRow::new("FPU Exception", fmt_bool(proc.fpu_exception)),
                    InfoRow::new("CPUID Level", fmt_val(proc.cpuid_level)),
                    InfoRow::new("WP", fmt_bool(proc.wp)),
                    InfoRow::new("Флаги", fmt_vec(&proc.flags)),
                    InfoRow::new("Баги", fmt_vec(&proc.bugs)),
                    InfoRow::new("BogoMIPS", fmt_val(proc.bogomips)),
                    InfoRow::new("Размер clflush", fmt_val(proc.clflush_size)),
                    InfoRow::new("Выравнивание кеша", fmt_val(proc.cache_alignment)),
                    InfoRow::new("Размер адресов", proc.address_sizes.clone()),
                    InfoRow::new("Управление питанием", proc.power_management.clone()),
                ];

                let proc_view = column![
                    text(format!("Процессор #{}", proc.processor.unwrap_or(0)))
                        .style(text::warning),
                    container(kv_info_table(rows)).style(container::rounded_box),
                ]
                .spacing(5);
                proc_list = proc_list.push(proc_view);
            }
            container(scrollable(proc_list))
        }
        DataLoadingState::Error(why) => super::error_page(why),
        DataLoadingState::Loading => super::loading_page(),
    }
}
