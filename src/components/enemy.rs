use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct Enemy {
    pub width: f32,
    pub height: f32,
}

impl Component for Enemy {
    type Storage = DenseVecStorage<Self>;
}
