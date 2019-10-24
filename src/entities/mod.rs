mod player;
mod enemy;
mod cloud;
//mod level;

pub use self::{
    enemy::init_enemy,
    player::init_player,
    cloud::init_cloud,
//    level::init_level
};
