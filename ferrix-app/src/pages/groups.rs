//! Groups list page

use crate::{
    Message, fl,
    load_state::DataLoadingState,
    pages::{InfoRow, fmt_val, kv_info_table},
};
use ferrix_lib::sys::Groups;

use iced::widget::{column, container, scrollable, text};

pub fn groups_page<'a>(groups: &'a DataLoadingState<Groups>) -> container::Container<'a, Message> {
    match groups {
        DataLoadingState::Loaded(groups) => {
            let mut groups_list = column![].spacing(5);
            for grp in &groups.groups {
                let rows = vec![
                    InfoRow::new(fl!("groups-name"), Some(grp.name.clone())),
                    InfoRow::new(fl!("groups-id"), fmt_val(Some(grp.gid))),
                    InfoRow::new(fl!("groups-members"), Some(format!("{:?}", &grp.users))),
                ];
                let grp_view = column![
                    text(fl!("groups-group", group_no = grp.gid)).style(text::warning),
                    container(kv_info_table(rows)).style(container::rounded_box),
                ]
                .spacing(5);
                groups_list = groups_list.push(grp_view);
            }
            container(scrollable(groups_list))
        }
        DataLoadingState::Error(why) => super::error_page(why),
        DataLoadingState::Loading => super::loading_page(),
    }
}
