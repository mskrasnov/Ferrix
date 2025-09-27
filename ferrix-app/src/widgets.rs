//! Custom widgets for UI

// use super::Message;
use iced::widget::{column, container, svg, text};

pub fn title<'a, T: ToString>(t: T) -> text::Text<'a> {
    text(t.to_string()).size(14)
}

#[derive(Debug, Clone)]
pub struct CardContainer {
    header: String,
    contents: String,
    icon_name: Option<String>,
}

impl CardContainer {
    pub fn new<H, C>(hdr: H, contents: C) -> Self
    where
        H: ToString,
        C: ToString,
    {
        Self {
            header: hdr.to_string(),
            contents: contents.to_string(),
            icon_name: None,
        }
    }

    pub fn set_icon<I: ToString>(&mut self, icon: I) {
        self.icon_name = Some(icon.to_string());
    }

    pub fn to_container<'a, Message>(&'a self) -> container::Container<'a, Message>
    where
        Message: 'a,
    {
        let mut cnt = column![title(&self.header),].spacing(5);
        if let Some(icon_name) = &self.icon_name {
            cnt = cnt.push(svg(icon_name));
        }
        cnt = cnt.push(text(&self.contents));

        container(cnt)
            .width(512)
            .height(512)
            .padding(10)
            .style(container::rounded_box)
    }
}
