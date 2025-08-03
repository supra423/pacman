use crate::constants::*;
use crate::game_objects::*;
use crate::map_operations::*;
use crate::pacman_methods::*;
use macroquad::input;
use macroquad::prelude::*;
use std::time::{Duration, Instant};

pub async fn run() {
    let map_image: Texture2D = load_texture("assets/pacmaze2.png").await.unwrap();
    let pacman_close: Texture2D = load_texture("assets/pacman_close.png").await.unwrap();
    let pacman_open: Texture2D = load_texture("assets/pacman_open.png").await.unwrap();
    let pacman_half: Texture2D = load_texture("assets/pacman_half.png").await.unwrap();

    let mut game_map = RAW_MAP;

    // for input buffering
    let mut next_direction = vec2(0.0, 0.0);

    // time between frames
    // let FRAME_TIME = 1.0 / FPS;

    // for animations and switching between modes of ghosts
    let mut timer = 0;

    // for frame limiting
    let frame_duration = Duration::from_secs_f64(FRAME_TIME as f64);

    // defining the entities, their initial positions, and speed
    // the ghosts are already provided with a direction at the start
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
        pacman.move_character(pacman.direction);

        blinky.move_character(blinky.direction);
        inky.move_character(inky.direction);
        pinky.move_character(pinky.direction);
        clyde.move_character(clyde.direction);

        let (pacman_row, pacman_col) = convert_pos_to_index(&pacman.position);
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

        // checks if character goes through tunnel, character goes right out of the other side
        pacman.go_to_other_side();

        blinky.go_to_other_side();
        inky.go_to_other_side();
        pinky.go_to_other_side();
        clyde.go_to_other_side();

        draw_circle(blinky.position.x, blinky.position.y, blinky.size, RED);
        draw_circle(pinky.position.x, pinky.position.y, blinky.size, PINK);
        draw_circle(clyde.position.x, clyde.position.y, blinky.size, ORANGE);
        draw_circle(inky.position.x, inky.position.y, blinky.size, BLUE);

        // draw_pacman(
        //     &pacman,
        //     &pacman_open,
        //     &pacman_close,
        //     &pacman_half,
        //     timer,
        //     pacman_is_colliding,
        // );
        pacman.draw(
            &pacman_open,
            &pacman_close,
            &pacman_half,
            timer,
            pacman_is_colliding,
        );

        debug_texts(&pacman, pacman_col, pacman_row, pacman_is_colliding);
        game_map = pacman_food_eat(game_map, pacman_col, pacman_row);
        if timer == u32::MAX {
            // reset timer if it exceeds
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
