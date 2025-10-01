/*
 * main.rs
 * styles/
 * widgets/
 * modals/
 */

use ferrix_lib::{cpu::Processors, ram::RAM, sys::OsRelease};
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

pub fn main() -> iced::Result {
    iced::application(Ferrix::default, Ferrix::update, Ferrix::view)
        .settings(iced::Settings {
            default_text_size: iced::Pixels(12.),
            ..Default::default()
        })
        .window(Settings {
            icon: Some(
                iced::window::icon::from_file(
                    "ferrix-app/data/icons/hicolor/scalable/apps/com.mskrasnov.Ferrix.png",
                )
                .unwrap(),
            ),
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
    pub settings: FXSettings,
}

impl Default for Ferrix {
    fn default() -> Self {
        Self {
            current_page: Page::default(),
            proc_data: None,
            ram_data: None,
            osrel_data: None,
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
        sidebar_button(Page::UsersGroups, cur_page),
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
