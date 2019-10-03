
use amethyst::{
    ecs::{ReadExpect, System},
};

use crate::{
    components::{Player},
};

pub struct PlayerSystem;

impl<'s> System<'s> for PlayerSystem {
    type SystemData = (
        ReadExpect<'s, Player>,
    );

    fn run(&mut self, player: Self::SystemData) {

    }
}

