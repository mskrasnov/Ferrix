//! Groups list page

use crate::{
    Message,
    pages::{InfoRow, fmt_val, kv_info_table},
};
use ferrix_lib::sys::Groups;

use iced::widget::{center, column, container, scrollable, text};

pub fn groups_page<'a>(groups: &'a Option<Groups>) -> container::Container<'a, Message> {
    match groups {
        None => container(center(text("Загрузка данных..."))),
        Some(groups) => {
            let mut groups_list = column![].spacing(5);
            for grp in &groups.groups {
                let rows = vec![
                    InfoRow::new("Имя группы", Some(grp.name.clone())),
                    InfoRow::new("Идентификатор группы", fmt_val(Some(grp.gid))),
                    InfoRow::new("Члены группы", Some(format!("{:?}", &grp.users))),
                ];
                let grp_view = column![
                    text(format!("Группа #{}", grp.gid)).style(text::warning),
                    container(kv_info_table(rows)).style(container::rounded_box),
                ]
                .spacing(5);
                groups_list = groups_list.push(grp_view);
            }
            container(scrollable(groups_list))
        }
    }
}
