
use amethyst::{
    ecs::{ReadStorage, System},
};

use crate::{
    components::{Enemy},
};

pub struct EnemySystem;

impl<'s> System<'s> for EnemySystem {
    type SystemData = (
        ReadStorage<'s, Enemy>,
    );

    fn run(&mut self, _enemy: Self::SystemData) {

    }
}

