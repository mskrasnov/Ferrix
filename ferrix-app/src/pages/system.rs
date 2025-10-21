//! Summary information about system

use crate::{
    Message,
    load_state::DataLoadingState,
    pages::{InfoRow, kv_info_table},
};

use ferrix_lib::sys::{LoadAVG, Uptime};
use iced::widget::{container, scrollable};

pub fn system_page<'a>(
    system: &'a DataLoadingState<crate::System>,
) -> container::Container<'a, Message> {
    match system {
        DataLoadingState::Loaded(sys) => {
            let rows = vec![
                InfoRow::new("Имя хоста", sys.hostname.clone()),
                InfoRow::new(
                    "Средняя нагрузка",
                    Some(match &sys.loadavg {
                        Some(loadavg) => string_loadavg(loadavg),
                        None => format!("???"),
                    }),
                ),
                InfoRow::new(
                    "Время работы",
                    Some(match &sys.uptime {
                        Some(uptime) => string_uptime(uptime),
                        None => format!("???"),
                    }),
                ),
            ];

            let sys_table = container(kv_info_table(rows)).style(container::rounded_box);

            container(scrollable(sys_table))
        }
        DataLoadingState::Error(why) => super::error_page(why),
        DataLoadingState::Loading => super::loading_page(),
    }
}

fn string_loadavg(lavg: &LoadAVG) -> String {
    format!("1min: {}\n5min: {}\n15min: {}", lavg.0, lavg.1, lavg.2)
}

fn string_uptime(uptime: &Uptime) -> String {
    format!(
        "работы: {}, простоя: {}",
        string_time(uptime.0),
        string_time(uptime.1),
    )
}

fn string_time(time: f32) -> String {
    let hours = (time / 3600.) as u32;
    let remain_secs_after_hours = time % 3600.;
    let mins = (remain_secs_after_hours / 60.) as u32;
    let secs = (remain_secs_after_hours % 60.) as u32;

    format!(
        "{}{}:{}{}:{}{}",
        if hours < 10 { "0" } else { "" },
        hours,
        if mins < 10 { "0" } else { "" },
        mins,
        if secs < 10 { "0" } else { "" },
        secs,
    )
}
