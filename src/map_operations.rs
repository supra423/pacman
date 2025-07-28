use crate::constants::*;
use crate::game_objects::*;
use macroquad::prelude::*;

pub fn load_food(map: [[u8; COLS]; ROWS]) -> Vec<FoodPellet> {
    let mut map_food = Vec::new();
    for i in 0..ROWS {
        for j in 0..COLS {
            if map[i][j] == 2 {
                let food_pellet = FoodPellet {
                    position: vec2(
                        BOARD_TOP_LEFT_COORDS.y + (TILE_SIZE * i as f32) + 16.0,
                        BOARD_TOP_LEFT_COORDS.x + (TILE_SIZE * j as f32) + 16.0,
                    ),
                    size: 4.0,
                    power_up: false,
                };
                map_food.push(food_pellet);
            } else if map[i][j] == 3 {
                let power_pellet = FoodPellet {
                    position: vec2(
                        BOARD_TOP_LEFT_COORDS.y + (TILE_SIZE * i as f32) + 16.0,
                        BOARD_TOP_LEFT_COORDS.x + (TILE_SIZE * j as f32) + 16.0,
                    ),
                    size: 10.0,
                    power_up: true,
                };
                map_food.push(power_pellet);
            }
        }
    }
    map_food
}

pub fn draw_elements(map: [[u8; COLS]; ROWS], map_image: &Texture2D) {
    draw_texture_ex(
        &map_image,
        BOARD_TOP_LEFT_COORDS.x,
        BOARD_TOP_LEFT_COORDS.y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(BOARD_DIMENSIONS.x, BOARD_DIMENSIONS.y)),
            ..Default::default()
        },
    );
    let foods = load_food(map);
    for food in &foods {
        draw_poly(food.position.y, food.position.x, 4, food.size, 45.0, WHITE);
    }
}

pub fn draw_characters(pacman: &PacMan) {
    draw_circle(pacman.position.x, pacman.position.y, pacman.size, YELLOW);
}

pub fn collision_check(pacman_pos: Vec2) -> bool {
    let row = (((pacman_pos.x) - BOARD_TOP_LEFT_COORDS.x) / TILE_SIZE).floor() as usize;
    let col = (((pacman_pos.y) - BOARD_TOP_LEFT_COORDS.y) / TILE_SIZE).floor() as usize;

    if RAW_MAP[col][row] == 1 { true } else { false }
}

pub fn collision_checking_offset(pacman: &PacMan) -> bool {
    if pacman.direction == vec2(1.0, 0.0)
        && collision_check(vec2(pacman.position.x + 12.0, pacman.position.y))
    {
        true
    } else if pacman.direction == vec2(-1.0, 0.0)
        && collision_check(vec2(pacman.position.x - 12.0, pacman.position.y))
    {
        true
    } else if pacman.direction == vec2(0.0, 1.0)
        && collision_check(vec2(pacman.position.x, pacman.position.y + 12.0))
    {
        true
    } else if pacman.direction == vec2(0.0, -1.0)
        && collision_check(vec2(pacman.position.x, pacman.position.y - 12.0))
    {
        true
    } else {
        false
    }
}

pub fn handle_controls() -> Option<Vec2> {
    if is_key_pressed(KeyCode::Right) || is_key_pressed(KeyCode::D) {
        Some(vec2(1.0, 0.0))
    } else if is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::A) {
        Some(vec2(-1.0, 0.0))
    } else if is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S) {
        Some(vec2(0.0, 1.0))
    } else if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W) {
        Some(vec2(0.0, -1.0))
    } else if is_key_pressed(KeyCode::Space) {
        Some(vec2(0.0, 0.0))
    } else {
        None
    }
}

pub fn pacman_food_eat(mut map: [[u8; COLS]; ROWS], col: usize, row: usize) -> [[u8; COLS]; ROWS] {
    if map[col][row] == 2 || map[col][row] == 3 {
        map[col][row] = 0;
    }
    return map;
}
pub fn can_move_to_direction(col: usize, row: usize, direction: Vec2) -> bool {
    if direction == vec2(1.0, 0.0) && RAW_MAP[col][row + 1] == 1 {
        false
    } else if direction == vec2(-1.0, 0.0) && RAW_MAP[col][row - 1] == 1 {
        false
    } else if direction == vec2(0.0, 1.0) && RAW_MAP[col + 1][row] == 1 {
        false
    } else if direction == vec2(0.0, -1.0) && RAW_MAP[col - 1][row] == 1 {
        false
    } else {
        true
    }
}

pub fn debug_texts(pacman: &PacMan, col: usize, row: usize, colliding: bool) {
    let pacman_pos_string = &pacman.position.to_string();
    let col_string = &col.to_string();
    let row_string = &row.to_string();

    if colliding {
        let collision_text = "COLLIDING";
        draw_text(&collision_text, 50.0, 25.0, 15.0, YELLOW);
    }

    draw_text(pacman_pos_string, 50.0, 35.0, 15.0, YELLOW);
    draw_text(row_string, 50.0, 50.0, 15.0, YELLOW);
    draw_text(col_string, 50.0, 65.0, 15.0, YELLOW);
}

pub fn centered_coordinates(row: f32, col: f32) -> Vec2 {
    let centered_x = ((row * TILE_SIZE) + BOARD_TOP_LEFT_COORDS.x) + 16.0;
    let centered_y = ((col * TILE_SIZE) + BOARD_TOP_LEFT_COORDS.y) + 16.0;
    vec2(centered_x, centered_y)
}
