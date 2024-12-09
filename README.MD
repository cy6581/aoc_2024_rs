# Advent of Code 2024 Rust Solutions

## About

Journal to document my Advent of Code solutions, with the aim to maximise my learning in Rust and Algorithms.

I usually have a different goal for each day's problem. To better document my thinking on what I was trying to practice/ learn from the day's problem, I tried to write a Commentary for each day.

[ ] TODO: Finish commentaries for Days 04 - 06

## How to run

Every day's solution is written as a separate binary crate. Run it like so:

`cargo run --bin <day_XX>` e.g., `cargo run --bin day_01`

The only reason I'm doing it like this is because I recently learnt that you can actually have _multiple_ binary crates in a package. Wow.

If you wish to benchmark the runtime of a particular solution, you an compile for release like so:

`cargo build --release --bin <day_XX>` e.g., `cargo build --release --bin day_01`

Then execute the release binary using:

`./target/release/day_<XX>` e.g., `./target/release/day_01`

The dev (aka debug) build and release build have a significant speed difference for certain days' solutions, e.g. day 06, so this can make things much faster.
