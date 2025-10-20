use iced::{Theme, widget::button};

/// A link button
pub fn link_button(theme: &Theme, status: button::Status) -> button::Style {
    let palette = theme.extended_palette();

    let base = button::Style {
        text_color: palette.danger.strong.color,
        ..button::Style::default()
    };

    match status {
        button::Status::Active | button::Status::Pressed => base,
        button::Status::Hovered => button::Style {
            text_color: palette.danger.base.color.scale_alpha(0.8),
            ..base
        },
        button::Status::Disabled => button_disabled(base),
    }
}

pub fn button_disabled(style: button::Style) -> button::Style {
    button::Style {
        background: style.background.map(|b| b.scale_alpha(0.5)),
        text_color: style.text_color.scale_alpha(0.5),
        ..style
    }
}
