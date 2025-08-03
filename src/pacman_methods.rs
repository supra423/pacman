use crate::constants::*;
use crate::game_objects::*;
use macroquad::prelude::*;

impl PacMan {
    pub fn new(position: Vec2, speed: f32) -> Self {
        Self {
            position,
            size: TILE_SIZE,
            direction: Vec2::ZERO,
            speed,
            powered_up: false,
        }
    }
    pub fn move_character(&mut self, direction: Vec2) {
        self.position += direction * self.speed * FRAME_TIME;
    }
    pub fn go_to_other_side(&mut self) -> f32 {
        if self.position.x > 1036.0 {
            self.position.x = 204.0;
        } else if self.position.x < 204.0 {
            self.position.x = 1036.0;
        }
        self.position.x
    }
    pub fn draw(
        &self,
        image1: &Texture2D,
        image2: &Texture2D,
        image3: &Texture2D,
        timer: u32,
        colliding: bool,
    ) {
        let rotation: f32;

        if self.direction == vec2(0.0, 0.0) || self.direction == vec2(-1.0, 0.0) {
            rotation = PI;
            self.animate_sprite(image1, image2, image3, rotation, timer, colliding);
        } else if self.direction == vec2(1.0, 0.0) {
            rotation = 0.0;
            self.animate_sprite(image1, image2, image3, rotation, timer, colliding);
        } else if self.direction == vec2(0.0, 1.0) {
            // PI / 2.0 means a 90 degree rotation
            // because angles are measured
            // in radians, instead of degrees
            // PI is defined in the constants file btw
            rotation = PI / 2.0;
            self.animate_sprite(image1, image2, image3, rotation, timer, colliding);
        } else if self.direction == vec2(0.0, -1.0) {
            rotation = 3.0 * PI / 2.0; // 270 degrees
            self.animate_sprite(image1, image2, image3, rotation, timer, colliding);
        } else if self.direction == vec2(0.0, 0.0) {
            rotation = 0.0;
            self.animate_sprite(image1, image2, image3, rotation, timer, colliding);
        }
    }

    pub fn animate_sprite(
        &self,
        image1: &Texture2D,
        image2: &Texture2D,
        image3: &Texture2D,
        rotation: f32,
        timer: u32,
        colliding: bool,
    ) {
        let value = timer % 20;
        if value > 15 || self.direction == vec2(0.0, 0.0) || colliding {
            draw_texture_ex(
                image2,
                self.position.x - 27.5,
                self.position.y - 27.5,
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
                self.position.x - 27.5,
                self.position.y - 27.5,
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
                self.position.x - 27.5,
                self.position.y - 27.5,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(55.0, 55.0)),
                    rotation: rotation,
                    ..Default::default()
                },
            );
        }
    }
}

pub fn pacman_food_eat(mut map: [[u8; COLS]; ROWS], col: usize, row: usize) -> [[u8; COLS]; ROWS] {
    if map[col][row] == 2 || map[col][row] == 3 {
        map[col][row] = 0;
    }
    return map;
}
