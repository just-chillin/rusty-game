use std::io::Cursor;
use crate::engine::Entity;

pub struct Player {
    sprite: Cursor<&'static [u8]>
}

impl Player {
    pub fn new() -> Player {
        Player {
            sprite: Cursor::new(include_bytes!("../../../assets/PepeSprite.bmp"))
        }
    }
}

impl Entity for Player {
    fn tick(&self) {
        unimplemented!()
    }
}