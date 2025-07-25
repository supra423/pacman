mod constants;
mod game_objects;
mod map_operations;

use crate::constants::*;
use crate::game_objects::*;
use crate::map_operations::*;

use macroquad::input;
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
        size: TILE_SIZE,
        direction: vec2(0.0, 0.0),
        speed: 240.0,
        powered_up: false,
    };
    let mut moving = true;
    let mut next_direction = vec2(0.0, 0.0);
    loop {
        let mut can_move = true;
        let frame_time = 1.0 / FPS;

        let row = ((pacman_collision_checker.position.x - BOARD_TOP_LEFT_COORDS.x) / TILE_SIZE)
            .floor() as usize;
        let col = ((pacman_collision_checker.position.y - BOARD_TOP_LEFT_COORDS.y) / TILE_SIZE)
            .floor() as usize;

        draw_elements(game_map, &map_image);

        pacman_collision_checker.position +=
            pacman_collision_checker.direction * pacman_collision_checker.speed * frame_time;

        draw_text(
            &pacman_collision_checker.position.to_string(),
            50.0,
            80.0,
            15.0,
            YELLOW,
        );

        let walls = load_walls(game_map);
        if let Some(direction) = handle_controls() {
            input_buffer = direction;
        }
        println!("{}", input_buffer);
        if can_move_to_direction(col, row, input_buffer) {
            pacman_collision_checker.direction = input_buffer;
        }

        let mut blocked = false;
        // AABB collision
        for wall in &walls {
            if is_colliding(
                pacman_collision_checker.position,
                pacman_collision_checker.size,
                vec2(wall.position.y, wall.position.x),
                wall.size,
            ) {
                blocked = true;
                let collision_text = format!("COLLIDING");
                draw_text(&collision_text, 50.0, 25.0, 15.0, YELLOW);
                break;
            }
        }

        // if can_move {
        //     pacman_collision_checker.direction = input_buffer;
        // }

        if blocked {
            pacman_collision_checker.position.x = pacman.position.x;
            pacman_collision_checker.position.y = pacman.position.y;
            // pacman_collision_checker.direction = vec2(0.0, 0.0);
            // input_buffer = vec2(0.0, 0.0);
            // moving = false;
        } else {
            // pacman_collision_checker.direction = input_buffer;

            // print!("{}, ", pacman_collision_checker.direction);
            // println!("{}", input_buffer);
        }
        // println!("{}", next_direction);

        pacman.position = pacman_collision_checker.position;
        draw_characters(&pacman);
        // draw_circle(
        //     pacman_collision_checker.position.x,
        //     pacman_collision_checker.position.y,
        //     pacman_collision_checker.size / 2.0_f32.sqrt(),
        //     WHITE,
        // );

        draw_text(&pacman.position.to_string(), 50.0, 35.0, 15.0, YELLOW);
        draw_text(&col.to_string(), 50.0, 50.0, 15.0, YELLOW);
        draw_text(&row.to_string(), 50.0, 65.0, 15.0, YELLOW);
        game_map = pacman_food_eat(game_map, &pacman);
        next_frame().await;
    }
}
