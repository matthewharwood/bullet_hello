use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct Background {
    pub width: f32,
    pub height: f32,
}

impl Component for Background {
    type Storage = DenseVecStorage<Self>;
}
