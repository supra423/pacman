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
        size: 16.0,
        direction: vec2(0.0, 0.0),
        speed: 200.0,
        powered_up: false,
    };
    let mut pacman_collision_checker = PacMan {
        position: vec2(CENTER.x, CENTER.y + 256.0),
        size: 16.0,
        direction: vec2(0.0, 0.0),
        speed: 200.0,
        powered_up: false,
    };
    loop {
        let frame_time = get_frame_time();

        draw_elements(RAW_MAP, &map_image);
        draw_circle(
            pacman.position.x,
            pacman.position.y,
            pacman.size * 2.0,
            YELLOW,
        );

        pacman_collision_checker.position +=
            pacman_collision_checker.direction * pacman.speed * frame_time;
        let mut input_buffer = pacman_collision_checker.direction;
        println!("{:?}", input_buffer);

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
        }

        if blocked {
            pacman_collision_checker.position = pacman.position;
        } else {
            pacman.position = pacman_collision_checker.position;
        }
        pacman.position = pacman_collision_checker.position;
        draw_circle(
            pacman_collision_checker.position.x,
            pacman_collision_checker.position.y,
            10.0,
            WHITE,
        );

        draw_text(&pacman.position.to_string(), 50.0, 35.0, 15.0, YELLOW);
        next_frame().await;
    }
}
