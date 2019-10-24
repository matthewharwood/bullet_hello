use amethyst::{
    ecs::{ReadStorage, System},
};

use crate::{
    components::{Background},
};

pub struct BackgroundSystem;

impl<'s> System<'s> for BackgroundSystem {
    type SystemData = (
        ReadStorage<'s, Background>,
    );

    fn run(&mut self, _cloud: Self::SystemData) {
    }
}

