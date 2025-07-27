use crate::constants::*;
use crate::game_objects::*;
use macroquad::prelude::*;

pub fn load_walls(map: [[u8; COLS]; ROWS]) -> Vec<Wall> {
    let mut map_walls = Vec::new();
    for i in 0..ROWS {
        for j in 0..COLS {
            if map[i][j] == 1 {
                let wall_object = Wall {
                    position: vec2(
                        BOARD_TOP_LEFT_COORDS.y + (TILE_SIZE * i as f32) + 16.0,
                        BOARD_TOP_LEFT_COORDS.x + (TILE_SIZE * j as f32) + 16.0,
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

pub fn aabb_collision(center_a: Vec2, size_a: f32, center_b: Vec2, size_b: f32) -> bool {
    let half_a = size_a / 2.0;
    let half_b = size_b / 2.0;

    let min_a = center_a - half_a;
    let max_a = center_a + half_a;

    let min_b = center_b - half_b;
    let max_b = center_b + half_b;

    min_a.x < max_b.x && max_a.x > min_b.x && min_a.y < max_b.y && max_a.y > min_b.y
}

pub fn collision_check(pacman_pos: Vec2, pacman_size: f32, walls: &Vec<Wall>) -> bool {
    for wall in walls {
        if aabb_collision(
            pacman_pos,
            pacman_size,
            vec2(wall.position.y, wall.position.x),
            wall.size,
        ) {
            return true;
        }
    }
    false
}

pub fn collision_checking_offset(pacman: &PacMan, walls: &Vec<Wall>) -> bool {
    if pacman.direction == vec2(1.0, 0.0)
        && collision_check(
            vec2(pacman.position.x + 12.0, pacman.position.y),
            pacman.size,
            walls,
        )
    {
        true
    } else if pacman.direction == vec2(-1.0, 0.0)
        && collision_check(
            vec2(pacman.position.x - 12.0, pacman.position.y),
            pacman.size,
            walls,
        )
    {
        true
    } else if pacman.direction == vec2(0.0, 1.0)
        && collision_check(
            vec2(pacman.position.x, pacman.position.y + 12.0),
            pacman.size,
            walls,
        )
    {
        true
    } else if pacman.direction == vec2(0.0, -1.0)
        && collision_check(
            vec2(pacman.position.x, pacman.position.y - 12.0),
            pacman.size,
            walls,
        )
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
    if map[row][col] == 2 || map[row][col] == 3 {
        map[row][col] = 0;
    }
    return map;
}
pub fn can_move_to_direction(col: usize, row: usize, direction: Vec2) -> bool {
    // RAW_MAP[y][x] != 1
    // this must be changed here to adjust pacman's collision checker a few pixels behind
    // where he is going, probably 16 pixels to make input buffering confirmation more "aligned"
    // let row = ((pacman.position.x - BOARD_TOP_LEFT_COORDS.x) / TILE_SIZE).floor() as usize;
    // let col = ((pacman.position.y - BOARD_TOP_LEFT_COORDS.y) / TILE_SIZE).floor() as usize;
    //
    if direction == vec2(1.0, 0.0) && RAW_MAP[row][col + 1] == 1 {
        false
    } else if direction == vec2(-1.0, 0.0) && RAW_MAP[row][col - 1] == 1 {
        false
    } else if direction == vec2(0.0, 1.0) && RAW_MAP[row + 1][col] == 1 {
        false
    } else if direction == vec2(0.0, -1.0) && RAW_MAP[row - 1][col] == 1 {
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
    draw_text(col_string, 50.0, 50.0, 15.0, YELLOW);
    draw_text(row_string, 50.0, 65.0, 15.0, YELLOW);
}

pub fn is_aligned_x(pacman_pos: f32) -> bool {
    if pacman_pos % 16.0 == 0.0 {
        true
    } else {
        false
    }
}

pub fn centered_coordinates(pacman_pos: Vec2) -> Vec2 {
    let row = ((pacman_pos.x - BOARD_TOP_LEFT_COORDS.x) / TILE_SIZE).floor() as f32;
    let col = ((pacman_pos.y - BOARD_TOP_LEFT_COORDS.y) / TILE_SIZE).floor() as f32;
    //
    // let col = (pacman_pos.x / tile_size).floor() as i32;
    // let row = (pacman_pos.y / tile_size).floor() as i32;
    // let centered_x = (pacman_pos.x / tile_size).round() * tile_size;
    // let centered_y = (pacman_pos.y / tile_size).round() * tile_size;
    let centered_x = ((row * TILE_SIZE) + BOARD_TOP_LEFT_COORDS.x) + 16.0;
    let centered_y = ((col * TILE_SIZE) + BOARD_TOP_LEFT_COORDS.y) + 16.0;
    vec2(centered_x, centered_y)
}
