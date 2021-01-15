use crate::entities::{
    Entity,
    player::Player,
};

struct Game {
    children: Vec<Box<dyn Entity>>
}

impl Game {
    fn new() -> Game {
        Game {
            children: vec![Box::new(Player::new())]
        }
    }
}

impl Entity for Game {
    fn tick(&self) {
        for ent in self.children.as_slice() {
            ent.tick();
        }
    }
}