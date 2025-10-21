//! Page with information about installed Linux distro

use crate::{
    Message, fl,
    load_state::DataLoadingState,
    pages::{InfoRow, kv_info_table},
};
use ferrix_lib::sys::OsRelease;

use iced::widget::{column, container, scrollable};

pub fn distro_page<'a>(
    osrel: &'a DataLoadingState<OsRelease>,
) -> container::Container<'a, Message> {
    match osrel {
        DataLoadingState::Loaded(osrel) => {
            let mut os_data = column![].spacing(5);
            let rows = vec![
                InfoRow::new(fl!("distro-name"), Some(osrel.name.clone())),
                InfoRow::new(fl!("distro-id"), osrel.id.clone()),
                InfoRow::new(fl!("distro-like"), osrel.id_like.clone()),
                InfoRow::new(fl!("distro-cpe"), osrel.cpe_name.clone()),
                InfoRow::new(fl!("distro-variant"), osrel.variant.clone()),
                InfoRow::new(fl!("distro-version"), osrel.version.clone()),
                InfoRow::new(fl!("distro-codename"), osrel.version_codename.clone()),
                InfoRow::new(fl!("distro-build-id"), osrel.build_id.clone()),
                InfoRow::new(fl!("distro-image-id"), osrel.image_id.clone()),
                InfoRow::new(fl!("distro-image-ver"), osrel.image_version.clone()),
                InfoRow::new(fl!("distro-homepage"), osrel.home_url.clone()),
                InfoRow::new(fl!("distro-docs"), osrel.documentation_url.clone()),
                InfoRow::new(fl!("distro-support"), osrel.support_url.clone()),
                InfoRow::new(fl!("distro-bugtracker"), osrel.bug_report_url.clone()),
                InfoRow::new(
                    fl!("distro-privacy-policy"),
                    osrel.privacy_policy_url.clone(),
                ),
                InfoRow::new(fl!("distro-logo"), osrel.logo.clone()),
                InfoRow::new(fl!("distro-def-host"), osrel.default_hostname.clone()),
                InfoRow::new(fl!("distro-sysext-lvl"), osrel.sysext_level.clone()),
            ];

            os_data = os_data.push(container(kv_info_table(rows)).style(container::rounded_box));
            container(scrollable(os_data))
        }
        DataLoadingState::Error(why) => super::error_page(why),
        DataLoadingState::Loading => super::loading_page(),
    }
}
