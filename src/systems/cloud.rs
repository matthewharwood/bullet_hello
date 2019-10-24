
use amethyst::{
    ecs::{ReadStorage, System},
};

use crate::{
    components::{Cloud},
};

pub struct CloudSystem;

impl<'s> System<'s> for CloudSystem {
    type SystemData = (
        ReadStorage<'s, Cloud>,
    );

    fn run(&mut self, _cloud: Self::SystemData) {

    }
}

