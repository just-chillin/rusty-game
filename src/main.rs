use entities::application::Application;

mod entities;

extern crate sdl2;

fn main() {
    let mut app = Application::new();
    loop {
        app.tick();
    }
}