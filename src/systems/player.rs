
use amethyst::{
    ecs::prelude::{Entities, Entity, LazyUpdate},
    core::{Transform},
    input::{InputHandler, StringBindings},
    ecs::{Read, WriteStorage, ReadStorage, System, Join, ReadExpect},
    renderer::{SpriteRender, Transparent},
};

use crate::{
    components::{Player, Shot},
    bullet_hello::{ARENA_HEIGHT, ARENA_MIN_X, ARENA_MIN_Y, ARENA_WIDTH, GAME_HEIGHT},
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
            let vertical = input.axis_value("vertical");
            let shooting_action = input.action_is_down("shoot").unwrap_or(false);
            println!("Translate: {:?}", transform.translation().y);
            if shooting_action {
                println!("shooting");
                shoot(&entities,
                      &sprite,
                      &lazy_update,
                      transform.translation().x,
                      transform.translation().y,
                      transform.translation().z,
                );
            }

            transform.move_right(horizontal);
            if let Some(mv_amount) = vertical {
                let scaled_amount = 1.2 * mv_amount as f32;
                let player_y = transform.translation().y;
                transform.set_translation_y(
                    (player_y + scaled_amount)
                        .min(GAME_HEIGHT * 1.2)
                        .max(0.0 - GAME_HEIGHT * 1.2),
                );
            }
        }
    }
}

pub fn shoot(
    entities: &Entities,
    shot_resource: &ReadExpect<SpriteResource>,
    lazy_update: &ReadExpect<LazyUpdate>,
    t_x: f32,
    t_y: f32,
    t_z: f32,
) {

    let fired_shot: Entity = entities.create();

    let mut local_transform = Transform::default();
    local_transform.set_translation_xyz(
        t_x,
        t_y,
        t_z,
    );

    let sprite_render = SpriteRender {
        sprite_sheet: shot_resource.sprite_sheet.clone(),
        sprite_number: 1,
    };

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