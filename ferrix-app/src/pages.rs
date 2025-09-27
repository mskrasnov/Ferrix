//! Pages with information about hardware and software

use iced::{
    Element, Length,
    widget::{table, text},
};

use crate::Message;

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

fn fmt_val<'a, V>(val: Option<V>) -> text::Text<'a>
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
        // .width(Length::FillPortion(1)),
        table::column(hdr_name("Значение"), |row: InfoRow<V>| {
            fmt_val(row.value)
        })
        .width(Length::Fill),
    ];

    table(columns, rows).width(Length::Fill).into()
}

fn hdr_name<'a>(s: &'a str) -> text::Text<'a> {
    text(s).style(text::secondary)
}
