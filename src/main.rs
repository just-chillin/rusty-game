mod application;
mod entities;

extern crate sdl2;

use crate::application::Application;

fn main() {
    let app = Application::new();
    loop {
        app.tick();
    }
}