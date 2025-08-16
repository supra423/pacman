use crate::constants::*;
use crate::game_objects::*;
use crate::map_operations::*;
use macroquad::prelude::*;
use std::time::{Duration, Instant};

pub async fn run() {
    let map_image: Texture2D = load_texture("assets/pacmaze2.png").await.unwrap();
    let pacman_close: Texture2D = load_texture("assets/pacman_close.png").await.unwrap();
    let pacman_open: Texture2D = load_texture("assets/pacman_open.png").await.unwrap();
    let pacman_half: Texture2D = load_texture("assets/pacman_half.png").await.unwrap();

    let mut game_map = RAW_MAP;

    // for animations and switching between modes of ghosts
    let mut timer: u32 = 0;

    // for frame limiting
    let frame_duration = Duration::from_secs_f64(FRAME_TIME as f64);
    let mut game_level = 1;

    // defining the entities, their initial positions, and speed
    let mut pacman = PacMan::new(vec2(CENTER.x, CENTER.y + 256.0), 300.0);
    // self.position = vec2(CENTER.x, CENTER.y - 32.0);

    let mut blinky = Ghost::new(vec2(CENTER.x, CENTER.y - 32.0), 300.0);
    let mut inky = Ghost::new(vec2(CENTER.x, CENTER.y - 32.0), 300.0);
    let mut pinky = Ghost::new(vec2(CENTER.x, CENTER.y - 32.0), 300.0);
    let mut clyde = Ghost::new(vec2(CENTER.x, CENTER.y - 32.0), 300.0);

    let mut pacman_power_duration: u16 = 360;

    loop {
        let start = Instant::now();
        if timer == 2 {
            std::thread::sleep(Duration::from_secs(3)); // reading this line is funny
        } else {
            if load_food(game_map).is_empty() {
                game_level += 1;
                pacman.reset_values();
                blinky.reset_values();
                inky.reset_values();
                pinky.reset_values();
                clyde.reset_values();
                if blinky.speed < 420.0 {
                    blinky.speed += 30.0;
                    inky.speed += 30.0;
                    pinky.speed += 30.0;
                    clyde.speed += 30.0;
                }
                if pacman_power_duration > 60 {
                    pacman_power_duration -= 60;
                }
                game_map = RAW_MAP;
                timer = 0;
            }
            if !pacman.powered_up {
                if pacman.aabb(&blinky)
                    || pacman.aabb(&inky)
                    || pacman.aabb(&pinky)
                    || pacman.aabb(&clyde)
                {
                    if pacman.lives > 0 {
                        pacman.lives -= 1;

                        pacman.reset_values();
                        blinky.reset_values();
                        inky.reset_values();
                        pinky.reset_values();
                        clyde.reset_values();
                        timer = 0;
                    } else {
                        return;
                    }
                }
            } else {
                pacman.power_up_timer += 1;
                if pacman.power_up_timer <= pacman_power_duration {
                    if pacman.aabb(&blinky) {
                        blinky.teleport_outside_pen();
                    } else if pacman.aabb(&inky) {
                        inky.teleport_outside_pen();
                    } else if pacman.aabb(&pinky) {
                        pinky.teleport_outside_pen();
                    } else if pacman.aabb(&clyde) {
                        clyde.teleport_outside_pen();
                    }
                } else {
                    pacman.powered_up = false;
                    pacman.power_up_timer = 0;
                }
            }

            display_level(game_level);
            // if timer % 60 == 0 {
            // println!("{}", timer / 60);
            // }
            blinky.draw_delay(timer, 2, game_map);
            inky.draw_delay(timer, 300, game_map);
            pinky.draw_delay(timer, 600, game_map);
            clyde.draw_delay(timer, 900, game_map);

            draw_elements(game_map, &map_image);

            if let Some(direction) = handle_controls() {
                pacman.next_direction = direction;
            }

            pacman.change_directions(game_map);

            // position calculation
            pacman.move_character(pacman.direction);

            blinky.move_character(blinky.direction);
            inky.move_character(inky.direction);
            pinky.move_character(pinky.direction);
            clyde.move_character(clyde.direction);

            // collision detection
            pacman.colliding(game_map);

            // checks if character goes through tunnel, character goes right out of the other side

            pacman.position.x = Entity::PacMan(&pacman).go_to_other_side();
            blinky.position.x = Entity::Ghost(&blinky).go_to_other_side();
            inky.position.x = Entity::Ghost(&inky).go_to_other_side();
            pinky.position.x = Entity::Ghost(&pinky).go_to_other_side();
            clyde.position.x = Entity::Ghost(&clyde).go_to_other_side();

            blinky.draw_color_switch(&pacman, RED);
            inky.draw_color_switch(&pacman, BLUE);
            pinky.draw_color_switch(&pacman, PINK);
            clyde.draw_color_switch(&pacman, ORANGE);

            // println!("{:?}", convert_pos_to_index(&pacman.position));

            pacman.draw(
                &pacman_open,
                &pacman_close,
                &pacman_half,
                timer,
                pacman.colliding,
            );
            // amount_of_moves_available(pacman.position, pacman.direction, game_map);

            pacman.draw_score();
            pacman.draw_lives();
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
        timer += 1;
    }
}
