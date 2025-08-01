mod constants;
mod game_loop;
mod game_objects;
mod map_operations;
mod pacman_functions;

use crate::constants::*;
use crate::game_loop::*;
#[macroquad::main(window_conf())]
async fn main() {
    run().await;
}
