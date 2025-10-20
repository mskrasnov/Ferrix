//! Pages with information about hardware and software

use iced::{
    Alignment::{self, Center},
    Element, Length,
    widget::{Column, center, column, container, row, rule, svg, table, text},
};

use crate::{Ferrix, Message, widgets::link_button};

mod cpu;
mod dashboard;
mod distro;
mod groups;
mod kernel;
mod ram;
mod settings;
mod system;
mod systemd;
mod users;

pub use kernel::KernelData;

use super::DataLoadingState;

impl<P> DataLoadingState<P> {
    pub fn page<'a>(&'a self) -> Option<container::Container<'a, Message>> {
        match self {
            Self::Loading => Some(container(center(text("Загрузка данных")))),
            Self::Error(why) => Some(container(
                column![
                    text("Ошибка загрузки данных!").style(text::danger).size(28),
                    text(why),
                ]
                .spacing(5),
            )),
            Self::Loaded(_) => None,
        }
    }

    pub fn to_option<'a>(&'a self) -> Option<&'a P> {
        match self {
            Self::Loaded(data) => Some(data),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
pub enum Page {
    /************************************
     *       Hardware & dashboard       *
     ************************************/
    #[default]
    Dashboard,
    Processors,
    Memory,
    Storage,
    DMI,
    Battery,
    Screen,

    /************************************
     *          Administration          *
     ************************************/
    Distro,
    SystemMisc,
    Users,
    Groups,
    SystemManager,
    Software,
    Environment,
    Sensors,

    /************************************
     *               Kernel             *
     ************************************/
    Kernel,
    KModules,
    Development,

    /************************************
     *              Service             *
     ************************************/
    Settings,
    About,
    Todo,
}

impl<'a> Page {
    pub fn title(&'a self) -> iced::widget::Column<'a, Message> {
        header_text(self.title_str())
    }

    pub fn title_str(&self) -> &'static str {
        match self {
            Self::Dashboard => "Обзор",
            Self::Processors => "Процессоры",
            Self::Memory => "Память",
            Self::Storage => "Накопители",
            Self::DMI => "Таблицы DMI",
            Self::Battery => "Аккумулятор",
            Self::Screen => "Экран",
            Self::Distro => "Дистрибутив",
            Self::Users => "Пользователи",
            Self::Groups => "Группы",
            Self::SystemManager => "Системный менеджер",
            Self::Software => "Установленное ПО",
            Self::Environment => "Окружение",
            Self::Sensors => "Сенсоры",
            Self::Kernel => "Ядро Linux",
            Self::KModules => "Модули ядра",
            Self::Development => "Разработка",
            Self::SystemMisc => "Разное",
            Self::Settings => "Настройки",
            Self::About => "О программе",
            Self::Todo => "Не реализованный функционал",
        }
    }

    pub fn page(&'a self, state: &'a Ferrix) -> Element<'a, Message> {
        let page = match self {
            Self::Dashboard => dashboard::dashboard(
                state.proc_data.to_option(),
                &state.ram_data,
                &state.osrel_data,
                &state.system,
            )
            .into(),
            Self::Processors => cpu::proc_page(&state.proc_data).into(),
            Self::Memory => ram::ram_page(&state.ram_data).into(),
            Self::Distro => distro::distro_page(&state.osrel_data).into(),
            Self::Kernel => kernel::kernel_page(&state.info_kernel, &state.kmodules_list).into(),
            Self::SystemMisc => system::system_page(&state.system).into(),
            Self::Users => users::users_page(&state.users_list).into(),
            Self::Groups => groups::groups_page(&state.groups_list).into(),
            Self::SystemManager => systemd::services_page(&state.sysd_services_list).into(),
            Self::Settings => settings::settings_page(&state).into(),
            Self::About => self.about_page().into(),
            _ => self.todo_page(),
        };

        column![self.title(), page,].spacing(5).into()
    }

    fn todo_page(&self) -> Element<'a, Message> {
        container(center(
            text("Этот функционал ещё не реализован")
                .size(16)
                .style(text::secondary),
        ))
        .into()
    }

    fn about_page(&'a self) -> container::Container<'a, Message> {
        let img = svg("ferrix-app/data/icons/hicolor/scalable/apps/com.mskrasnov.Ferrix.svg")
            .width(128)
            .height(128);
        let header = row![
            img,
            column![
                text("Ferrix — ещё один системный профайлер для Linux").size(24),
                text(format!(
                    "Версия Ferrix: {}. Версия ferrix-lib: {}",
                    env!("CARGO_PKG_VERSION"),
                    ferrix_lib::FX_LIB_VERSION,
                ))
                .size(14),
            ]
            .spacing(5),
        ]
        .align_y(Center)
        .spacing(5);

        let about_info = row![
            column![
                text("Автор:").style(text::secondary),
                text("Фидбек:").style(text::secondary),
                text("Исходный код:").style(text::secondary),
                text("crates.io:").style(text::secondary),
            ]
            .align_x(Alignment::End)
            .spacing(5),
            column![
                row![
                    text("(C) 2025 Михаил Краснов"),
                    link_button("(GitHub)", "https://github.com/mskrasnov"),
                ]
                .spacing(5),
                link_button("mskrasnov07 at ya dot ru", "mailto:mskrasnov07@ya.ru"),
                link_button("GitHub", "https://github.com/mskrasnov/Ferrix"),
                row![
                    link_button("ferrix-app", "https://crates.io/crates/ferrix-app"),
                    text(", "),
                    link_button("ferrix-lib", "https://crates.io/crates/ferrix-lib"),
                ],
            ]
            .spacing(5),
        ]
        .spacing(5);

        let misc =
            text("Вы можете отправить донат на карту:\n\t2202 2062 5233 5406 (Сбер)\nСпасибо!");

        let contents = column![
            column![header, rule::horizontal(1)].spacing(2),
            about_info,
            misc,
        ]
        .spacing(5);

        container(contents)
    }
}

#[derive(Debug, Clone)]
pub struct InfoRow<V> {
    pub param_header: String,
    pub value: Option<V>,
}

impl<V> InfoRow<V> {
    pub fn new<P>(param: P, value: Option<V>) -> Self
    where
        P: Into<String>,
        V: ToString,
    {
        Self {
            param_header: param.into(),
            value,
        }
    }
}

fn text_fmt_val<'a, V>(val: Option<V>) -> text::Text<'a>
where
    V: ToString + 'a,
{
    match val {
        Some(val) if !val.to_string().is_empty() => text(val.to_string()),
        Some(_) => text("N/A"),
        None => text(""),
    }
}

pub fn kv_info_table<'a, V>(rows: Vec<InfoRow<V>>) -> Element<'a, Message>
where
    V: ToString + Clone + 'a,
{
    let columns = [
        table::column(hdr_name("Параметр"), |row: InfoRow<V>| {
            text(row.param_header)
        }),
        table::column(hdr_name("Значение"), |row: InfoRow<V>| {
            text_fmt_val(row.value)
        })
        .width(Length::Fill),
    ];

    table(columns, rows).padding(2).width(Length::Fill).into()
}

fn hdr_name<'a>(s: &'a str) -> text::Text<'a> {
    text(s).style(text::secondary)
}

fn header_text<'a>(txt: &'a str) -> Column<'a, Message> {
    column![text(txt).size(22), rule::horizontal(1)].spacing(2)
}

fn fmt_val<T>(val: Option<T>) -> Option<String>
where
    T: ToString + Copy,
{
    match val {
        Some(val) => Some(val.to_string()),
        None => None,
    }
}

fn fmt_vec<T>(val: &Option<Vec<T>>) -> Option<String>
where
    T: ToString + Clone,
{
    match val {
        Some(val) => {
            let mut s = String::new();
            for i in val {
                s = format!("{s}{} ", i.to_string());
            }
            Some(s)
        }
        None => None,
    }
}

fn fmt_bool(val: Option<bool>) -> Option<String> {
    match val {
        Some(val) => match val {
            true => Some("YES".to_string()),
            false => Some("NO".to_string()),
        },
        None => None,
    }
}
