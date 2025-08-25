// use crate::constants::*;
// use crate::map_operations::*;
use macroquad::prelude::*;

pub struct FoodPellet {
    pub position: Vec2,
    pub size: f32,
}

pub struct PacMan {
    pub position: Vec2,
    pub pos_in_grid: (usize, usize),
    pub direction: Vec2,
    pub next_direction: Vec2,
    // pub size: f32,
    pub speed: f32,
    pub powered_up: bool,
    pub colliding: bool,
    pub power_up_timer: u16,
    pub score: u32,
    pub lives: u8,
}

pub struct Ghost {
    pub position: Vec2,
    pub curr_pos_in_grid: (usize, usize),
    pub prev_pos_in_grid: (usize, usize),
    pub can_draw: bool,
    pub direction: Vec2,
    pub size: f32,
    pub speed: f32,
    pub can_change_direction: bool,
    // pub frightened_mode: bool,
    // pub scatter_mode: bool,
    // pub chase_mode: bool,
    // pub respawn_mode: bool,
}

pub enum Entity<'a> {
    PacMan(&'a PacMan),
    Ghost(&'a Ghost),
}
