use amethyst::{
  assets::Handle,
  core::transform::Transform,
  ecs::prelude::World,
  prelude::Builder,
  renderer::{SpriteRender, SpriteSheet, Transparent},
};

use crate::{
  bullet_hello::{ARENA_HEIGHT, ARENA_MIN_X, ARENA_MIN_Y, ARENA_WIDTH},
  components::Cloud,
};

pub fn init_cloud(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
  let mut local_transform = Transform::default();
  local_transform.set_translation_xyz(
    ARENA_MIN_X + (ARENA_WIDTH / 2.0),
    ARENA_MIN_Y + (ARENA_HEIGHT / 2.0),
    0.8,
  );

  let sprite_render = SpriteRender {
    sprite_sheet: sprite_sheet_handle.clone(),
    sprite_number: 0,
  };

  world
    .create_entity()
    .with(sprite_render)
    .with(Cloud {
      width: 10.0,
      height: 10.0,
    })
    .with(local_transform)
    .with(Transparent)
    .build();
}
