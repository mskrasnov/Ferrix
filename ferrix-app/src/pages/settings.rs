//! Program settings page

use crate::{Ferrix, Message, widgets::icon_tooltip};
use iced::{
    Alignment::Center,
    Theme,
    widget::{center, column, container, pick_list, row, rule, slider, text},
};

pub fn settings_page<'a>(state: &'a Ferrix) -> container::Container<'a, Message> {
    let update_sec = slider(
        1..=10,
        state.settings.update_period,
        Message::ChangeUpdatePeriod,
    );
    let update_changer = column![
        row![text("Период обновления").size(16), icon_tooltip("about", "Укажите период обновления данных (в секундах). Чем выше период обновления, тем ниже нагрузка на ПК."), rule::horizontal(1.)]
            .spacing(5)
            .align_y(Center),
        row![
            update_sec,
            container(center(text(state.settings.update_period).size(14)))
                .style(container::rounded_box)
                .width(22)
                .height(22),
        ]
        .spacing(5)
        .align_y(Center),
    ]
    .spacing(5);

    let theme_selector = pick_list(
        Theme::ALL,
        Some(&state.settings.theme),
        Message::ChangeTheme,
    );
    let theme_changer = column![
        row![text("Оформление программы").size(16), icon_tooltip("about", "Стиль оформления влияет на цвета интерфейса и шрифта. Выберите то, что нравится вам."), rule::horizontal(1.)]
            .spacing(5)
            .align_y(Center),
        row![text("Выберите нужный стиль оформления:"), theme_selector]
            .spacing(5)
            .align_y(Center),
    ]
    .spacing(5);

    let layout = container(column![update_changer, theme_changer].spacing(5));
    layout
}
