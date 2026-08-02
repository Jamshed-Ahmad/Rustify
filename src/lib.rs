#![forbid(unsafe_code)]

pub mod error;
pub mod slugify;
pub mod special;

pub use error::{Result, SlugifyError};

pub use slugify::{slugify, smart_truncate, SlugifyOptions, DEFAULT_SEPARATOR};
pub use special::{
    get_cyrillic, get_german, get_greek, get_pre_translations, ReplacementPair,
};

pub static CYRILLIC: once_cell::sync::Lazy<Vec<ReplacementPair>> =
    once_cell::sync::Lazy::new(get_cyrillic);
pub static GERMAN: once_cell::sync::Lazy<Vec<ReplacementPair>> =
    once_cell::sync::Lazy::new(get_german);
pub static GREEK: once_cell::sync::Lazy<Vec<ReplacementPair>> =
    once_cell::sync::Lazy::new(get_greek);
pub static PRE_TRANSLATIONS: once_cell::sync::Lazy<Vec<ReplacementPair>> =
    once_cell::sync::Lazy::new(get_pre_translations);
