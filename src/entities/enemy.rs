use amethyst::{
  assets::Handle,
  core::transform::Transform,
  ecs::prelude::World,
  prelude::Builder,
  renderer::{SpriteRender, SpriteSheet, Transparent},
};

use crate::{
  bullet_hello::{ARENA_HEIGHT, ARENA_MIN_X, ARENA_MIN_Y, ARENA_WIDTH},
  components::Enemy,
};

pub fn init_enemy(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>, y: &i32) {
  let mut local_transform = Transform::default();
  local_transform.set_translation_xyz(
    ARENA_MIN_X +  (ARENA_WIDTH / 2.0),
    ARENA_MIN_Y + (*y as f32) + (ARENA_HEIGHT / 2.0),
    0.9,
  );

  let sprite_render = SpriteRender {
    sprite_sheet: sprite_sheet_handle.clone(),
    sprite_number: 0,
  };

  world
    .create_entity()
    .with(sprite_render)
    .with(Enemy {
      width: 10.0,
      height: 10.0,
    })
    .with(local_transform)
    .with(Transparent)
    .build();
}
