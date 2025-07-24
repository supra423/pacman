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
    loop {
        let frame_time = 1.0 / FPS;

        draw_elements(RAW_MAP, &map_image);

        let mut input_buffer = pacman_collision_checker.direction;

        pacman_collision_checker.position +=
            pacman_collision_checker.direction * pacman_collision_checker.speed * frame_time;

        let walls = load_walls(RAW_MAP);
        let mut blocked = false;
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

        if let Some(direction) = pacman_movements() {
            pacman_collision_checker.direction = direction;
            input_buffer = direction;
        }

        if pacman_collision_checker.direction == vec2(1.0, 0.0)
            || pacman_collision_checker.direction == vec2(-1.0, 0.0)
        {}

        if blocked {
            pacman_collision_checker.position.x = pacman.position.x;
            pacman_collision_checker.position.y = pacman.position.y;
        }
        pacman.position = pacman_collision_checker.position;

        draw_circle(pacman.position.x, pacman.position.y, pacman.size, YELLOW);
        draw_poly_lines(
            pacman.position.x,
            pacman.position.y,
            4,
            pacman.size,
            45.0,
            2.0,
            GREEN,
        );
        draw_circle(
            pacman_collision_checker.position.x,
            pacman_collision_checker.position.y,
            // pacman_collision_checker.size / 2.0_f32.sqrt(),
            10.0,
            WHITE,
        );
        draw_poly_lines(
            pacman_collision_checker.position.x,
            pacman_collision_checker.position.y,
            4,
            pacman_collision_checker.size,
            45.0,
            2.0,
            BLUE,
        );

        draw_text(&pacman.position.to_string(), 50.0, 35.0, 15.0, YELLOW);
        next_frame().await;
    }
}
