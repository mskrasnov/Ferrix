//! CPU page

use crate::{
    DataLoadingState, Message, fl,
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
                    InfoRow::new(fl!("cpu-vendor"), proc.vendor_id.clone()),
                    InfoRow::new(fl!("cpu-family"), fmt_val(proc.cpu_family)),
                    InfoRow::new(fl!("cpu-model"), proc.model_name.clone()),
                    InfoRow::new(fl!("cpu-stepping"), fmt_val(proc.stepping)),
                    InfoRow::new(fl!("cpu-microcode"), proc.microcode.clone()),
                    InfoRow::new(fl!("cpu-freq"), fmt_val(proc.cpu_mhz)),
                    InfoRow::new(fl!("cpu-cache"), fmt_val(proc.cache_size)),
                    InfoRow::new(fl!("cpu-physical-id"), fmt_val(proc.physical_id)),
                    InfoRow::new(fl!("cpu-siblings"), fmt_val(proc.siblings)),
                    InfoRow::new(fl!("cpu-core-id"), fmt_val(proc.core_id)),
                    InfoRow::new(fl!("cpu-cpu-cores"), fmt_val(proc.cpu_cores)),
                    InfoRow::new(fl!("cpu-apicid"), fmt_val(proc.apicid)),
                    InfoRow::new(fl!("cpu-iapicid"), fmt_val(proc.initial_apicid)),
                    InfoRow::new(fl!("cpu-fpu"), fmt_bool(proc.fpu)),
                    InfoRow::new(fl!("cpu-fpu-e"), fmt_bool(proc.fpu_exception)),
                    InfoRow::new(fl!("cpu-cpuid-lvl"), fmt_val(proc.cpuid_level)),
                    InfoRow::new(fl!("cpu-wp"), fmt_bool(proc.wp)),
                    InfoRow::new(fl!("cpu-flags"), fmt_vec(&proc.flags)),
                    InfoRow::new(fl!("cpu-bugs"), fmt_vec(&proc.bugs)),
                    InfoRow::new(fl!("cpu-bogomips"), fmt_val(proc.bogomips)),
                    InfoRow::new(fl!("cpu-clflush"), fmt_val(proc.clflush_size)),
                    InfoRow::new(fl!("cpu-cache-align"), fmt_val(proc.cache_alignment)),
                    InfoRow::new(fl!("cpu-address-size"), proc.address_sizes.clone()),
                    InfoRow::new(fl!("cpu-power"), proc.power_management.clone()),
                ];

                let proc_view = column![
                    text(fl!(
                        "cpu-processor_no",
                        proc_no = proc.processor.unwrap_or(0)
                    ))
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
