pub mod game;

pub trait Entity {
    fn tick(&self);
}