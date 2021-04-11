![Maintenance](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)
[![CI](https://github.com/BartMassey/igpay-atinlay/actions/workflows/main.yml/badge.svg)](https://github.com/BartMassey/igpay-atinlay/actions)
[![crates-io](https://img.shields.io/crates/v/igpay-atinlay.svg)](https://crates.io/crates/igpay-atinlay)
[![api-docs](https://docs.rs/igpay-atinlay/badge.svg)](https://docs.rs/igpay-atinlay)

# igpay-atinlay: convert text to Pig Latin
Bart Massey 2021 (version 0.1.0)

Text to [Pig Latin](https://en.wikipedia.org/wiki/Pig_Latin) conversion.

This crate began as a solution to [Exercise
8.2](https://doc.rust-lang.org/book/ch08-03-hash-maps.html#summary) in [The Rust Programming
Language](https://doc.rust-lang.org/book/). The innocuous-looking phrase "Keep in mind the
details about UTF-8 encoding!" in that exercise conceals a world of pain.

At the end of the day, Pig Latin is not a well-defined dialect. This program will produce
questionable results for complicated cases and non-Romance languages (for which the
underlying crate has no idea of extra vowels). It will probably produce nonsense answers on
"exotic" languages.

## Authors

* Reddit [/u/AlexRodger](https://reddit.com/u/AlexRodger) wrote the [original
  code](https://www.reddit.com/r/learnrust/comments/mo5lvd/rate_and_critic_my_solution_to_exercise_2_in/).

* Reddit [/u/hjd_thd](https://reddit.com/u/hjd_thd) rewrote to get an [FP
  solution](https://www.reddit.com/r/learnrust/comments/mo5lvd/rate_and_critic_my_solution_to_exercise_2_in/gu1w5s6).

* Reddit [/u/po8](https://reddit.com/u/po8) rewrote again into a more production-grade
  version. The rewrite ended up being pretty from-scratch.

# License

This crate is made available under the "MIT
license". Please see the file `LICENSE` in this distribution
for license terms.

# Acknowledgments

Thanks to the `cargo-readme` crate for generation of this `README`.
