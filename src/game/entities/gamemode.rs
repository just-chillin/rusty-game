use crate::engine::entities::gamemode::GameController;
use crate::engine::Entity;
use crate::game::entities::player::Player;

pub struct BoringGameMode {
    children: Vec<Box<dyn Entity>>
}

impl Entity for BoringGameMode {
    fn tick(&self) {
        for ent in self.children.as_slice() {
            ent.tick();
        }
    }
}

impl GameController for BoringGameMode {
    fn new() -> Box<dyn GameController> {
        Box::new(BoringGameMode {
            children: vec![Box::new(Player::new())]
        })
    }
}