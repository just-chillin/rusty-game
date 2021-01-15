pub mod gamemode;
mod player;

pub trait Entity {
    fn tick(&self);
}