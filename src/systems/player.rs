
use amethyst::{
    ecs::prelude::{Entities, Entity, LazyUpdate},
    core::{Transform},
    input::{InputHandler, StringBindings},
    ecs::{Read, WriteStorage, ReadStorage, System, Join, ReadExpect},
    renderer::{SpriteRender, Transparent},
};

use crate::{
    components::{Player, Shot},
    bullet_hello::{ARENA_HEIGHT, ARENA_MIN_X, ARENA_MIN_Y, ARENA_WIDTH},
    resources::SpriteResource,
};


pub struct PlayerSystem;

impl<'s> System<'s> for PlayerSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        Read<'s, InputHandler<StringBindings>>,
        Entities<'s>,
        ReadExpect<'s, SpriteResource>,
        ReadExpect<'s, LazyUpdate>,
    );

    fn run(&mut self, (mut transforms, players, input, entities, sprite, lazy_update): Self::SystemData) {
        for (player, transform) in (&players, &mut transforms).join() {
            let horizontal = input.axis_value("horizontal").unwrap_or(0.0);
            let vertical = input.axis_value("vertical").unwrap_or(0.0);
            let shooting_action = input.action_is_down("shoot").unwrap_or(false);

            if shooting_action {
                println!("shooting");
                shoot(&entities,
                      &sprite,
                      &lazy_update);
            }

            transform.move_right(horizontal);
            transform.move_up(vertical);
        }
    }
}

pub fn shoot(
    entities: &Entities,
    shot_resource: &ReadExpect<SpriteResource>,
    lazy_update: &ReadExpect<LazyUpdate>) {

    let fired_shot: Entity = entities.create();

    let mut local_transform = Transform::default();
    local_transform.set_translation_xyz(
        ARENA_MIN_X + (ARENA_WIDTH / 2.0),
        ARENA_MIN_Y + (ARENA_HEIGHT / 6.0),
        0.9,
    );

    let sprite_render = SpriteRender {
        sprite_sheet: shot_resource.sprite_sheet.clone(),
        sprite_number: 1,
    };
    println!("shots fired");
    lazy_update.insert(fired_shot, sprite_render);
    lazy_update.insert(fired_shot, Shot {
        width: 20.0,
        height: 20.0,
        velocity: [0.0, 60.0],
//        hitbox_back: 0.0,
//        hitbox_front: 0.0,
//        hitbox_side: 0.0,
    });
    lazy_update.insert(fired_shot, local_transform);
    lazy_update.insert(fired_shot, Transparent);

}