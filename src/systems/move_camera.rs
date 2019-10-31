use amethyst::{
    ecs::prelude::{Entities, Entity},
    core::{Transform},
    input::{InputHandler, StringBindings},
    ecs::{Read, WriteStorage, ReadStorage, System, Join},
    renderer::{Camera},
};

use crate::{
    bullet_hello::{GAME_HEIGHT, ARENA_MIN_X, ARENA_MIN_Y, ARENA_WIDTH},
};

pub struct MoveCameraSystem;

impl<'s> System<'s> for MoveCameraSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Camera>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, cameras, input): Self::SystemData) {
        for (camera, transform) in (&cameras, &mut transforms).join() {
            let movement = input.axis_value("vertical");

            if let Some(mv_amount) = movement {
                let scaled_amount = 1.2 * mv_amount as f32;
                let camera_y = transform.translation().y;
                transform.set_translation_y(
                    (camera_y + scaled_amount)
                        .min(GAME_HEIGHT * 1.2)
                        .max(0.0 - GAME_HEIGHT * 1.2),

                );
            }
        }
    }
}