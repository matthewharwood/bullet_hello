use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct Cloud {
    pub width: f32,
    pub height: f32,
}

impl Component for Cloud {
    type Storage = DenseVecStorage<Self>;
}
