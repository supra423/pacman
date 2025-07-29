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
    let pacman_close: Texture2D = load_texture("assets/pacman_close.png").await.unwrap();
    let pacman_open: Texture2D = load_texture("assets/pacman_open.png").await.unwrap();
    let pacman_half: Texture2D = load_texture("assets/pacman_half.png").await.unwrap();
    let mut game_map = RAW_MAP;
    let mut input_buffer = vec2(0.0, 0.0);
    let frame_time = 1.0 / FPS;
    let mut previous_pacman = PacMan {
        position: vec2(CENTER.x, CENTER.y + 256.0),
        size: TILE_SIZE,
        direction: vec2(0.0, 0.0),
        speed: 300.0,
        powered_up: false,
    };
    let mut next_pacman = PacMan {
        position: vec2(CENTER.x, CENTER.y + 256.0),
        size: TILE_SIZE,
        direction: vec2(0.0, 0.0),
        speed: 300.0,
        powered_up: false,
    };
    let mut timer = 0;
    loop {
        timer += 1;
        draw_elements(game_map, &map_image);
        if let Some(direction) = handle_controls() {
            input_buffer = direction;
        }

        next_pacman.position += next_pacman.direction * next_pacman.speed * frame_time;

        let row = ((next_pacman.position.x - BOARD_TOP_LEFT_COORDS.x) / TILE_SIZE).floor() as usize;
        let col = ((next_pacman.position.y - BOARD_TOP_LEFT_COORDS.y) / TILE_SIZE).floor() as usize;
        let mut colliding = false;
        if collision_checking_offset(&next_pacman) {
            colliding = true;
        }

        if next_pacman.direction != input_buffer {
            if can_move_to_direction(col, row, input_buffer) {
                next_pacman.position = centered_coordinates(col as f32, row as f32);
                next_pacman.direction = input_buffer;
            }
        } else {
            next_pacman.direction = input_buffer;
        }

        if colliding {
            next_pacman.position.x = previous_pacman.position.x;
            next_pacman.position.y = previous_pacman.position.y;
        }

        if next_pacman.position.x > 1036.0 {
            next_pacman.position.x = 204.0;
        } else if next_pacman.position.x < 204.0 {
            next_pacman.position.x = 1036.0;
        }

        previous_pacman.position = vec2(
            (next_pacman.position.x).floor(),
            (next_pacman.position.y).floor(),
        );
        draw_characters(
            &next_pacman,
            &pacman_open,
            &pacman_close,
            &pacman_half,
            timer,
            colliding,
        );
        debug_texts(&previous_pacman, col, row, colliding);
        game_map = pacman_food_eat(game_map, col, row);
        if timer == 4_294_967_295 {
            timer = 0;
        }
        // keeping this for debugging some stuff
        // std::thread::sleep(std::time::Duration::from_millis(30));
        next_frame().await;
    }
}
