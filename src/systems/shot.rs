use amethyst::{
    core::{Transform},
    input::{InputHandler, StringBindings},
    ecs::{Read, WriteStorage, ReadStorage, System, Join},
};

use crate::{
    components::{Shot},
    components::{Player},
};

pub struct ShotSystem;

impl<'s> System<'s> for ShotSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Shot>,
        ReadStorage<'s, Player>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, shot, players, input): Self::SystemData) {
//        for (player, transform) in (&players, &mut transforms).join() {
//            let horizontal = input.axis_value("horizontal").unwrap_or(0.0);
//            let vertical = input.axis_value("vertical").unwrap_or(0.0);
//
//            transform.move_right(horizontal);
//            transform.move_up(vertical);
//        }
    }
}

