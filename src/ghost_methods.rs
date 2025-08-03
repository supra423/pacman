use crate::constants::*;
use crate::game_objects::*;
use macroquad::prelude::*;

impl Ghost {
    pub fn new(position: Vec2, speed: f32, direction: Vec2) -> Self {
        Self {
            position,
            size: TILE_SIZE,
            direction,
            speed,
            frightened_mode: true,
            scatter_mode: false,
            chase_mode: false,
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
}
