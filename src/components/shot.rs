use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct Shot {
    pub width: f32,
    pub height: f32,
    pub velocity: [f32; 2],
}

impl Component for Shot {
    type Storage = DenseVecStorage<Self>;
}
