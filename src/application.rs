use std::borrow::Borrow;
use std::ops::Deref;
use sdl2::render::WindowCanvas;
use sdl2::{VideoSubsystem, EventSubsystem, Sdl};
use sdl2::video::Window;
use std::rc::Rc;
use crate::entities::Entity;
use sdl2::pixels::Color;

pub struct Application {
    sdl_ctx: Sdl,
    video_subsystem: VideoSubsystem,
    evt_subsystem: EventSubsystem,
    canvas: WindowCanvas,
    entities: std::vec::Vec<Box<dyn Entity>>,
}

impl Application {
    pub fn new() -> Application {
        let sdl_ctx = sdl2::init().unwrap();
        let video_subsystem = sdl_ctx.video().unwrap();
        let evt_subsystem = sdl_ctx.event().unwrap();
        let window = video_subsystem.window("rusty-game!", 800, 600)
            .build()
            .unwrap();
        let mut canvas = window.into_canvas().build().unwrap();
        Application {
            sdl_ctx, video_subsystem, evt_subsystem, canvas,
            entities: vec![]
        }
    }

    pub fn tick(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 255, 255));
        self.canvas.clear();
        for ent in self.entities.as_slice() {
            ent.tick();
        }
        canvas.present();
    }
}