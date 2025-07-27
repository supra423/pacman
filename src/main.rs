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
    let frame_time = 1.0 / FPS;
    let mut pacman = PacMan {
        position: vec2(CENTER.x, CENTER.y + 256.0),
        size: TILE_SIZE,
        direction: vec2(0.0, 0.0),
        speed: 300.0,
        powered_up: false,
    };
    let mut pacman_collision_checker = PacMan {
        position: vec2(CENTER.x, CENTER.y + 256.0),
        size: TILE_SIZE / 4.0,
        direction: vec2(0.0, 0.0),
        speed: 300.0,
        powered_up: false,
    };
    loop {
        draw_elements(game_map, &map_image);
        let row = ((pacman_collision_checker.position.x - BOARD_TOP_LEFT_COORDS.x) / TILE_SIZE)
            .floor() as usize;
        let col = ((pacman_collision_checker.position.y - BOARD_TOP_LEFT_COORDS.y) / TILE_SIZE)
            .floor() as usize;
        if let Some(direction) = handle_controls() {
            input_buffer = direction;
        }

        // note, one thing I noticed is that, move confirmation must be done before this
        // line: pacman_collision_checker.position += ....
        // let pacman_collision_pos = (
        //     pacman_collision_checker.position.x,
        //     pacman_collision_checker.position.y,
        // );

        pacman_collision_checker.position +=
            pacman_collision_checker.direction * pacman_collision_checker.speed * frame_time;

        let mut colliding = false;
        let walls = load_walls(game_map);
        if collision_checking_offset(&pacman_collision_checker, &walls) {
            colliding = true;
        }

        if pacman_collision_checker.direction != input_buffer {
            if can_move_to_direction(row, col, input_buffer) {
                pacman_collision_checker.position =
                    centered_coordinates(pacman_collision_checker.position);
                pacman_collision_checker.direction = input_buffer;
            }
        } else {
            pacman_collision_checker.direction = input_buffer;
        }

        if colliding {
            pacman_collision_checker.position.x = pacman.position.x;
            pacman_collision_checker.position.y = pacman.position.y;
        }

        if pacman_collision_checker.position.x > 1028.0 {
            pacman_collision_checker.position.x = 204.0;
        } else if pacman_collision_checker.position.x < 204.0 {
            pacman_collision_checker.position.x = 1028.0;
        }

        pacman.position = vec2(
            (pacman_collision_checker.position.x).floor(),
            (pacman_collision_checker.position.y).floor(),
        );
        draw_characters(&pacman);
        debug_texts(&pacman_collision_checker, row, col, colliding);
        game_map = pacman_food_eat(game_map, row, col);
        next_frame().await;
    }
}
