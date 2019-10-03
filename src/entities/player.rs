use amethyst::{
//  assets::Handle,
  core::transform::Transform,
  ecs::prelude::World,
  prelude::Builder,
//  renderer::{
//      SpriteRender,
// SpriteSheet,
//    Transparent
//  },
};

use crate::components::Player;
//sprite_sheet_handle: Handle<SpriteSheet>
pub fn init_player(world: &mut World) {
  let mut local_transform = Transform::default();
  local_transform.set_translation_xyz(0.0, 0.0, 1.0);

  // let sprite_render = SpriteRender {
  //   sprite_sheet: sprite_sheet_handle.clone(),
  //   sprite_number: 0,
  // };

  world
    .create_entity()
    // .with(sprite_render)
    .with(Player {
      width: 10.0,
      height: 10.0,
      pos_x: 0.0,
      pos_y: 0.0,
    })
    .with(local_transform)
//    .with(Transparent)
    .build();
}
