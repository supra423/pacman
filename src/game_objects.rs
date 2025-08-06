use crate::constants::*;
use crate::map_operations::*;
use macroquad::prelude::*;

pub struct FoodPellet {
    pub position: Vec2,
    pub size: f32,
    pub power_up: bool,
}

pub struct PacMan {
    pub position: Vec2,
    pub direction: Vec2,
    pub size: f32,
    pub speed: f32,
    pub powered_up: bool,
}

pub struct Ghost {
    pub position: Vec2,
    pub prev_pos_in_grid: (usize, usize),
    pub can_draw: bool,
    pub direction: Vec2,
    pub size: f32,
    pub speed: f32,
    pub can_change_direction: bool,
    pub frightened_mode: bool,
    pub scatter_mode: bool,
    pub chase_mode: bool,
    pub respawn_mode: bool,
}

pub enum Entity<'a> {
    PacMan(&'a PacMan),
    Ghost(&'a Ghost),
}
