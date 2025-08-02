use crate::constants::*;
use crate::game_objects::*;
use crate::map_operations::*;
use crate::pacman_functions::*;
use macroquad::prelude::*;
use std::time::{Duration, Instant};

pub async fn run() {
    let map_image: Texture2D = load_texture("assets/pacmaze2.png").await.unwrap();
    let pacman_close: Texture2D = load_texture("assets/pacman_close.png").await.unwrap();
    let pacman_open: Texture2D = load_texture("assets/pacman_open.png").await.unwrap();
    let pacman_half: Texture2D = load_texture("assets/pacman_half.png").await.unwrap();
    let mut game_map = RAW_MAP;
    let mut input_buffer = vec2(0.0, 0.0);
    let frame_time = 1.0 / FPS;
    let mut timer = 0;

    // for frame limiting
    let frame_duration = Duration::from_secs_f64(1.0 / FPS as f64);

    // entities
    let mut pacman = PacMan::new(vec2(CENTER.x, CENTER.y + 256.0), 300.0);
    let mut blinky = Ghost::new(vec2(CENTER.x, CENTER.y - 128.0), 375.0, vec2(1.0, 0.0));
    let mut inky = Ghost::new(vec2(CENTER.x, CENTER.y - 128.0), 375.0, vec2(-1.0, 0.0));
    let mut pinky = Ghost::new(vec2(CENTER.x, CENTER.y - 128.0), 375.0, vec2(1.0, 0.0));
    let mut clyde = Ghost::new(vec2(CENTER.x, CENTER.y - 128.0), 375.0, vec2(-1.0, 0.0));
    loop {
        let start = Instant::now();
        timer += 1;

        draw_elements(game_map, &map_image);

        if let Some(direction) = handle_controls() {
            input_buffer = direction;
        }

        if pacman.direction != input_buffer {
            if can_move_to_direction(pacman.position, input_buffer, game_map) {
                pacman.position = centered_coordinates(pacman.position);
                pacman.direction = input_buffer;
            }
        } else {
            pacman.direction = input_buffer;
        }

        pacman.position += pacman.direction * pacman.speed * frame_time;

        blinky.position += blinky.direction * blinky.speed * frame_time;
        inky.position += inky.direction * inky.speed * frame_time;
        pinky.position += pinky.direction * pinky.speed * frame_time;
        clyde.position += clyde.direction * clyde.speed * frame_time;

        let (pacman_row, pacman_col) = convert_pos_to_index(pacman.position);
        let mut pacman_is_colliding = false;
        if collision_checking_offset(&Entity::PacMan(&pacman), game_map) {
            pacman_is_colliding = true;
        }

        let mut blinky_is_colliding = false;
        if collision_checking_offset(&Entity::Ghost(&blinky), game_map) {
            blinky_is_colliding = true;
        }
        let mut inky_is_colliding = false;
        if collision_checking_offset(&Entity::Ghost(&inky), game_map) {
            inky_is_colliding = true;
        }
        let mut pinky_is_colliding = false;
        if collision_checking_offset(&Entity::Ghost(&pinky), game_map) {
            pinky_is_colliding = true;
        }
        let mut clyde_is_colliding = false;
        if collision_checking_offset(&Entity::Ghost(&clyde), game_map) {
            clyde_is_colliding = true;
        }

        if pacman_is_colliding {
            pacman.position = centered_coordinates(pacman.position);
        }

        if blinky_is_colliding {
            blinky.direction = frightened_move(
                centered_coordinates(blinky.position),
                blinky.direction,
                game_map,
            );
            blinky.position = centered_coordinates(blinky.position);
        }

        if inky_is_colliding {
            inky.direction = frightened_move(
                centered_coordinates(inky.position),
                inky.direction,
                game_map,
            );
            inky.position = centered_coordinates(inky.position);
        }
        if pinky_is_colliding {
            pinky.direction = frightened_move(
                centered_coordinates(pinky.position),
                pinky.direction,
                game_map,
            );
            pinky.position = centered_coordinates(pinky.position);
        }
        if clyde_is_colliding {
            clyde.direction = frightened_move(
                centered_coordinates(clyde.position),
                clyde.direction,
                game_map,
            );
            clyde.position = centered_coordinates(clyde.position);
        }

        if pacman.position.x > 1036.0 {
            pacman.position.x = 204.0;
        } else if pacman.position.x < 204.0 {
            pacman.position.x = 1036.0;
        }

        if blinky.position.x > 1036.0 {
            blinky.position.x = 204.0;
        } else if blinky.position.x < 204.0 {
            blinky.position.x = 1036.0;
        }
        if inky.position.x > 1036.0 {
            inky.position.x = 204.0;
        } else if inky.position.x < 204.0 {
            inky.position.x = 1036.0;
        }
        if pinky.position.x > 1036.0 {
            pinky.position.x = 204.0;
        } else if pinky.position.x < 204.0 {
            pinky.position.x = 1036.0;
        }
        if clyde.position.x > 1036.0 {
            clyde.position.x = 204.0;
        } else if clyde.position.x < 204.0 {
            clyde.position.x = 1036.0;
        }

        draw_circle(blinky.position.x, blinky.position.y, blinky.size, RED);
        draw_circle(pinky.position.x, pinky.position.y, blinky.size, PINK);
        draw_circle(inky.position.x, inky.position.y, blinky.size, BLUE);
        draw_circle(clyde.position.x, clyde.position.y, blinky.size, ORANGE);

        draw_pacman(
            &pacman,
            &pacman_open,
            &pacman_close,
            &pacman_half,
            timer,
            pacman_is_colliding,
        );

        debug_texts(&pacman, pacman_col, pacman_row, pacman_is_colliding);
        game_map = pacman_food_eat(game_map, pacman_col, pacman_row);
        if timer == u32::MAX {
            // reset value if it exceeds
            timer = 0;
        }
        // keeping this for debugging some stuff
        // std::thread::sleep(std::time::Duration::from_millis(30));
        let elapsed = start.elapsed();
        if elapsed < frame_duration {
            std::thread::sleep(frame_duration - elapsed);
        }
        next_frame().await;
    }
}
