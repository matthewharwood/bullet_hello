use amethyst::{
    core::{Transform},
    core::timing::Time,
    input::{InputHandler, StringBindings},
    ecs::{Read, WriteStorage, ReadStorage, System, Join},
    ecs::prelude::{Entities},
};

use crate::{
    components::{Shot},
    components::{Player},
    bullet_hello::{ARENA_MAX_Y}
};

pub struct ShotSystem;

impl<'s> System<'s> for ShotSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Shot>,
        Read<'s, Time>,
        Entities<'s>,
    );

    fn run(&mut self, (mut transforms, shots, time, entities): Self::SystemData) {
        for (transform, shot, entity) in (&mut transforms, &shots, &*entities).join() {
            if  (transform.translation()[1] - shot.height) > ARENA_MAX_Y {
                entities.delete(entity);
            }
            transform.prepend_translation_y(shot.velocity[1] * time.delta_seconds());
        }
    }
}

