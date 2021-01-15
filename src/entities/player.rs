use crate::entities::Entity;
use std::io::Cursor;

pub(crate) struct Player {
    sprite: Cursor<T>
}

impl Player {
    pub fn new() -> Player {
        Player {
            sprite: Cursor(include_bytes!("../../assets/PepeSprite.bmp"))
        }
    }
}

impl Entity for Player {
    fn tick(&self) {
        unimplemented!()
    }
}