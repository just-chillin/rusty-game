use crate::engine::Entity;

pub trait GameController: Entity {
    fn new() -> Box<dyn GameController> where Self: Sized;
}