use crate::constants::*;
use crate::game_objects::*;
use macroquad::prelude::*;

pub fn load_walls(map: [[u8; COLS]; ROWS]) -> Vec<Wall> {
    let board_top_left_coords_x = BOARD_TOP_LEFT_COORDS.x;
    let board_top_left_coords_y = BOARD_TOP_LEFT_COORDS.y;

    let mut map_walls = Vec::new();
    for i in 0..ROWS {
        for j in 0..COLS {
            if map[i][j] == 1 {
                let wall_object = Wall {
                    position: vec2(
                        board_top_left_coords_y + (TILE_SIZE * i as f32) + 16.0,
                        board_top_left_coords_x + (TILE_SIZE * j as f32) + 16.0,
                    ),
                    size: TILE_SIZE,
                };
                map_walls.push(wall_object);
            }
        }
    }
    map_walls
}

pub fn load_food(map: [[u8; COLS]; ROWS]) -> Vec<FoodPellet> {
    let board_top_left_coords_x = BOARD_TOP_LEFT_COORDS.x;
    let board_top_left_coords_y = BOARD_TOP_LEFT_COORDS.y;

    let mut map_food = Vec::new();
    for i in 0..ROWS {
        for j in 0..COLS {
            if map[i][j] == 2 {
                let food_pellet = FoodPellet {
                    position: vec2(
                        board_top_left_coords_y + (TILE_SIZE * i as f32) + 16.0,
                        board_top_left_coords_x + (TILE_SIZE * j as f32) + 16.0,
                    ),

                    size: 4.0,
                    power_up: false,
                };
                map_food.push(food_pellet);
            } else if map[i][j] == 3 {
                let power_pellet = FoodPellet {
                    position: vec2(
                        board_top_left_coords_y + (TILE_SIZE * i as f32) + 16.0,
                        board_top_left_coords_x + (TILE_SIZE * j as f32) + 16.0,
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
    // let walls = load_walls(map);
    // for wall in &walls {
    //     draw_poly_lines(
    //         wall.position.y,
    //         wall.position.x,
    //         4,
    //         wall.size / 2.0_f32.sqrt(),
    //         45.0,
    //         2.0,
    //         GREEN,
    //     );
    // }
    let foods = load_food(map);
    for food in &foods {
        draw_poly(food.position.y, food.position.x, 4, food.size, 45.0, WHITE);
    }
}

pub fn draw_characters(pacman: &PacMan) {
    draw_circle(pacman.position.x, pacman.position.y, pacman.size, YELLOW);
}

pub fn is_colliding(center_a: Vec2, size_a: f32, center_b: Vec2, size_b: f32) -> bool {
    let half_a = size_a / 2.0;
    let half_b = size_b / 2.0;

    let min_a = center_a - half_a;
    let max_a = center_a + half_a;

    let min_b = center_b - half_b;
    let max_b = center_b + half_b;

    min_a.x < max_b.x && max_a.x > min_b.x && min_a.y < max_b.y && max_a.y > min_b.y
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

pub fn pacman_food_eat(mut map: [[u8; COLS]; ROWS], pacman: &PacMan) -> [[u8; COLS]; ROWS] {
    let row = ((pacman.position.x - BOARD_TOP_LEFT_COORDS.x) / TILE_SIZE) as usize;
    let col = ((pacman.position.y - BOARD_TOP_LEFT_COORDS.y) / TILE_SIZE) as usize;

    if map[col][row] == 2 || map[col][row] == 3 {
        map[col][row] = 0;
    }
    return map;
}
pub fn can_move_to_direction(col: usize, row: usize, direction: Vec2) -> bool {
    // RAW_MAP[y][x] != 1
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
pub fn is_x_aligned(pacman: &PacMan) -> bool {
    pacman.position.x % 16.0 == 0.0
}

// pub fn align_x(pos_x: f32, direction: Vec2) -> f32 {
//     if direction == vec2(1.0, 0.0) {
//         let aligned_x = (pos_x % 16);
//     }
//     // aligned_x
// }
