use amethyst::{
    core::{Transform},
    core::timing::Time,
    input::{InputHandler, StringBindings},
    ecs::{Read, WriteStorage, ReadStorage, System, Join},
//    ecs::prelude::{Entities},
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
        Read<'s, Time>,
//        Entities<'s>,
    );

    fn run(&mut self, (mut transforms, shots, time): Self::SystemData) {
        for (transform, shot) in (&mut transforms, &shots).join() {
            transform.prepend_translation_y(shot.velocity[1] * time.delta_seconds());
        }
    }
}

