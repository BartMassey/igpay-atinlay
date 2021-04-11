//! Pig Latin encoder for command-line arguments. Run like
//!
//! ```text
//! cargo run --example pig-latin "Can't" touch this! Awoo-away!
//! cargo run --example pig-latin "Can't touch this! Awoo-away!"
//! ```
//!
//! Either of these will print
//!
//! ```text
//! An'tcay ouchtay histay! Awoo-awayhay!
//! ```

use igpay_atinlay::text_to_pig_latin;

/// Emit command-line argument text as Pig Latin with "h" vowel suffix.
pub fn main() {
    let text: Vec<String> = std::env::args()
        .skip(1)
        .collect();
    println!("{}", text_to_pig_latin(&text.join(" "), "h"));
}
