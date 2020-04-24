extern crate input;
extern crate piston;
extern crate piston_window;
extern crate static_assertions as sa;

mod agent;
mod config;
mod game;
mod player;
mod ship;

fn main() {
    game::run();
}
