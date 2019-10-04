use amethyst::{
  assets::{AssetStorage, Handle, Loader},
  core::transform::Transform,
  ecs::prelude::{Dispatcher, DispatcherBuilder, Entity},
  prelude::*,
  renderer::{
    formats::texture::ImageFormat, Camera, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture,
  },
};

use crate::{entities::init_player, systems};

pub const GAME_WIDTH: f32 = 360.0;
pub const GAME_HEIGHT: f32 = 270.0;
pub const ARENA_MIN_Y: f32 = 0.0;
pub const ARENA_MAX_Y: f32 = GAME_HEIGHT - ARENA_MIN_Y;
pub const ARENA_MIN_X: f32 = GAME_WIDTH / 8.0;
pub const ARENA_MAX_X: f32 = GAME_WIDTH - ARENA_MIN_X;
pub const ARENA_HEIGHT: f32 = ARENA_MAX_Y - ARENA_MIN_Y;
pub const ARENA_WIDTH: f32 = ARENA_MAX_X - ARENA_MIN_X;

pub struct BulletHello {
  dispatcher: Dispatcher<'static, 'static>,
}

impl Default for BulletHello {
  fn default() -> Self {
    BulletHello {
      dispatcher: DispatcherBuilder::new()
        .with(systems::PlayerSystem, "player_system", &[])
        .build(),
    }
  }
}

impl SimpleState for BulletHello {
  fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
    let world = data.world;
    let sprite_sheet_handle = load_spritesheet(world, "player.png", "player.ron");
    self.dispatcher.setup(&mut world.res);
    init_camera(world);
    init_player(world, sprite_sheet_handle);
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

fn load_spritesheet(
  world: &mut World,
  spritesheet: &str,
  spritesheet_ron: &str,
) -> Handle<SpriteSheet> {
  let texture_handle = {
    let loader = world.read_resource::<Loader>();
    let texture_storage = world.read_resource::<AssetStorage<Texture>>();
    loader.load(
      format!("texture/{}", spritesheet),
      ImageFormat::default(),
      (),
      &texture_storage,
    )
  };

  let loader = world.read_resource::<Loader>();
  let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
  loader.load(
    format!("data/{}", spritesheet_ron),
    SpriteSheetFormat(texture_handle),
    (),
    &sprite_sheet_store,
  )
}
