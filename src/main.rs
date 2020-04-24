//!
#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

extern crate static_assertions as sa;

mod agent;
mod config;
mod game;
mod player;
mod ship;

fn main() {
    game::run();
}
