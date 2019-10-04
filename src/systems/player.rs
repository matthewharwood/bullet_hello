
use amethyst::{
    ecs::{ReadStorage, System},
};

use crate::{
    components::{Player},
};

pub struct PlayerSystem;

impl<'s> System<'s> for PlayerSystem {
    type SystemData = (
        ReadStorage<'s, Player>,
    );

    fn run(&mut self, player: Self::SystemData) {

    }
}

