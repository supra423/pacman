use crate::constants::*;
use crate::game_objects::*;
use macroquad::prelude::*;

pub fn load_walls(map: [[u8; COLS]; ROWS]) -> Vec<Wall> {
    let rect_top_left_coords_x = RECT_TOP_LEFT_COORDS.x;
    let rect_top_left_coords_y = RECT_TOP_LEFT_COORDS.y;

    let mut map_walls = Vec::new();
    for i in 0..ROWS {
        for j in 0..COLS {
            if map[i][j] == 1 {
                let wall_object = Wall {
                    position: vec2(
                        rect_top_left_coords_y + (TILE_SIZE * i as f32) + 16.0,
                        rect_top_left_coords_x + (TILE_SIZE * j as f32) + 16.0,
                    ),
                    size: TILE_SIZE,
                };
                map_walls.push(wall_object);
            }
        }
    }
    return map_walls;
}

pub fn load_food(map: [[u8; COLS]; ROWS]) -> Vec<FoodPellet> {
    let rect_top_left_coords_x = RECT_TOP_LEFT_COORDS.x;
    let rect_top_left_coords_y = RECT_TOP_LEFT_COORDS.y;

    let mut map_food = Vec::new();
    for i in 0..ROWS {
        for j in 0..COLS {
            if map[i][j] == 2 {
                let food_pellet = FoodPellet {
                    position: vec2(
                        rect_top_left_coords_y + (TILE_SIZE * i as f32) + 16.0,
                        rect_top_left_coords_x + (TILE_SIZE * j as f32) + 16.0,
                    ),

                    size: 4.0,
                    power_up: false,
                };
                map_food.push(food_pellet);
            } else if map[i][j] == 3 {
                let power_pellet = FoodPellet {
                    position: vec2(
                        rect_top_left_coords_y + (TILE_SIZE * i as f32) + 16.0,
                        rect_top_left_coords_x + (TILE_SIZE * j as f32) + 16.0,
                    ),

                    size: 10.0,
                    power_up: true,
                };
                map_food.push(power_pellet);
            }
        }
    }
    return map_food;
}

pub fn draw_elements(map: [[u8; COLS]; ROWS], map_image: &Texture2D) {
    draw_texture_ex(
        &map_image,
        RECT_TOP_LEFT_COORDS.x,
        RECT_TOP_LEFT_COORDS.y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(BOARD_DIMENSIONS.x, BOARD_DIMENSIONS.y)),
            ..Default::default()
        },
    );
    let walls = load_walls(map);
    for wall in &walls {
        // draw_poly_lines(
        //     wall.position.y,
        //     wall.position.x,
        //     4,
        //     wall.size / 2.0_f32.sqrt(),
        //     45.0,
        //     1.0,
        //     GREEN,
        // );
    }
    let foods = load_food(map);
    for food in &foods {
        draw_poly(food.position.y, food.position.x, 4, food.size, 45.0, WHITE);
    }
}

// pub fn aabb_collision_center(pos_a: Vec2, size_a: f32, pos_b: Vec2, size_b: f32) -> bool {
//     let (pos_a_x, pos_a_y) = (pos_a.x, pos_a.y);
//     // let (aw, ah) = (size_a, size_a);
//     let (pos_b_x, pos_b_y) = (pos_b.x, pos_b.y);
//     // let (bw, bh) = (size_b, size_b);
//
//     // let half_aw = sizea / 2.0;
//     // let half_ah = ah / 2.0;
//     // let half_bw = bw / 2.0;
//     // let half_bh = bh / 2.0;
//     let half_size_a = size_a / 2.0_f32.sqrt();
//     let half_size_b = size_b / 2.0_f32.sqrt();
// }

// pub fn is_colliding(pos_a: Vec2, mut size_a: f32, pos_b: Vec2, mut size_b: f32) -> bool {
//     // size_a /= 2.0_f32.sqrt();
//     // size_a *= 3.5;
//     // size_b /= 2.0_f32.sqrt();
//
//     pos_a.x <= pos_b.x + size_b
//         && pos_a.x + size_a >= pos_b.x
//         && pos_a.y <= pos_b.y + size_b
//         && pos_a.y + size_a >= pos_b.y
// }
pub fn is_colliding(center_a: Vec2, size_a: f32, center_b: Vec2, size_b: f32) -> bool {
    let half_a = size_a / 2.0;
    let half_b = size_b / 2.0_f32.sqrt();

    let min_a = center_a - half_a;
    let max_a = center_a + half_a;

    let min_b = center_b - half_b;
    let max_b = center_b + half_b;

    min_a.x < max_b.x && max_a.x > min_b.x && min_a.y < max_b.y && max_a.y > min_b.y
}

// pub fn aabb_collision(object_a: Vec2, object_b: Vec2) -> bool {
//     a_x_min;
//     a_x_max;
//     a_y_min;
//     a_y_max;
//
//     b_x_min;
//     b_x_max;
//     b_y_min;
//     b_y_max;
// }

pub fn handle_controls() -> char {
    if is_key_pressed(KeyCode::Right) || is_key_pressed(KeyCode::D) {
        return 'r';
    } else if is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::A) {
        return 'l';
    } else if is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S) {
        return 'd';
    } else if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W) {
        return 'u';
    }
    return 'n';
}

pub fn pacman_movements() -> Option<Vec2> {
    let input = handle_controls();
    if input == 'r' {
        return Some(vec2(1.0, 0.0));
    } else if input == 'l' {
        return Some(vec2(-1.0, 0.0));
    } else if input == 'd' {
        return Some(vec2(0.0, 1.0));
    } else if input == 'u' {
        return Some(vec2(0.0, -1.0));
    } else if is_key_pressed(KeyCode::Space) {
        return Some(vec2(0.0, 0.0));
    } else {
        return None;
    }
}
pub fn can_move_in_direction(pos: Vec2, direction: Vec2, map: &[[u8; COLS]; ROWS]) -> bool {
    let next_pos = pos + direction * TILE_SIZE;

    let col = ((next_pos.x / TILE_SIZE).floor()) as usize;
    let row = ((next_pos.y / TILE_SIZE).floor()) as usize;
    println!("{col}");
    println!("{row}");

    if row < map.len() && col < map[0].len() && map[row][col] != 1 {
        return true;
    } else {
        return false;
    }
}
