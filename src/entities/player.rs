use amethyst::{
  assets::Handle,
  core::transform::Transform,
  ecs::prelude::World,
  prelude::Builder,
  renderer::{SpriteRender, SpriteSheet, Transparent},
};

use crate::{
  bullet_hello::{ARENA_HEIGHT, ARENA_MIN_X, ARENA_MIN_Y, ARENA_WIDTH},
  components::Player,
};

pub fn init_player(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>, x: f32) {
  let mut local_transform = Transform::default();
  local_transform.set_translation_xyz(
    ARENA_MIN_X + x + (ARENA_WIDTH / 2.0),
    ARENA_MIN_Y + (ARENA_HEIGHT / 6.0),
    0.9,
  );

  let sprite_render = SpriteRender {
    sprite_sheet: sprite_sheet_handle,
    sprite_number: 0,
  };

  world
    .create_entity()
    .with(sprite_render)
    .with(Player {
      width: 10.0,
      height: 10.0
    })
    .with(local_transform)
    .with(Transparent)
    .build();
}
