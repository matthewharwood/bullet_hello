use amethyst::{
    core::{Transform},
    input::{InputHandler, StringBindings},
    ecs::{Read, WriteStorage, ReadStorage, System, Join},
    ecs::prelude::{Entities},
};

use crate::{
    components::{Shot},
    components::{Player, Enemy},
    bullet_hello::{ARENA_MAX_Y}
};

pub struct ShotCollisionSystem;

impl<'s> System<'s> for ShotCollisionSystem {
    type SystemData = (
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Shot>,
        ReadStorage<'s, Enemy>,
        Entities<'s>,
    );

    fn run(&mut self, (transforms, shots, enemies, entities): Self::SystemData) {
        for (shot_transform, shot, shot_entity) in (&transforms, &shots, &*entities).join() {
            for (enemy_transform, enemy, enemy_entity) in (&transforms, &enemies, &*entities).join() {
                let is_colliding = collision(
                    enemy_transform.translation().x,
                    enemy_transform.translation().y,
                    shot_transform.translation().x,
                    shot_transform.translation().y,
                    enemy.width,
                    enemy.height,
                    shot.width,
                    shot.height,
                );
                if is_colliding  {
                    entities.delete(enemy_entity);
                    entities.delete(shot_entity);
                }
            }
        }
    }
}


pub fn collision(mut x1: f32, mut y1: f32, mut x2: f32, mut y2: f32, hitbox_width_1: f32, hitbox_height_1: f32, hitbox_width_2: f32, hitbox_height_2: f32) -> bool {
    x1 -= hitbox_width_1 / 2.0;
    y1 -= hitbox_height_1 / 2.0;
    x2 -= hitbox_width_2 / 2.0;
    y2 -= hitbox_height_2 / 2.0;

    x1 < (x2 + hitbox_width_2) && (x1 + hitbox_width_1) > x2 && y1 < (y2 + hitbox_height_2) && (y1 + hitbox_height_1) > y2
}