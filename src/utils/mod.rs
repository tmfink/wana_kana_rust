#[macro_use]
pub mod hashmap_macro;
pub mod get_chunk;
pub mod hiragana_to_katakana;
pub mod is_char_consonant;
pub mod is_char_english_punctuation;
pub mod is_char_hiragana;
pub mod is_char_in_range;
pub mod is_char_japanese;
pub mod is_char_japanese_number;
pub mod is_char_japanese_punctuation;
pub mod is_char_kana;
pub mod is_char_kanji;
pub mod is_char_katakana;
pub mod is_char_latin_number;
pub mod is_char_long_dash;
pub mod is_char_punctuation;
pub mod is_char_romaji;
pub mod is_char_slash_dot;
pub mod is_char_upper_case;
pub mod is_char_vowel;
pub mod katakana_to_hiragana;
pub mod romaji_to_hiragana;

pub use get_chunk::*;
pub use hashmap_macro::*;
pub use hiragana_to_katakana::*;
pub use is_char_consonant::*;
pub use is_char_english_punctuation::*;
pub use is_char_hiragana::*;
pub use is_char_in_range::*;
pub use is_char_japanese::*;
pub use is_char_japanese_number::*;
pub use is_char_japanese_punctuation::*;
pub use is_char_kana::*;
pub use is_char_kanji::*;
pub use is_char_katakana::*;
pub use is_char_latin_number::*;
pub use is_char_long_dash::*;
pub use is_char_punctuation::*;
pub use is_char_romaji::*;
pub use is_char_slash_dot::*;
pub use is_char_upper_case::*;
pub use is_char_vowel::*;
pub use katakana_to_hiragana::*;
pub use romaji_to_hiragana::*;
