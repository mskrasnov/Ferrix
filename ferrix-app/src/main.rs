use anyhow::Result;
use ferrix_lib::{
    cpu::Processors,
    init::{Connection, SystemdServices},
    ram::RAM,
    sys::{Groups, LoadAVG, OsRelease, Uptime, Users, get_hostname},
};
use iced::{
    Alignment::Center,
    Element, Length, Size, Subscription, Task, Theme, time,
    widget::{column, container, row, scrollable, text},
    window::Settings,
};
use std::time::Duration;

pub mod i18n;
pub mod icons;
pub mod load_state;
pub mod modals;
pub mod pages;
pub mod styles;
pub mod utils;
pub mod widgets;

use load_state::DataLoadingState;
use pages::*;

use crate::widgets::{icon_button, sidebar_button};

const APP_LOGO: &[u8] = include_bytes!("../data/icons/hicolor/scalable/apps/win_logo.png");

pub fn main() -> iced::Result {
    iced::application(Ferrix::default, Ferrix::update, Ferrix::view)
        .settings(iced::Settings {
            default_text_size: iced::Pixels(12.),
            ..Default::default()
        })
        .window(Settings {
            icon: Some(iced::window::icon::from_file_data(APP_LOGO, None).unwrap()),
            min_size: Some(Size {
                width: 790.,
                height: 480.,
            }),
            ..Default::default()
        })
        .window_size((790., 480.))
        .antialiasing(true)
        .subscription(Ferrix::subscription)
        .theme(Ferrix::theme)
        .title("Ferrix")
        .run()
}

#[derive(Debug, Clone)]
pub enum Message {
    GetCPUData,
    CPUDataReceived(DataLoadingState<Processors>),

    GetRAMData,
    RAMDataReceived(DataLoadingState<RAM>),

    GetOsReleaseData,
    OsReleaseDataReceived(DataLoadingState<OsRelease>),

    GetKernelData,
    KernelDataReceived(DataLoadingState<KernelData>),

    GetUsersData,
    UsersDataReceived(DataLoadingState<Users>),

    GetGroupsData,
    GroupsDataReceived(DataLoadingState<Groups>),

    GetSystemdServices,
    SystemdServicesReceived(DataLoadingState<SystemdServices>),

    GetSystemData,
    SystemDataReceived(DataLoadingState<System>),

    Dummy,
    ChangeTheme(Theme),
    SelectPage(Page),
    ChangeUpdatePeriod(u8),
    LinkButtonPressed(String),
}

#[derive(Debug)]
pub struct Ferrix {
    pub current_page: Page,
    pub proc_data: DataLoadingState<Processors>,
    pub ram_data: DataLoadingState<RAM>,
    pub osrel_data: DataLoadingState<OsRelease>,
    pub info_kernel: DataLoadingState<KernelData>,
    pub users_list: DataLoadingState<Users>,
    pub groups_list: DataLoadingState<Groups>,
    pub sysd_services_list: DataLoadingState<SystemdServices>,
    pub system: DataLoadingState<System>,
    pub settings: FXSettings,
}

impl Default for Ferrix {
    fn default() -> Self {
        Self {
            current_page: Page::default(),
            proc_data: DataLoadingState::Loading,
            ram_data: DataLoadingState::Loading,
            osrel_data: DataLoadingState::Loading,
            info_kernel: DataLoadingState::Loading,
            users_list: DataLoadingState::Loading,
            groups_list: DataLoadingState::Loading,
            sysd_services_list: DataLoadingState::Loading,
            system: DataLoadingState::Loading,
            settings: FXSettings::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct System {
    pub hostname: Option<String>,
    pub loadavg: Option<LoadAVG>,
    pub uptime: Option<Uptime>,
}

impl System {
    pub fn new() -> Result<Self> {
        Ok(Self {
            hostname: get_hostname(),
            loadavg: Some(LoadAVG::new()?),
            uptime: Some(Uptime::new()?),
        })
    }
}

#[derive(Debug, Clone)]
pub struct FXSettings {
    pub update_period: u8,
    pub theme: Theme,
}

impl Default for FXSettings {
    fn default() -> Self {
        Self {
            update_period: 1,
            theme: Theme::GruvboxDark,
        }
    }
}

impl Ferrix {
    fn theme(&self) -> Theme {
        self.settings.theme.clone()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::CPUDataReceived(state) => {
                self.proc_data = state;
                Task::none()
            }
            Message::GetCPUData => Task::perform(
                async move {
                    let proc = Processors::new();
                    match proc {
                        Ok(proc) => DataLoadingState::Loaded(proc),
                        Err(why) => DataLoadingState::Error(why.to_string()),
                    }
                },
                |val| Message::CPUDataReceived(val),
            ),
            Message::RAMDataReceived(state) => {
                self.ram_data = state;
                Task::none()
            }
            Message::GetRAMData => Task::perform(
                async move {
                    let ram = RAM::new();
                    match ram {
                        Ok(ram) => DataLoadingState::Loaded(ram),
                        Err(why) => DataLoadingState::Error(why.to_string()),
                    }
                },
                |val| Message::RAMDataReceived(val),
            ),
            Message::OsReleaseDataReceived(state) => {
                self.osrel_data = state;
                Task::none()
            }
            Message::GetOsReleaseData => Task::perform(
                async move {
                    let osrel = OsRelease::new();
                    match osrel {
                        Ok(osrel) => DataLoadingState::Loaded(osrel),
                        Err(why) => DataLoadingState::Error(why.to_string()),
                    }
                },
                |val| Message::OsReleaseDataReceived(val),
            ),
            Message::KernelDataReceived(state) => {
                self.info_kernel = state;
                Task::none()
            }
            Message::GetKernelData => Task::perform(
                async move {
                    let kern = KernelData::new();
                    match kern {
                        Ok(mut kern) => {
                            kern.mods.modules.sort_by_key(|md| md.name.clone());
                            DataLoadingState::Loaded(kern)
                        }
                        Err(why) => DataLoadingState::Error(why.to_string()),
                    }
                },
                |val| Message::KernelDataReceived(val),
            ),
            Message::UsersDataReceived(state) => {
                self.users_list = state;
                Task::none()
            }
            Message::GetUsersData => Task::perform(
                async move {
                    let users = Users::new();
                    match users {
                        Ok(mut users) => {
                            users.users.sort_by_key(|usr| usr.uid);
                            DataLoadingState::Loaded(users)
                        }
                        Err(why) => DataLoadingState::Error(why.to_string()),
                    }
                },
                |val| Message::UsersDataReceived(val),
            ),
            Message::GroupsDataReceived(state) => {
                self.groups_list = state;
                Task::none()
            }
            Message::GetGroupsData => Task::perform(
                async move {
                    let groups = Groups::new();
                    match groups {
                        Ok(mut groups) => {
                            groups.groups.sort_by_key(|grp| grp.gid);
                            DataLoadingState::Loaded(groups)
                        }
                        Err(why) => DataLoadingState::Error(why.to_string()),
                    }
                },
                |val| Message::GroupsDataReceived(val),
            ),
            Message::SystemdServicesReceived(state) => {
                self.sysd_services_list = state;
                Task::none()
            }
            Message::GetSystemdServices => Task::perform(
                async move {
                    let conn = Connection::session().await;
                    if let Err(why) = conn {
                        return DataLoadingState::Error(why.to_string());
                    }
                    let conn = conn.unwrap();

                    let srv_list = SystemdServices::new_from_connection(&conn).await;
                    match srv_list {
                        Ok(srv_list) => DataLoadingState::Loaded(srv_list),
                        Err(why) => DataLoadingState::Error(why.to_string()),
                    }
                },
                |val| Message::SystemdServicesReceived(val),
            ),
            Message::SystemDataReceived(state) => {
                self.system = state;
                Task::none()
            }
            Message::GetSystemData => Task::perform(
                async move {
                    let sys = System::new();
                    match sys {
                        Ok(sys) => DataLoadingState::Loaded(sys),
                        Err(why) => DataLoadingState::Error(why.to_string()),
                    }
                },
                |val| Message::SystemDataReceived(val),
            ),
            Message::SelectPage(page) => {
                self.current_page = page;
                Task::none()
            }
            Message::ChangeTheme(theme) => {
                self.settings.theme = theme;
                Task::none()
            }
            Message::ChangeUpdatePeriod(period) => {
                self.settings.update_period = period;
                Task::none()
            }
            Message::LinkButtonPressed(url) => {
                // TODO: add error handling
                let _ = utils::xdg_open(url);
                Task::none()
            }
            _ => Task::none(),
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        let mut scripts = vec![
            time::every(Duration::from_secs(self.settings.update_period as u64))
                .map(|_| Message::GetCPUData),
            time::every(Duration::from_secs(self.settings.update_period as u64))
                .map(|_| Message::GetRAMData),
        ];

        if self.osrel_data.is_none()
            && (self.current_page == Page::Distro || self.current_page == Page::Dashboard)
        {
            scripts.push(time::every(Duration::from_millis(50)).map(|_| Message::GetOsReleaseData));
        }

        if self.info_kernel.is_none() && self.current_page == Page::Kernel {
            scripts.push(time::every(Duration::from_millis(50)).map(|_| Message::GetKernelData));
        }

        if self.users_list.is_none() && self.current_page == Page::Users {
            scripts.push(time::every(Duration::from_millis(10)).map(|_| Message::GetUsersData));
        }

        if self.system.is_none() {
            scripts.push(time::every(Duration::from_millis(50)).map(|_| Message::GetSystemData));
        }

        if self.groups_list.is_none() && self.current_page == Page::Groups {
            scripts.push(time::every(Duration::from_millis(10)).map(|_| Message::GetGroupsData));
        }

        if self.sysd_services_list.is_none() && self.current_page == Page::SystemManager {
            scripts
                .push(time::every(Duration::from_millis(10)).map(|_| Message::GetSystemdServices));
        } else if self.sysd_services_list.is_some() && self.current_page == Page::SystemManager {
            scripts.push(
                time::every(Duration::from_secs(
                    self.settings.update_period as u64 * 10u64,
                ))
                .map(|_| Message::GetSystemdServices),
            );
        }

        if self.system.is_none() && self.current_page == Page::SystemMisc {
            scripts.push(time::every(Duration::from_millis(10)).map(|_| Message::GetSystemData));
        } else if self.system.is_some() && self.current_page == Page::SystemMisc {
            scripts.push(
                time::every(Duration::from_secs(self.settings.update_period as u64))
                    .map(|_| Message::GetSystemData),
            );
        }

        Subscription::batch(scripts)
    }

    fn view<'a>(&'a self) -> Element<'a, Message> {
        row![sidebar(self.current_page), self.current_page.page(&self)]
            .spacing(5)
            .padding(5)
            .into()
    }
}

fn sidebar<'a>(cur_page: Page) -> container::Container<'a, Message> {
    let buttons_bar = row![
        icon_button("export", fl!("sidebar-export")).on_press(Message::Dummy),
        icon_button("settings", fl!("sidebar-settings")).on_press(Message::SelectPage(Page::Settings)),
        icon_button("about", fl!("sidebar-about")).on_press(Message::SelectPage(Page::About)),
    ]
    .spacing(2)
    .align_y(Center);

    let pages_bar = column![
        text(fl!("sidebar-hardware")).style(text::secondary),
        sidebar_button(Page::Dashboard, cur_page),
        sidebar_button(Page::Processors, cur_page),
        sidebar_button(Page::Memory, cur_page),
        sidebar_button(Page::Storage, cur_page),
        sidebar_button(Page::DMI, cur_page),
        sidebar_button(Page::Battery, cur_page),
        sidebar_button(Page::Screen, cur_page),
        text(fl!("sidebar-admin")).style(text::secondary),
        sidebar_button(Page::Distro, cur_page),
        sidebar_button(Page::Users, cur_page),
        sidebar_button(Page::Groups, cur_page),
        sidebar_button(Page::SystemManager, cur_page),
        sidebar_button(Page::Software, cur_page),
        sidebar_button(Page::Environment, cur_page),
        sidebar_button(Page::Sensors, cur_page),
        text(fl!("sidebar-system")).style(text::secondary),
        sidebar_button(Page::Kernel, cur_page),
        sidebar_button(Page::KModules, cur_page),
        sidebar_button(Page::Development, cur_page),
        sidebar_button(Page::SystemMisc, cur_page),
        text(fl!("sidebar-manage")).style(text::secondary),
        sidebar_button(Page::Settings, cur_page),
        sidebar_button(Page::About, cur_page),
    ]
    .spacing(5);

    container(column![buttons_bar, scrollable(pages_bar)].spacing(5))
        .padding(5)
        .style(container::bordered_box)
        .height(Length::Fill)
}
