/*
 * main.rs
 * styles/
 * widgets/
 * modals/
 */

use std::time::Duration;

use ferrix_lib::cpu::Processors;
use iced::{
    Color, Element, Length, Subscription, Task, time,
    widget::{button, center, column, container, row, scrollable, text},
};

pub mod modals;
pub mod pages;
pub mod styles;
pub mod widgets;

use pages::*;

pub fn main() -> iced::Result {
    iced::application(Ferrix::default, Ferrix::update, Ferrix::view)
        .window_size((690., 480.))
        .antialiasing(true)
        .subscription(Ferrix::subscription)
        .theme(iced::Theme::Dark)
        .title("Ferrix")
        .settings(iced::Settings {
            default_text_size: iced::Pixels(14.),
            ..Default::default()
        })
        .run()
}

#[derive(Debug, Clone)]
pub enum Message {
    GetCPUData,
    CPUDataReceived((Option<Processors>, Option<String>)),
    Dummy,
    SelectPage(Page),
}

#[derive(Debug, Clone, Copy, Default)]
pub enum Page {
    Dashboard,
    #[default]
    Processors,
    Memory,
    Storage,
}

impl<'a> Page {
    pub fn page(&'a self, state: &'a Ferrix) -> Element<'a, Message> {
        match self {
            Self::Processors => self.proc_page(&state.proc_data).into(),
            _ => self.todo_page(),
        }
    }

    fn todo_page(&self) -> Element<'a, Message> {
        container(center(
            text("Этот функционал ещё не реализован")
                .size(16)
                .style(text::secondary),
        ))
        .into()
    }

    fn proc_page(
        &'a self,
        processors: &'a Option<Processors>,
    ) -> container::Container<'a, Message> {
        match processors {
            None => container(center(text("Загрузка данных..."))),
            Some(proc) => {
                let mut proc_list = column![header_text("Процессор")].spacing(5);
                for proc in &proc.entries {
                    let rows = vec![
                        InfoRow::new("Производитель", proc.vendor_id.clone()),
                        InfoRow::new("Модель", proc.model_name.clone()),
                        InfoRow::new("Stepping", fmt_val(proc.stepping)),
                        InfoRow::new("Частота", fmt_val(proc.cpu_mhz)),
                        InfoRow::new("Размер L3 кеша", fmt_val(proc.cache_size)),
                        InfoRow::new("Физический ID", fmt_val(proc.physical_id)),
                        InfoRow::new("ID ядра", fmt_val(proc.core_id)),
                        InfoRow::new("Число ядер", fmt_val(proc.cpu_cores)),
                        InfoRow::new("APIC ID", fmt_val(proc.apicid)),
                        InfoRow::new("Флаги", fmt_vec(&proc.flags)),
                        InfoRow::new("Баги", fmt_vec(&proc.bugs)),
                    ];

                    let proc_view = column![
                        text(format!("Процессор #{}", proc.processor.unwrap_or(0))).style(|_| {
                            text::Style {
                                color: Some(Color::from_rgba8(255, 255, 250, 15.)),
                            }
                        }),
                        container(kv_info_table(rows)).style(container::rounded_box),
                    ]
                    .spacing(5);
                    proc_list = proc_list.push(proc_view);
                }
                container(scrollable(proc_list))
            }
        }
    }
}

#[derive(Debug)]
pub struct Ferrix {
    pub current_page: Page,
    pub proc_data: Option<Processors>,
}

impl Default for Ferrix {
    fn default() -> Self {
        Self {
            current_page: Page::default(),
            proc_data: None,
        }
    }
}

impl Ferrix {
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
            Message::SelectPage(page) => {
                self.current_page = page;
                Task::none()
            }
            _ => Task::none(),
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([time::every(Duration::from_secs(1)).map(|_| Message::GetCPUData)])
    }

    fn view<'a>(&'a self) -> Element<'a, Message> {
        row![sidebar(), self.current_page.page(&self)].spacing(5).padding(5).into()
    }
}

fn sidebar<'a>() -> container::Container<'a, Message> {
    container(scrollable(
        column![
            row![
                button("Экспорт...")
                    .style(button::text)
                    .on_press(Message::Dummy),
                button("Обновить")
                    .style(button::text)
                    .on_press(Message::Dummy)
            ]
            .spacing(2),
            text("Оборудование").style(text::secondary),
            sidebar_button("Обзор", Page::Dashboard),
            sidebar_button("Процессор", Page::Processors),
            sidebar_button("Память", Page::Memory),
            sidebar_button("Накопители", Page::Storage),
            // button("Процессор")
            //     .style(button::text)
            //     .on_press(Message::Dummy),
            // button("Память")
            //     .style(button::text)
            //     .on_press(Message::Dummy),
            // button("Хранилище")
            //     .style(button::text)
            //     .on_press(Message::Dummy),
            // text("Администрирование").style(text::secondary),
            // button("Дистрибутив")
            //     .style(button::text)
            //     .on_press(Message::Dummy),
            // button("Пользователи и группы")
            //     .style(button::text)
            //     .on_press(Message::Dummy),
            // button("Системный менеджер")
            //     .style(button::text)
            //     .on_press(Message::Dummy),
            // button("Установленное ПО")
            //     .style(button::text)
            //     .on_press(Message::Dummy),
        ]
        .spacing(5),
    ))
    .padding(5)
    .style(container::bordered_box)
    .height(Length::Fill)
}

fn sidebar_button<'a>(txt: &'a str, page: Page) -> button::Button<'a, Message> {
    button(txt)
        .style(button::text)
        .on_press(Message::SelectPage(page))
}

fn header_text<'a>(txt: &'a str) -> text::Text<'a> {
    text(txt).size(22)
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
