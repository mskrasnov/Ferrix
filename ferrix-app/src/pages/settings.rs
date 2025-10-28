//! Program settings page

use crate::{Ferrix, Message, fl, widgets::icon_tooltip};
use iced::{
    Alignment::Center,
    widget::{button, center, column, container, pick_list, row, rule, slider, text},
};

pub fn settings_page<'a>(state: &'a Ferrix) -> container::Container<'a, Message> {
    let update_sec = slider(
        1..=10,
        state.settings.update_period,
        Message::ChangeUpdatePeriod,
    );
    let update_changer = column![
        row![
            text(fl!("settings-update-period")).size(16),
            icon_tooltip(
                "about",
                fl!("settings-uperiod-tip"),
            ),
            rule::horizontal(1.)
        ]
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
        crate::Style::ALL,
        Some(state.settings.style),
        Message::ChangeTheme,
    );
    let theme_changer = column![
        row![
            text(fl!("settings-look")).size(16),
            icon_tooltip(
                "about",
                fl!("settings-look-tip")
            ),
            rule::horizontal(1.)
        ]
        .spacing(5)
        .align_y(Center),
        row![text(fl!("settings-look-select")), theme_selector]
            .spacing(5)
            .align_y(Center),
    ]
    .spacing(5);

    let layout = container(
        column![
            update_changer,
            theme_changer,
            button(text(fl!("settings-save"))).on_press(Message::SaveSettingsButtonPressed),
        ]
        .spacing(5),
    );
    layout
}
