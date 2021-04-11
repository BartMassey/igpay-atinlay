//! Text to Pig Latin conversion.
//!
//! Began as a solution to [Exercise
//! 8.2](https://doc.rust-lang.org/book/ch08-03-hash-maps.html#summary) in [The Rust
//! Programming Language](https://doc.rust-lang.org/book/) The innocuous-looking phrase "Keep
//! in mind the details about UTF-8 encoding!" in that exercise conceals a world of pain.
//!
//! At the end of the day, Pig Latin is not a well-defined dialect. This program will produce
//! questionable results for complicated cases and non-Romance languages (for which the
//! underlying crate has no idea of extra vowels). It will probably produce nonsense answers on
//! "exotic" languages.
//!
//! # Authors
//!
//! * Reddit
//!   [/u/AlexRodger](https://reddit.com/u/AlexRodger) wrote
//!   the [original
//!   code](https://www.reddit.com/r/learnrust/comments/mo5lvd/rate_and_critic_my_solution_to_exercise_2_in/).
//!
//! * Reddit [/u/hjd_thd](https://reddit.com/u/hjd_thd)
//!   rewrote to get an [FP
//!   solution](https://www.reddit.com/r/learnrust/comments/mo5lvd/rate_and_critic_my_solution_to_exercise_2_in/gu1w5s6).
//!
//! * Reddit [/u/po8](https://reddit.com/u/po8) rewrote again into a more production-grade
//!   version. The rewrite ended up being pretty from-scratch.

use is_vowel::IsRomanceVowel;

use once_cell::sync::Lazy;
use regex::Regex;

/// Transform `word` to [Pig Latin](https://en.wikipedia.org/wiki/Pig_Latin). Word is assumed to
/// be in a [Romance language](https://en.wikipedia.org/wiki/Romance_languages): see
/// `[IsRomanceVowel][IsRomanceVowel]::[is_romance_vowel][is_romance_vowel]` for the definition
/// of "vowel" used here.

/// `vowel_suffix_start` is the string appended before "ay" after words beginning with a vowel:
/// this is usually "w" or "h", but also might be "j", "" or any of a number of other things
/// depending on the Pig Latin "dialect".
pub fn word_to_pig_latin(word: &str, vowel_suffix_start: &str) -> String {
    let mut chars = word.chars();
    let first = chars.next();
    let mut result: String = match first {
        Some(first) => {
            if first.is_romance_vowel() {
                let mut result = word.to_string();
                result += vowel_suffix_start;
                result
            } else {
                if first.is_uppercase() {
                    let (_, max_hint) = chars.size_hint();
                    let max_hint = max_hint.unwrap_or(32);
                    let mut result = String::with_capacity(max_hint);
                    if let Some(second) = chars.next() {
                        result.extend(second.to_uppercase());
                    }
                    result.extend(chars);
                    result.extend(first.to_lowercase());
                    result
                } else {
                    let mut result: String = chars.collect();
                    result.push(first);
                    result
                }
            }
        }
        None => {
            return word.to_string();
        }
    };
    result += "ay";
    result
}

/// Map alphabetic words in `text` using the `word_processor` mapping function.
fn map_words<F>(text: &str, mut word_processor: F) -> String
    where F: FnMut(&str) -> String
{
    // XXX Should perhaps have other connecting punctuation not covered by the Unicode tables
    // here?
    // XXX Not sure how to avoid repeating the regex here. May not be a way on stable Rust
    // currently.
    #[cfg(not(feature = "split_hyphens"))]
    const WORD_RE_STR: &'static str = r"(?x)
            \p{Alphabetic} (
                ( \p{Alphabetic}
                  | \p{Join_Control}
                  | \p{Mark}
                  | \p{Connector_Punctuation}
                  | [-'’]
                )*
              \p{Alphabetic} )?
        ";
    #[cfg(feature = "split_hyphens")]
    const WORD_RE_STR: &'static str = r"(?x)
            \p{Alphabetic} (
                ( \p{Alphabetic}
                  | \p{Join_Control}
                  | \p{Mark}
                  | \p{Connector_Punctuation}
                  | ['’]
                )*
              \p{Alphabetic} )?
        ";
    static WORD_RE: Lazy<Regex> = Lazy::new(|| Regex::new(WORD_RE_STR).unwrap());
    WORD_RE.replace_all(text, |w: &regex::Captures| {
        word_processor(w.get(0).unwrap().as_str())
    }).to_string()
}

/// Transform `text` to [Pig Latin](https://en.wikipedia.org/wiki/Pig_Latin).
/// See [`to_pig_latin_word`][to_pig_latin_word] for an explanation of
/// `vowel_suffix_start`.
pub fn text_to_pig_latin(text: &str, vowel_suffix_start: &str) -> String {
    map_words(text.as_ref(), |w| word_to_pig_latin(w, vowel_suffix_start))
}
