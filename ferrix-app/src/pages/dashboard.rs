//! Dashboard page

use crate::Message;
use ferrix_lib::{cpu::Processors, ram::RAM, sys::OsRelease};

use iced::{
    Font, Length, never,
    widget::{button, column, container, rich_text, row, space, span, text::IntoFragment},
};

pub fn dashboard<'a>(
    proc: Option<&'a Processors>,
    ram: Option<&'a RAM>,
    osr: Option<&'a OsRelease>,
    system: Option<&'a crate::System>,
) -> container::Container<'a, Message> {
    let (proc_name, proc_threads) = {
        match proc {
            Some(proc) => {
                let model = &proc.entries[0].model_name;
                let vendor = match model {
                    Some(model) => model,
                    None => "N/A",
                };
                let threads = proc.entries.len();

                (vendor, threads)
            }
            None => ("N/A", 0),
        }
    };
    let (total_ram, avail_ram) = {
        match ram {
            Some(ram) => (
                ram.total.round(2).unwrap_or(ferrix_lib::utils::Size::None),
                ram.available
                    .round(2)
                    .unwrap_or(ferrix_lib::utils::Size::None),
            ),
            None => (ferrix_lib::utils::Size::None, ferrix_lib::utils::Size::None),
        }
    };
    let os_name = {
        match osr {
            Some(osr) => match &osr.pretty_name {
                Some(pname) => pname,
                None => &osr.name,
            },
            None => "Generic Linux",
        }
    };
    let hostname = match system {
        Some(system) => match &system.hostname {
            Some(hostname) => hostname as &str,
            None => "Unknown hostname",
        },
        None => "Unknown hostname",
    };

    container(
        column![
            row![
                card(
                    "Процессор",
                    format!("{}, {} потоков", proc_name, proc_threads),
                    Message::SelectPage(crate::pages::Page::Processors),
                ),
                card(
                    "Оперативная память",
                    format!("{}/{}", avail_ram, total_ram),
                    Message::SelectPage(crate::pages::Page::Memory),
                ),
                card(
                    "Система",
                    os_name,
                    Message::SelectPage(crate::pages::Page::Distro),
                ),
                card(
                    "Имя хоста",
                    hostname,
                    Message::SelectPage(crate::pages::Page::SystemMisc),
                ),
            ]
            .spacing(5)
        ]
        .spacing(5),
    )
}

fn card<'a, H, C>(header: H, contents: C, on_press: Message) -> button::Button<'a, Message>
where
    H: IntoFragment<'a>,
    C: IntoFragment<'a>,
{
    button(
        container(
            column![
                rich_text![
                    span(header)
                        .font(Font {
                            weight: iced::font::Weight::Bold,
                            ..Default::default()
                        })
                        .size(16),
                ]
                .on_link_click(never),
                space().width(Length::Fill).height(Length::Fill),
                iced::widget::text(contents),
            ]
            .spacing(5),
        )
        .width(135)
        .max_width(135)
        .height(135)
        .max_height(135)
        .style(container::rounded_box)
        .padding(5),
    )
    .style(button::text)
    .padding(0)
    .on_press(on_press)
}
