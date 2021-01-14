mod entities;

struct Game {
    children: Vec<Box<dyn Entity>>
}

impl Game {
    fn new() -> Game {
        Game {
            children: vec![]
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