use crate::constants::{FRAME_TIME, TILE_SIZE};
use macroquad::math::Vec2;

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
    pub direction: Vec2,
    pub size: f32,
    pub speed: f32,
    pub frightened_mode: bool,
    pub scatter_mode: bool,
    pub chase_mode: bool,
}

pub enum Entity<'a> {
    PacMan(&'a PacMan),
    Ghost(&'a Ghost),
}
