extern crate sdl2;

pub trait Entity {
    fn new() -> dyn Entity;
    fn tick();
}

struct Player {

}

impl Entity for Player {
    fn new() -> Player {
        unimplemented!()
    }

    fn tick() {
        unimplemented!()
    }
}

fn main() {
    let sdl_ctx = sdl2::init().unwrap();
    let video_subsystem = sdl_ctx.video().unwrap();
    let evt_subsystem = sdl_ctx.event().unwrap();
    let win = video_subsystem.window("rusty-game!", 800, 600)
        .build()
        .unwrap();
    let mut canvas = win.into_canvas().build().unwrap();
}