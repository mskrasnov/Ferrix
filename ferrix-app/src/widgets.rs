//! Custom widgets for UI

// use super::Message;
use iced::{
    Color,
    widget::{button, container, svg, text, tooltip::Position},
};

use crate::{Message, pages::Page};

pub fn icon_button<'a>(icon_name: &'a str, tooltip: &'a str) -> button::Button<'a, Message> {
    let svg_path = format!(
        "ferrix-app/data/icons/hicolor/symbolic/actions/ferrix-{}.svg",
        icon_name
    );
    let icon = svg(svg_path).style(|theme: &iced::Theme, _| svg::Style {
        color: Some(theme.palette().text),
    });

    button(iced::widget::tooltip(
        icon.width(16).height(16),
        container(text(tooltip).size(11).style(|s: &iced::Theme| text::Style {
            color: Some(if s.extended_palette().is_dark {
                s.palette().text
            } else {
                Color::WHITE
            }),
        }))
        .padding(2)
        .style(|_| container::Style {
            background: Some(iced::Background::Color(Color::from_rgba8(0, 0, 0, 0.71))),
            border: iced::Border {
                radius: iced::border::Radius::from(2),
                ..iced::Border::default()
            },
            ..Default::default()
        }),
        Position::Bottom,
    ))
    .style(button::subtle)
    .padding(2)
}

pub fn sidebar_button<'a>(page: Page, cur_page: Page) -> button::Button<'a, Message> {
    button(page.title_str())
        .style(if page != cur_page {
            button::subtle
        } else {
            button::secondary
        })
        .on_press(Message::SelectPage(page))
}
