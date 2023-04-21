use fluent_templates::fluent_bundle::FluentValue;
use fluent_templates::{static_loader, LanguageIdentifier, Loader};
use std::collections::HashMap;

static_loader! {
    pub static TEXTS = {
        locales: "./assets/texts",
        fallback_language: "en-US"
    };
}

pub fn text(language: &LanguageIdentifier, id: &str) -> String {
    TEXTS.lookup(language, id).unwrap_or_else(|| {
        tracing::error!("Unknown text id '{id}'");
        id.to_string()
    })
}

pub fn text_with_args<T: AsRef<str>>(
    language: &LanguageIdentifier,
    id: &str,
    args: &HashMap<T, FluentValue>,
) -> String {
    TEXTS
        .lookup_with_args(language, id, args)
        .unwrap_or_else(|| {
            tracing::error!("Unknown text id '{id}'");
            id.to_string()
        })
}
