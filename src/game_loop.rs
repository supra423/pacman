use crate::constants::*;
use crate::game_objects::*;
use crate::map_operations::*;
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
    let mut timer: u32 = 0;

    // for frame limiting
    let frame_duration = Duration::from_secs_f64(FRAME_TIME as f64);

    // defining the entities, their initial positions, and speed
    // the ghosts are already provided with a direction at the start
    let mut pacman = PacMan::new(vec2(CENTER.x, CENTER.y + 256.0), 300.0);
    let mut blinky = Ghost::new(vec2(CENTER.x, CENTER.y - 128.0), 300.0, vec2(1.0, 0.0));
    let mut inky = Ghost::new(vec2(CENTER.x, CENTER.y - 128.0), 300.0, vec2(0.0, 0.0));
    let mut pinky = Ghost::new(vec2(CENTER.x, CENTER.y - 128.0), 300.0, vec2(0.0, 0.0));
    let mut clyde = Ghost::new(vec2(CENTER.x, CENTER.y - 128.0), 300.0, vec2(0.0, 0.0));

    let mut blinky_previous_pos = convert_pos_to_index(&blinky.position);
    loop {
        let start = Instant::now();
        timer += 1;

        if timer_function(timer, 1) == 0 {
            // println!("{}", timer / 60);
        }
        if timer_function(timer, 5) == 0 {
            // println!("asd");
            inky.direction = vec2(-1.0, 0.0);
        }
        if timer_function(timer, 10) == 0 {
            // println!("asd");
            pinky.direction = vec2(1.0, 0.0);
        }
        if timer_function(timer, 15) == 0 {
            // println!("asd");
            clyde.direction = vec2(-1.0, 0.0);
        }

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
        // inky.move_character(inky.direction);
        // pinky.move_character(pinky.direction);
        // clyde.move_character(clyde.direction);

        let mut pacman_is_colliding = false;
        // if collision_checking_offset(&Entity::PacMan(&pacman), game_map) {
        if Entity::PacMan(&pacman).collision_checking_offset(game_map) {
            pacman_is_colliding = true;
            pacman.position = centered_coordinates(pacman.position);
        }
        // if Entity::Ghost(&blinky).collision_checking_offset(game_map) {
        //     blinky.position = centered_coordinates(blinky.position);
        //     // blinky.direction = vec2(0.0, 0.0);
        // }

        if blinky_previous_pos != convert_pos_to_index(&blinky.position) {
            blinky.can_change_direction = true;
            println!("current: {:?}", blinky_previous_pos);
            println!("previous: {:?}", convert_pos_to_index(&blinky.position));
        }

        if blinky.can_change_direction {
            blinky_previous_pos = convert_pos_to_index(&blinky.position);
            (blinky.position, blinky.direction) = choose_ghost_mode(&blinky, game_map);
            blinky.can_change_direction = false;
        }
        if Entity::Ghost(&blinky).collision_checking_offset(game_map) {
            // blinky.position = centered_coordinates(blinky.position);
            // blinky.direction = vec2(0.0, 0.0);
            blinky.can_change_direction = true;
        }
        //
        // (inky.position, inky.direction) = choose_ghost_mode(&inky, game_map);
        // (pinky.position, pinky.direction) = choose_ghost_mode(&pinky, game_map);
        // (clyde.position, clyde.direction) = choose_ghost_mode(&clyde, game_map);

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

        // println!("{:?}", convert_pos_to_index(&pacman.position));

        pacman.draw(
            &pacman_open,
            &pacman_close,
            &pacman_half,
            timer,
            pacman_is_colliding,
        );
        // amount_of_moves_available(pacman.position, pacman.direction, game_map);

        pacman.debug_texts(pacman_is_colliding);
        game_map = pacman.food_eat(game_map);
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
