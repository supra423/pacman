use crate::constants::*;
use crate::game_objects::*;
use macroquad::prelude::*;

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
pub fn pacman_food_eat(mut map: [[u8; COLS]; ROWS], col: usize, row: usize) -> [[u8; COLS]; ROWS] {
    if map[col][row] == 2 || map[col][row] == 3 {
        map[col][row] = 0;
    }
    return map;
}
