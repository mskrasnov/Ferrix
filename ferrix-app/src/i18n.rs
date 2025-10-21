//! Internationalization support

use i18n_embed::{
    DesktopLanguageRequester,
    fluent::{FluentLanguageLoader, fluent_language_loader},
};
use lazy_static::lazy_static;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "./i18n"]
struct Locales;

fn read() -> FluentLanguageLoader {
    let ldr: FluentLanguageLoader = fluent_language_loader!();
    let req_langs = DesktopLanguageRequester::requested_languages();
    i18n_embed::select(&ldr, &Locales, &req_langs).unwrap();

    ldr
}

lazy_static! {
    pub static ref LANG_LDR: FluentLanguageLoader = read();
}

#[macro_export]
macro_rules! fl {
    ($message_id:literal) => {{
        i18n_embed_fl::fl!($crate::i18n::LANG_LDR, $message_id)
    }};

    ($message_id:literal, $($args:expr),*) => {{
        i18n_embed_fl::fl!($crate::i18n::LANG_LDR, $message_id, $($args), *)
    }};
}
