//! Page with information about installed Linux distro

use crate::{
    Message,
    pages::{InfoRow, Page, kv_info_table},
};
use ferrix_lib::sys::OsRelease;

use iced::widget::{center, column, container, scrollable, text};

pub fn distro_page<'a>(osrel: &'a Option<OsRelease>) -> container::Container<'a, Message> {
    match osrel {
        None => container(center(text("Загрузка данных..."))),
        Some(osrel) => {
            let mut os_data = column![Page::Distro.title()].spacing(5);
            let rows = vec![
                InfoRow::new("Название ОС", Some(osrel.name.clone())),
                InfoRow::new("Идентификатор", osrel.id.clone()),
                InfoRow::new("Дериватив от", osrel.id_like.clone()),
                InfoRow::new("Имя CPE", osrel.cpe_name.clone()),
                InfoRow::new("Редакция/вариант", osrel.variant.clone()),
                InfoRow::new("Версия", osrel.version.clone()),
                InfoRow::new("Кодовое имя", osrel.version_codename.clone()),
                InfoRow::new("ID сборки", osrel.build_id.clone()),
                InfoRow::new("ID образа", osrel.image_id.clone()),
                InfoRow::new("Версия образа", osrel.image_version.clone()),
                InfoRow::new("Домашняя страница", osrel.home_url.clone()),
                InfoRow::new("Документация", osrel.documentation_url.clone()),
                InfoRow::new("Поддержка", osrel.support_url.clone()),
                InfoRow::new("Багтрекер", osrel.bug_report_url.clone()),
                InfoRow::new(
                    "Политика конфиденциальности",
                    osrel.privacy_policy_url.clone(),
                ),
                InfoRow::new("Логотип", osrel.logo.clone()),
                InfoRow::new("Стандартное имя хоста", osrel.default_hostname.clone()),
                InfoRow::new("Уровень поддержки расширений", osrel.sysext_level.clone()),
            ];

            os_data = os_data.push(container(kv_info_table(rows)).style(container::rounded_box));
            container(scrollable(os_data))
        }
    }
}
