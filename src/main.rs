use amethyst::{
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },

    utils::application_root_dir,
};

pub mod components;
pub mod systems;
pub mod resources;
pub mod entities;

mod bullet_hello;

use crate::bullet_hello::{MenuScreen};

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let assets_path = app_root.join("assets");
    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");
    let bindings_config = config_dir.join("bindings.ron");

    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(bindings_config)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)
                        .with_clear([1.0, 0.827, 0.098, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with(systems::ShotSystem, "shot_system", &[])
        .with(systems::PlayerSystem, "player_system", &[])
        .with(systems::ShotCollisionSystem, "shot_collision_system", &[]);


    let mut game = Application::build(assets_path, MenuScreen::default())?.build(game_data)?;
    game.run();

    Ok(())
}
