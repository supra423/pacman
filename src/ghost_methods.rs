use std::convert;

use crate::constants::*;
use crate::game_objects::{Entity, Ghost};
use crate::map_operations::*;
use macroquad::prelude::*;

impl Ghost {
    pub fn new(position: Vec2, speed: f32) -> Self {
        Self {
            position,
            curr_pos_in_grid: convert_pos_to_index(&position),
            prev_pos_in_grid: convert_pos_to_index(&position),
            can_draw: false,
            size: TILE_SIZE,
            direction: vec2(0.0, 0.0),
            speed,
            can_change_direction: true,
            frightened_mode: true,
            scatter_mode: false,
            chase_mode: false,
            respawn_mode: false,
        }
    }
    pub fn move_character(&mut self, direction: Vec2) {
        self.position += direction * self.speed * FRAME_TIME;
        self.curr_pos_in_grid = convert_pos_to_index(&self.position);
    }
    pub fn go_to_other_side(&mut self) -> f32 {
        if self.position.x > 1030.0 {
            self.position.x = 210.0;
        } else if self.position.x < 210.0 {
            self.position.x = 1030.0;
        }
        self.position.x
    }
    pub fn change_direction(&mut self, map: [[u8; COLS]; ROWS]) {
        if Entity::Ghost(&self).collision_checking_offset(map) {
            self.can_change_direction = true;
        }
        if self.can_change_direction {
            self.prev_pos_in_grid = convert_pos_to_index(&self.position);
            (self.position, self.direction) = update_frightened_position(&self, map);
            self.can_change_direction = false;
        }
        if self.prev_pos_in_grid != convert_pos_to_index(&self.position) {
            self.can_change_direction = true;
        }
    }
    pub fn reset_values(&mut self) {
        self.direction = vec2(0.0, 0.0);
        self.position = vec2(CENTER.x, CENTER.y - 128.0);
        self.can_draw = false;
    }
    pub fn draw_delay(&mut self, timer: u32, frame: u32, map: [[u8; COLS]; ROWS]) {
        if timer >= frame {
            self.change_direction(map);
            self.can_draw = true;
        }
        if timer == frame - 1 || timer == frame {
            self.position = vec2(CENTER.x, CENTER.y - 128.0);
            self.direction = vec2(0.0, -1.0);
        }
    }
}
