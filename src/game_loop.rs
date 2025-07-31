use crate::constants::*;
use crate::game_objects::*;
use crate::map_operations::*;
use macroquad::prelude::*;

pub async fn run() {
    let map_image: Texture2D = load_texture("assets/pacmaze2.png").await.unwrap();
    let pacman_close: Texture2D = load_texture("assets/pacman_close.png").await.unwrap();
    let pacman_open: Texture2D = load_texture("assets/pacman_open.png").await.unwrap();
    let pacman_half: Texture2D = load_texture("assets/pacman_half.png").await.unwrap();
    let mut game_map = RAW_MAP;
    let mut input_buffer = vec2(0.0, 0.0);
    let frame_time = 1.0 / FPS;
    let mut timer = 0;

    // entities
    let mut previous_pacman = PacMan::new(vec2(CENTER.x, CENTER.y + 256.0), 0.0);
    let mut next_pacman = PacMan::new(vec2(CENTER.x, CENTER.y + 256.0), 300.0);
    let mut blinky = Ghost::new(vec2(CENTER.x, CENTER.y - 128.0), 300.0);
    let mut inky = Ghost::new(vec2(CENTER.x - 64.0, CENTER.y - 32.0), 300.0);
    let mut pinky = Ghost::new(vec2(CENTER.x, CENTER.y - 32.0), 300.0);
    let mut clyde = Ghost::new(vec2(CENTER.x + 64.0, CENTER.y - 32.0), 300.0);
    loop {
        timer += 1;
        draw_elements(game_map, &map_image);
        if let Some(direction) = handle_controls() {
            input_buffer = direction;
        }

        next_pacman.position += next_pacman.direction * next_pacman.speed * frame_time;

        let pacman_row =
            ((next_pacman.position.x - BOARD_TOP_LEFT_COORDS.x) / TILE_SIZE).floor() as usize;
        let pacman_col =
            ((next_pacman.position.y - BOARD_TOP_LEFT_COORDS.y) / TILE_SIZE).floor() as usize;
        let mut pacman_is_colliding = false;
        if collision_checking_offset(&Entity::PacMan(&next_pacman)) {
            pacman_is_colliding = true;
        }

        if next_pacman.direction != input_buffer {
            if can_move_to_direction(next_pacman.position, input_buffer) {
                // next_pacman.position = centered_coordinates(pacman_col as f32, pacman_row as f32);
                next_pacman.position = centered_coordinates(next_pacman.position);
                next_pacman.direction = input_buffer;
            }
        } else {
            next_pacman.direction = input_buffer;
        }

        if pacman_is_colliding {
            // next_pacman.position = centered_coordinates(pacman_col as f32, pacman_row as f32);
            next_pacman.position = centered_coordinates(next_pacman.position);
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
        draw_pacman(
            &next_pacman,
            &pacman_open,
            &pacman_close,
            &pacman_half,
            timer,
            pacman_is_colliding,
        );

        draw_circle(blinky.position.x, blinky.position.y, blinky.size, RED);
        draw_circle(pinky.position.x, pinky.position.y, blinky.size, PINK);
        draw_circle(inky.position.x, inky.position.y, blinky.size, BLUE);
        draw_circle(clyde.position.x, clyde.position.y, blinky.size, ORANGE);
        debug_texts(
            &previous_pacman,
            pacman_col,
            pacman_row,
            pacman_is_colliding,
        );
        game_map = pacman_food_eat(game_map, pacman_col, pacman_row);
        if timer == 4_294_967_295 {
            timer = 0;
        }
        // keeping this for debugging some stuff
        // std::thread::sleep(std::time::Duration::from_millis(30));
        next_frame().await;
    }
}
