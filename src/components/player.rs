use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct Player {
  pub width: f32,
  pub height: f32,
  pub pos_x: f32,
  pub pos_y: f32,
}

impl Component for Player {
  type Storage = DenseVecStorage<Self>;
}
