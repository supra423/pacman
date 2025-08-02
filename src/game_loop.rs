use crate::constants::*;
use crate::game_objects::*;
use crate::map_operations::*;
use crate::pacman_functions::*;
use macroquad::input;
use macroquad::prelude::*;
use std::time::{Duration, Instant};

pub async fn run() {
    let map_image: Texture2D = load_texture("assets/pacmaze2.png").await.unwrap();
    let pacman_close: Texture2D = load_texture("assets/pacman_close.png").await.unwrap();
    let pacman_open: Texture2D = load_texture("assets/pacman_open.png").await.unwrap();
    let pacman_half: Texture2D = load_texture("assets/pacman_half.png").await.unwrap();
    let mut game_map = RAW_MAP;
    let mut next_direction = vec2(0.0, 0.0);
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
            next_direction = direction;
        }

        if pacman.direction != next_direction {
            if can_move_to_direction(pacman.position, next_direction, game_map) {
                pacman.position = centered_coordinates(pacman.position);
                pacman.direction = next_direction;
            }
        }

        // position calculation
        pacman.position = update_character_position(&Entity::PacMan(&pacman), frame_time);

        blinky.position = update_character_position(&Entity::Ghost(&blinky), frame_time);
        inky.position = update_character_position(&Entity::Ghost(&inky), frame_time);
        pinky.position = update_character_position(&Entity::Ghost(&pinky), frame_time);
        clyde.position = update_character_position(&Entity::Ghost(&clyde), frame_time);

        let (pacman_row, pacman_col) = convert_pos_to_index(pacman.position);
        let mut pacman_is_colliding = false;
        if collision_checking_offset(&Entity::PacMan(&pacman), game_map) {
            pacman_is_colliding = true;
            pacman.position = centered_coordinates(pacman.position);
        }

        // frightened position update (TEMPORARY)
        (blinky.position, blinky.direction) = update_frightened_position(&blinky, game_map);
        (inky.position, inky.direction) = update_frightened_position(&inky, game_map);
        (pinky.position, pinky.direction) = update_frightened_position(&pinky, game_map);
        (clyde.position, clyde.direction) = update_frightened_position(&clyde, game_map);

        // if character goes through tunnel, character goes right out of the other side
        pacman.position.x = go_to_other_side(&Entity::PacMan(&pacman));

        blinky.position.x = go_to_other_side(&Entity::Ghost(&blinky));
        inky.position.x = go_to_other_side(&Entity::Ghost(&inky));
        pinky.position.x = go_to_other_side(&Entity::Ghost(&pinky));
        clyde.position.x = go_to_other_side(&Entity::Ghost(&clyde));

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
