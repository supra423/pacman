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

pub fn draw_pacman(
    pacman: &PacMan,
    image1: &Texture2D,
    image2: &Texture2D,
    image3: &Texture2D,
    timer: u32,
    colliding: bool,
) {
    // draw_circle(pacman.position.x, pacman.position.y, pacman.size, YELLOW);
    let rotation: f32;

    if pacman.direction == vec2(0.0, 0.0) || pacman.direction == vec2(-1.0, 0.0) {
        rotation = PI;
        animate_pacman_sprite(pacman, image1, image2, image3, rotation, timer, colliding);
    } else if pacman.direction == vec2(1.0, 0.0) {
        rotation = 0.0;
        animate_pacman_sprite(pacman, image1, image2, image3, rotation, timer, colliding);
    } else if pacman.direction == vec2(0.0, 1.0) {
        // PI / 2.0 means a 90 degree rotation
        // because angles are measured
        // in radians, instead of degrees
        // PI is defined in the constants file btw
        rotation = PI / 2.0;
        animate_pacman_sprite(pacman, image1, image2, image3, rotation, timer, colliding);
    } else if pacman.direction == vec2(0.0, -1.0) {
        rotation = 3.0 * PI / 2.0; // 270 degrees
        animate_pacman_sprite(pacman, image1, image2, image3, rotation, timer, colliding);
    } else if pacman.direction == vec2(0.0, 0.0) {
        rotation = 0.0;
        animate_pacman_sprite(pacman, image1, image2, image3, rotation, timer, colliding);
    }
}

pub fn animate_pacman_sprite(
    pacman: &PacMan,
    image1: &Texture2D,
    image2: &Texture2D,
    image3: &Texture2D,
    rotation: f32,
    timer: u32,
    colliding: bool,
) {
    let value = timer % 20;
    if value > 15 || pacman.direction == vec2(0.0, 0.0) || colliding {
        draw_texture_ex(
            image2,
            pacman.position.x - 27.5,
            pacman.position.y - 27.5,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(55.0, 55.0)),
                rotation: rotation,
                ..Default::default()
            },
        );
    } else if value > 10 || value <= 5 {
        draw_texture_ex(
            image3,
            pacman.position.x - 27.5,
            pacman.position.y - 27.5,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(55.0, 55.0)),
                rotation: rotation,
                ..Default::default()
            },
        );
    } else {
        draw_texture_ex(
            image1,
            pacman.position.x - 27.5,
            pacman.position.y - 27.5,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(55.0, 55.0)),
                rotation: rotation,
                ..Default::default()
            },
        );
    }
}

pub fn collision_check(position: Vec2) -> bool {
    let row = (((position.x) - BOARD_TOP_LEFT_COORDS.x) / TILE_SIZE).floor() as usize;
    let col = (((position.y) - BOARD_TOP_LEFT_COORDS.y) / TILE_SIZE).floor() as usize;

    if RAW_MAP[col][row] == 1 { true } else { false }
}

pub fn collision_checking_offset(a: &Entity) -> bool {
    let (position, direction) = match a {
        Entity::PacMan(pacman) => (pacman.position, pacman.direction),
        Entity::Ghost(ghost) => (ghost.position, ghost.direction),
    };
    if direction == vec2(1.0, 0.0) && collision_check(vec2(position.x + 16.0, position.y)) {
        true
    } else if direction == vec2(-1.0, 0.0) && collision_check(vec2(position.x - 16.0, position.y)) {
        true
    } else if direction == vec2(0.0, 1.0) && collision_check(vec2(position.x, position.y + 16.0)) {
        true
    } else if direction == vec2(0.0, -1.0) && collision_check(vec2(position.x, position.y - 16.0)) {
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

pub fn can_move_to_direction(position: Vec2, direction: Vec2) -> bool {
    let (row, col) = convert_pos_to_index(position);
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

pub fn centered_coordinates(position: Vec2) -> Vec2 {
    let (row, col) = convert_pos_to_index(position);
    let centered_x = (((row as f32) * TILE_SIZE) + BOARD_TOP_LEFT_COORDS.x) + 16.0;
    let centered_y = (((col as f32) * TILE_SIZE) + BOARD_TOP_LEFT_COORDS.y) + 16.0;
    vec2(centered_x, centered_y)
}

pub fn convert_pos_to_index(position: Vec2) -> (usize, usize) {
    let row = ((position.x - BOARD_TOP_LEFT_COORDS.x) / TILE_SIZE).floor() as usize;
    let col = ((position.y - BOARD_TOP_LEFT_COORDS.y) / TILE_SIZE).floor() as usize;
    (row, col)
}
