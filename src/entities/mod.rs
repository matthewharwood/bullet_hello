mod player;
mod enemy;
mod cloud;
//mod level;
mod background;
mod shot;
pub use self::{
    enemy::init_enemy,
    player::init_player,
    cloud::init_cloud,
    background::init_background,
    shot::init_shot,
//    level::init_level
};
