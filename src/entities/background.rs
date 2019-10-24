use amethyst::{
    assets::Handle,
    core::transform::Transform,
    ecs::prelude::World,
    prelude::Builder,
    renderer::{SpriteRender, SpriteSheet, Transparent},
};

use crate::{
    components::Background,
};

pub fn init_background(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut local_transform = Transform::default();


    local_transform.set_translation_xyz(
        0.0,
        0.0,
        0.9,
    );

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 0,
    };

    world
        .create_entity()
        .with(sprite_render)
        .with(Background {width: 960.0, height: 720.0})
        .with(local_transform)
        .with(Transparent)
        .build();
}
