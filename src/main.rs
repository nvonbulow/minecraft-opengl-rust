#[macro_use]
extern crate glium;
extern crate image;
extern crate uuid;

mod game;

fn main() {
    game::start();
}