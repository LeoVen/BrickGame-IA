extern crate piston_window;
extern crate static_assertions as sa;

mod agent;
mod config;
mod game;
mod ship;

fn main() {
    game::run();
}
