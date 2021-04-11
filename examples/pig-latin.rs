//! Solution to exercise 8.2 in [The Rust Programming Language](https://doc.rust-lang.org/book/)
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
//! * Reddit [/u/po8](https://reddit.com/u/po8) rewrote
//!   again into a more production-grade version.

use is_vowel::IsRomanceVowel;

/// Transform `word` to [Pig Latin](https://en.wikipedia.org/wiki/Pig_Latin). Word is assumed to
/// be in a [Romance language](https://en.wikipedia.org/wiki/Romance_languages): see
/// `[IsRomanceVowel][IsRomanceVowel]::[is_romance_vowel][is_romance_vowel]` for the definition
/// of "vowel" used here.

/// `vowel_suffix_start` is the string appended before "ay" after words beginning with a vowel:
/// this is usually "w" or "h", but also might be "j", "" or any of a number of other things
/// depending on the Pig Latin "dialect".
pub fn word_to_pig_latin<S, T>(word: S, vowel_suffix_start: T) -> String
    where S: AsRef<str>, T: AsRef<str>
{
    let rword = word.as_ref();
    let mut chars = rword.chars();
    let first = chars.next();
    let mut result = match first {
        Some(first) => {
            if first.is_romance_vowel() {
                let suf = vowel_suffix_start.as_ref();
                let mut result = rword.to_string();
                result += suf;
                result
            } else {
                let mut result: String = chars.collect();
                result.push(first);
                result
            }
        }
        None => {
            return rword.to_string();
        }
    };
    result += "ay";
    result
}

/// Map alphabetic words in `text` using the `word_processor` mapping function.
fn map_words<S, F>(text: S, mut word_processor: F) -> String
    where S: AsRef<str>, F: FnMut(&str) -> String
{
    let text: Vec<char> = text.as_ref().chars().collect();
    let mut start: &[char] = &text;
    let mut result = String::with_capacity(start.len());
    while !start.is_empty() {
        if start[0].is_alphabetic() {
            let index = match start.find(|c| !c.is_alphabetic()) {
                Some(i) => i,
                None => start.len(),
            };
            let word: String = start[..index].iter().cloned().collect();
            result += &word_processor(&word);
            start = &start[index..];
        } else {
            result.push(start[0]);
            start = &start[1..];
        }
    }
    result
}

/// Transform `text` to [Pig Latin](https://en.wikipedia.org/wiki/Pig_Latin).
/// See [`to_pig_latin_word`][to_pig_latin_word] for an explanation of
/// `vowel_suffix_start`.
pub fn text_to_pig_latin<S, T>(text: S, vowel_suffix_start: T) -> String
    where S: AsRef<str>, T: AsRef<str>
{
    map_words(text.as_ref(), |w| word_to_pig_latin(w, vowel_suffix_start.as_ref()))
}

/// Emit command-line argument text as Pig Latin with "h" vowel suffix.
pub fn main() {
    let text: String = std::env::args()
        .skip(1)
        .collect::<Vec<_>>()
        .join(" ");
    println!("{}", text_to_pig_latin(text, "h"));
}
