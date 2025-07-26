mod constants;
mod game_objects;
mod map_operations;

use crate::constants::*;
use crate::game_objects::*;
use crate::map_operations::*;

use macroquad::prelude::*;

#[macroquad::main(window_conf())]

async fn main() {
    let map_image: Texture2D = load_texture("assets/pacmaze2.png").await.unwrap();
    let mut game_map = RAW_MAP;
    let mut input_buffer = vec2(0.0, 0.0);
    let mut pacman = PacMan {
        position: vec2(CENTER.x, CENTER.y + 256.0),
        size: TILE_SIZE,
        direction: vec2(0.0, 0.0),
        speed: 240.0,
        powered_up: false,
    };
    let mut pacman_collision_checker = PacMan {
        position: vec2(CENTER.x, CENTER.y + 256.0),
        size: TILE_SIZE / 4.0,
        direction: vec2(0.0, 0.0),
        speed: 240.0,
        powered_up: false,
    };
    loop {
        let frame_time = 1.0 / FPS;

        draw_elements(game_map, &map_image);
        let row = ((pacman_collision_checker.position.x - BOARD_TOP_LEFT_COORDS.x) / TILE_SIZE)
            .floor() as usize;
        let col = ((pacman_collision_checker.position.y - BOARD_TOP_LEFT_COORDS.y) / TILE_SIZE)
            .floor() as usize;

        pacman_collision_checker.position +=
            pacman_collision_checker.direction * pacman_collision_checker.speed * frame_time;

        if let Some(direction) = handle_controls() {
            input_buffer = direction;
        }

        let mut colliding = false;
        let walls = load_walls(game_map);
        // MAIN COLLISION CHECKING
        if pacman_collision_checker.direction == vec2(1.0, 0.0) {
            if collision_check(
                vec2(
                    pacman_collision_checker.position.x + 12.0,
                    pacman_collision_checker.position.y,
                ),
                pacman_collision_checker.size,
                walls,
            ) {
                colliding = true;
            }
        } else if pacman_collision_checker.direction == vec2(-1.0, 0.0) {
            if collision_check(
                vec2(
                    pacman_collision_checker.position.x - 12.0,
                    pacman_collision_checker.position.y,
                ),
                pacman_collision_checker.size,
                walls,
            ) {
                colliding = true;
            }
        } else if pacman_collision_checker.direction == vec2(0.0, 1.0) {
            if collision_check(
                vec2(
                    pacman_collision_checker.position.x,
                    pacman_collision_checker.position.y + 12.0,
                ),
                pacman_collision_checker.size,
                walls,
            ) {
                colliding = true;
            }
        } else if pacman_collision_checker.direction == vec2(0.0, -1.0) {
            if collision_check(
                vec2(
                    pacman_collision_checker.position.x,
                    pacman_collision_checker.position.y - 12.0,
                ),
                pacman_collision_checker.size,
                walls,
            ) {
                colliding = true;
            }
        }

        if colliding {
            pacman_collision_checker.position.x = pacman.position.x;
            pacman_collision_checker.position.y = pacman.position.y;
        }
        if can_move_to_direction(col, row, input_buffer) {
            pacman_collision_checker.direction = input_buffer;
        }

        pacman.position = pacman_collision_checker.position;
        draw_characters(&pacman);
        debug_texts(&pacman_collision_checker, row, col, colliding);
        game_map = pacman_food_eat(game_map, row, col);
        next_frame().await;
    }
}
