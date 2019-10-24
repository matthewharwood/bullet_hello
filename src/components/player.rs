use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct Player {
  pub width: f32,
  pub height: f32,
}

impl Component for Player {
  type Storage = DenseVecStorage<Self>;
}
