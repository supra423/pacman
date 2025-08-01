use macroquad::math::Vec2;

use crate::constants::TILE_SIZE;

#[derive(Debug)]
pub struct Wall {
    pub position: Vec2,
    pub size: f32,
}

#[derive(Debug)]
pub struct FoodPellet {
    pub position: Vec2,
    pub size: f32,
    pub power_up: bool,
}

#[derive(Debug)]
pub struct PacMan {
    pub position: Vec2,
    pub direction: Vec2,
    pub size: f32,
    pub speed: f32,
    pub powered_up: bool,
}

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
}

#[derive(Debug)]
pub struct Ghost {
    pub position: Vec2,
    pub direction: Vec2,
    pub size: f32,
    pub speed: f32,
    pub frightened_mode: bool,
    pub scatter_mode: bool,
    pub chase_mode: bool,
}

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
}

pub enum Entity<'a> {
    PacMan(&'a PacMan),
    Ghost(&'a Ghost),
}
