use amethyst::{
  // assets::{AssetStorage, Loader, Handle},
  core::transform::Transform,
  prelude::*,
  renderer::Camera,
  ecs::prelude::{Dispatcher, DispatcherBuilder, Entity},
};

//use crate::entities::init_player;
//use crate::components::Player;
use crate::{
  entities::init_player,
  systems,
};

pub const GAME_WIDTH: f32 = 360.0;
pub const GAME_HEIGHT: f32 = 270.0;
pub const ARENA_MIN_Y: f32 = 0.0;
pub const ARENA_MAX_Y: f32 = GAME_HEIGHT - ARENA_MIN_Y;
pub const ARENA_MIN_X: f32 = GAME_WIDTH / 8.0;
pub const ARENA_MAX_X: f32 = GAME_WIDTH - ARENA_MIN_X;
pub const ARENA_HEIGHT: f32 = ARENA_MAX_Y - ARENA_MIN_Y;
pub const ARENA_WIDTH: f32 = ARENA_MAX_X - ARENA_MIN_X;
pub const ARENA_SPAWN_OFFSET: f32 = 20.0;

pub struct BulletHello {
  dispatcher: Dispatcher<'static, 'static>,
}

impl Default for BulletHello {
  fn default() -> Self {
    BulletHello{
      dispatcher: DispatcherBuilder::new()
          .with(systems::PlayerSystem, "player_system", &[])
          .build(),
    }
  }
}

impl SimpleState for BulletHello {
  fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
    let world = data.world;
    //  let sprite_sheet_handle = load_spritesheet(world, "spritesheet.png", "spritesheet.ron");
//    world.register::<Player>();
    self.dispatcher.setup(&mut world.res);
    init_camera(world);
    init_player(world);
  }
}

fn init_camera(world: &mut World) {
  let mut transform = Transform::default();
  transform.set_translation_xyz(GAME_WIDTH * 0.5, GAME_HEIGHT * 0.5, 1.0);

  world
    .create_entity()
    .with(Camera::standard_2d(GAME_WIDTH, GAME_HEIGHT))
    .with(transform)
    .build();
}
