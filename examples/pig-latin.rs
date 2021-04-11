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

use igpay_atinlay::IgpayAtinlay;

/// Emit command-line argument text as Pig Latin with "h" vowel suffix.
pub fn main() {
    let text: Vec<String> = std::env::args()
        .skip(1)
        .collect();
    let pig = IgpayAtinlay::new("h", false);
    println!("{}", pig.text_to_pig_latin(&text.join(" ")));
}
