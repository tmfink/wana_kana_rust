//!
//! ### ワナカナ <--> WanaKana <--> わなかな
//!
//! Utility library for checking and converting between Japanese characters - Kanji, Hiragana, Katakana - and Romaji (Ported from https://github.com/WaniKani/WanaKana)
//! # Examples
//! ```
//! use wana_kana::to_romaji::*;
//! use wana_kana::to_kana::*;
//! use wana_kana::to_hiragana::*;
//! use wana_kana::Options;
//! assert_eq!(to_romaji("ワナカナ"), "wanakana");
//! assert_eq!(to_hiragana("WanaKana"), "わなかな");
//! assert_eq!(to_kana("WANAKANA"), "ワナカナ");
//! ```
extern crate itertools;
#[macro_use]
extern crate lazy_static;
extern crate fnv;

#[cfg(feature = "enable_regex")]
extern crate regex;

#[macro_use]
pub mod utils;

pub mod is_hiragana;
pub mod is_japanese;
pub mod is_kana;
pub mod is_kanji;
pub mod is_katakana;
pub mod is_mixed;
pub mod is_romaji;

pub mod to_hiragana;
pub mod to_kana;
pub mod to_katakana;
pub mod to_romaji;

pub mod tokenize;
pub mod trim_okurigana;

pub mod constants;
mod options;

pub use crate::options::Options;
