mod constants;
mod entity_methods;
mod game_loop;
mod game_objects;
mod ghost_methods;
mod map_operations;
mod pacman_methods;

use crate::constants::*;
use crate::game_loop::*;
#[macroquad::main(window_conf())]
async fn main() {
    run().await;
}
