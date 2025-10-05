/*
 * main.rs
 * styles/
 * widgets/
 * modals/
 */

use ferrix_lib::{
    cpu::Processors,
    ram::RAM,
    sys::{Groups, Kernel, OsRelease, Users},
};
use iced::{
    Alignment::Center,
    Element, Length, Size, Subscription, Task, Theme, time,
    widget::{column, container, row, scrollable, text},
    window::Settings,
};
use std::time::Duration;

pub mod modals;
pub mod pages;
pub mod styles;
pub mod widgets;

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
    CPUDataReceived((Option<Processors>, Option<String>)),

    GetRAMData,
    RAMDataReceived((Option<RAM>, Option<String>)),

    GetOsReleaseData,
    OsReleaseDataReceived((Option<OsRelease>, Option<String>)),

    GetKernelData,
    KernelDataReceived((Option<Kernel>, Option<String>)),

    GetUsersData,
    UsersDataReceived((Option<Users>, Option<String>)),

    GetGroupsData,
    GroupsDataReceived((Option<Groups>, Option<String>)),

    Dummy,
    ChangeTheme(Theme),
    SelectPage(Page),
    ChangeUpdatePeriod(u8),
}

#[derive(Debug)]
pub struct Ferrix {
    pub current_page: Page,
    pub proc_data: Option<Processors>,
    pub ram_data: Option<RAM>,
    pub osrel_data: Option<OsRelease>,
    pub info_kernel: Option<Kernel>,
    pub users_list: Option<Users>,
    pub groups_list: Option<Groups>,
    pub settings: FXSettings,
}

impl Default for Ferrix {
    fn default() -> Self {
        Self {
            current_page: Page::default(),
            proc_data: None,
            ram_data: None,
            osrel_data: None,
            info_kernel: None,
            users_list: None,
            groups_list: None,
            settings: FXSettings::default(),
        }
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
            Message::CPUDataReceived((val, err)) => {
                if let Some(val) = val {
                    // dbg!(val);
                    self.proc_data = Some(val);
                } else {
                    if let Some(err) = err {
                        eprintln!("{err}");
                    }
                }
                Task::none()
            }
            Message::GetCPUData => Task::perform(
                async move {
                    let proc = Processors::new();
                    match proc {
                        Ok(proc) => (Some(proc), None),
                        Err(why) => (None, Some(why.to_string())),
                    }
                },
                |val| Message::CPUDataReceived(val),
            ),
            Message::RAMDataReceived((val, err)) => {
                if let Some(val) = val {
                    self.ram_data = Some(val);
                } else {
                    if let Some(err) = err {
                        eprintln!("{err}");
                    }
                }
                Task::none()
            }
            Message::GetRAMData => Task::perform(
                async move {
                    let ram = RAM::new();
                    match ram {
                        Ok(ram) => (Some(ram), None),
                        Err(why) => (None, Some(why.to_string())),
                    }
                },
                |val| Message::RAMDataReceived(val),
            ),
            Message::OsReleaseDataReceived((val, err)) => {
                if let Some(val) = val {
                    self.osrel_data = Some(val);
                } else {
                    if let Some(err) = err {
                        eprintln!("{err}");
                    }
                }
                Task::none()
            }
            Message::GetOsReleaseData => Task::perform(
                async move {
                    let osrel = OsRelease::new();
                    match osrel {
                        Ok(osrel) => (Some(osrel), None),
                        Err(why) => (None, Some(why.to_string())),
                    }
                },
                |val| Message::OsReleaseDataReceived(val),
            ),
            Message::KernelDataReceived((val, err)) => {
                if let Some(val) = val {
                    self.info_kernel = Some(val);
                } else {
                    if let Some(err) = err {
                        eprintln!("{err}");
                    }
                }
                Task::none()
            }
            Message::GetKernelData => Task::perform(
                async move {
                    let kern = Kernel::new();
                    match kern {
                        Ok(kern) => (Some(kern), None),
                        Err(why) => (None, Some(why.to_string())),
                    }
                },
                |val| Message::KernelDataReceived(val),
            ),
            Message::UsersDataReceived((val, err)) => {
                if let Some(val) = val {
                    self.users_list = Some(val);
                } else {
                    if let Some(err) = err {
                        eprintln!("{err}");
                    }
                }
                Task::none()
            }
            Message::GetUsersData => Task::perform(
                async move {
                    let users = Users::new();
                    match users {
                        Ok(mut users) => {
                            users.users.sort_by_key(|usr| usr.uid);
                            (Some(users), None)
                        }
                        Err(why) => (None, Some(why.to_string())),
                    }
                },
                |val| Message::UsersDataReceived(val),
            ),
            Message::GroupsDataReceived((val, err)) => {
                if let Some(val) = val {
                    self.groups_list = Some(val);
                } else {
                    if let Some(err) = err {
                        eprintln!("{err}");
                    }
                }
                Task::none()
            }
            Message::GetGroupsData => Task::perform(
                async move {
                    let groups = Groups::new();
                    match groups {
                        Ok(mut groups) => {
                            groups.groups.sort_by_key(|grp| grp.gid);
                            (Some(groups), None)
                        }
                        Err(why) => (None, Some(why.to_string())),
                    }
                },
                |val| Message::GroupsDataReceived(val),
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
            scripts.push(time::every(Duration::from_millis(50)).map(|_| Message::GetUsersData));
        }

        if self.groups_list.is_none() && self.current_page == Page::Groups {
            scripts.push(time::every(Duration::from_millis(50)).map(|_| Message::GetGroupsData));
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
        icon_button("export", "Экспорт").on_press(Message::Dummy),
        icon_button("settings", "Настройки").on_press(Message::SelectPage(Page::Settings)),
        icon_button("about", "О программе").on_press(Message::SelectPage(Page::About)),
    ]
    .spacing(2)
    .align_y(Center);

    let pages_bar = column![
        text("Оборудование").style(text::secondary),
        sidebar_button(Page::Dashboard, cur_page),
        sidebar_button(Page::Processors, cur_page),
        sidebar_button(Page::Memory, cur_page),
        sidebar_button(Page::Storage, cur_page),
        sidebar_button(Page::DMI, cur_page),
        sidebar_button(Page::Battery, cur_page),
        sidebar_button(Page::Screen, cur_page),
        text("Администрирование").style(text::secondary),
        sidebar_button(Page::Distro, cur_page),
        sidebar_button(Page::Users, cur_page),
        sidebar_button(Page::Groups, cur_page),
        sidebar_button(Page::SystemManager, cur_page),
        sidebar_button(Page::Software, cur_page),
        sidebar_button(Page::Environment, cur_page),
        sidebar_button(Page::Sensors, cur_page),
        text("Система").style(text::secondary),
        sidebar_button(Page::Kernel, cur_page),
        sidebar_button(Page::KModules, cur_page),
        sidebar_button(Page::Development, cur_page),
        text("Обслуживание").style(text::secondary),
        sidebar_button(Page::Settings, cur_page),
        sidebar_button(Page::About, cur_page),
    ]
    .spacing(5);

    container(column![buttons_bar, scrollable(pages_bar)].spacing(5))
        .padding(5)
        .style(container::bordered_box)
        .height(Length::Fill)
}
