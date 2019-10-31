use amethyst::{
  assets::{AssetStorage, Handle, Loader},
  core::transform::Transform,
  ecs::prelude::{Dispatcher, DispatcherBuilder, Entity, World},
  prelude::*,
  renderer::{
    formats::texture::ImageFormat, Camera, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture,
  },
  input::{VirtualKeyCode, is_key_down},
};


use crate::{
  entities::init_enemy,
  entities::init_player,
  entities::init_cloud,
  entities::init_background,
  entities::init_shot,
//  entities::init_level,
  systems,
  resources::SpriteResource,
};


pub const GAME_WIDTH: f32 = 360.0;
pub const GAME_HEIGHT: f32 = 270.0;
pub const ARENA_MIN_Y: f32 = 0.0;
pub const ARENA_MAX_Y: f32 = GAME_HEIGHT - ARENA_MIN_Y;
pub const ARENA_MIN_X: f32 = GAME_WIDTH / 8.0;
pub const ARENA_MAX_X: f32 = GAME_WIDTH - ARENA_MIN_X;
pub const ARENA_HEIGHT: f32 = ARENA_MAX_Y - ARENA_MIN_Y;
pub const ARENA_WIDTH: f32 = ARENA_MAX_X - ARENA_MIN_X;


pub fn initialize_sprite_resource(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) -> SpriteResource {
  let sprite_resource = SpriteResource {
    sprite_sheet: sprite_sheet_handle,
  };

  world.add_resource(sprite_resource.clone());
  sprite_resource
}

pub struct BulletHello {
  dispatcher: Dispatcher<'static, 'static>,
}

pub struct MenuScreen {
  dispatcher: Dispatcher<'static, 'static>,
}

impl Default for BulletHello {
  fn default() -> Self {
    BulletHello {
      dispatcher: DispatcherBuilder::new()
        .with(systems::EnemySystem, "enemy_system", &[])
        .with(systems::CloudSystem, "cloud_system", &[])
        .build(),
    }
  }
}

impl SimpleState for BulletHello {
  fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
    let world = data.world;
    let sprite_sheet_handle_player = load_spritesheet(world, "player.png", "player.ron");
    let sprite_sheet_handle_enemy = load_spritesheet(world, "enemy.png", "enemy.ron");
    let sprite_sheet_handle_cloud = load_spritesheet(world, "cloud.png", "cloud.ron");
    initialize_sprite_resource(world,sprite_sheet_handle_player.clone());
    self.dispatcher.setup(&mut world.res);
    init_camera(world);
    init_player(world, sprite_sheet_handle_player.clone(), 10.0);
    for x in 0..10 {
      let y = x * 100;
      init_enemy(world, sprite_sheet_handle_enemy.clone(), &y);
    }

    init_cloud(world, sprite_sheet_handle_cloud);
//    init_level(world, sprite_sheet_handle_cloud.clone(), resources::LEVEL_ONE)
  }

  fn on_stop(&mut self, data: StateData<GameData>) {
    data.world.delete_all();
  }

  fn handle_event(&mut self, data: StateData<GameData>,
                  event: StateEvent) -> SimpleTrans {
    if let StateEvent::Window(event) = &event {
      if is_key_down(&event, VirtualKeyCode::Escape) {
        // Play the game
        println!("Go to Menu Game");
        return Trans::Pop;
      }
    }
    Trans::None
  }
}

impl Default for MenuScreen {
  fn default() -> Self {
    MenuScreen {
      dispatcher: DispatcherBuilder::new()
          .with(systems::BackgroundSystem, "background_system", &[])
//          .with(systems::PlayerSystem, "player_system", &[])
          .build(),
    }
  }
}

impl SimpleState for MenuScreen {

  fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
    let world = data.world;
    let sprite_sheet_handle_player = load_spritesheet(world, "player.png", "player.ron");
    let sprite_sheet_handle_bg = load_spritesheet(world, "background.png", "background.ron");
    initialize_sprite_resource(world,sprite_sheet_handle_player.clone());
    self.dispatcher.setup(&mut world.res);
    init_camera(world);
    init_background(world, sprite_sheet_handle_bg);
//    init_player(world, sprite_sheet_handle_player.clone(), 5.0);
  }

  fn on_pause(&mut self, data: StateData<GameData>) {
    data.world.delete_all();
  }

  fn on_resume(&mut self, data: StateData<GameData>) {

  }

  fn handle_event(&mut self, data: StateData<GameData>,
                  event: StateEvent) -> SimpleTrans {
    if let StateEvent::Window(event) = &event {
      if is_key_down(&event, VirtualKeyCode::Return) {
        // Play the game
        println!("Play Game");
        return Trans::Push(Box::new(BulletHello::default()));
      }
    }
    Trans::None
  }
}

fn init_camera(world: &mut World) {
  let mut transform = Transform::default();
  transform.set_translation_xyz(GAME_WIDTH * 0.5, GAME_HEIGHT * 0.5, 2.0);

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
