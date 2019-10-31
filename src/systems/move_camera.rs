use amethyst::{
    ecs::prelude::{Entities, Entity},
    core::{Transform},
    input::{InputHandler, StringBindings},
    ecs::{Read, WriteStorage, ReadStorage, System, Join},
    renderer::{Camera},
};

use crate::{
    bullet_hello::{ARENA_HEIGHT, ARENA_MIN_X, ARENA_MIN_Y, ARENA_WIDTH},
};

pub struct MoveCameraSystem;

impl<'s> System<'s> for MoveCameraSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Camera>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, players, input): Self::SystemData) {
        for (player, transform) in (&players, &mut transforms).join() {
            let vertical = input.axis_value("vertical").unwrap_or(0.0);

            transform.move_up(vertical);
        }
    }
}