mod engine;
mod game;

use game::entities::gamemode;
use engine::Application;
use crate::game::entities::BoringGameMode;

extern crate sdl2;

fn main() {
    let app = Application::new();
    app.run::<BoringGameMode>();
}