use crate::constants::*;
use crate::game_objects::*;
use macroquad::prelude::*;

impl Entity<'_> {
    fn get_pos(&self) -> Vec2 {
        let position = match self {
            Entity::PacMan(pacman) => pacman.position,
            Entity::Ghost(ghost) => ghost.position,
        };
        position
    }
    fn get_dir(&self) -> Vec2 {
        let direction = match self {
            Entity::PacMan(pacman) => pacman.direction,
            Entity::Ghost(ghost) => ghost.direction,
        };
        direction
    }
    fn collision_check(&self, position: Vec2, map: [[u8; COLS]; ROWS]) -> bool {
        let row = (((position.x) - BOARD_TOP_LEFT_COORDS.x) / TILE_SIZE).floor() as usize;
        let col = (((position.y) - BOARD_TOP_LEFT_COORDS.y) / TILE_SIZE).floor() as usize;

        if map[col][row] == 1 { true } else { false }
    }
    pub fn collision_checking_offset(&self, map: [[u8; COLS]; ROWS]) -> bool {
        // let (position, direction) = match self {
        //     Entity::PacMan(pacman) => (pacman.position, pacman.direction),
        //     Entity::Ghost(ghost) => (ghost.position, ghost.direction),
        // };
        let position = self.get_pos();
        let direction = self.get_dir();
        if direction == vec2(1.0, 0.0)
            && self.collision_check(vec2(position.x + 16.0, position.y), map)
        {
            true
        } else if direction == vec2(-1.0, 0.0)
            && self.collision_check(vec2(position.x - 16.0, position.y), map)
        {
            true
        } else if direction == vec2(0.0, 1.0)
            && self.collision_check(vec2(position.x, position.y + 16.0), map)
        {
            true
        } else if direction == vec2(0.0, -1.0)
            && self.collision_check(vec2(position.x, position.y - 16.0), map)
        {
            true
        } else {
            false
        }
    }
    pub fn go_to_other_side(&mut self) -> f32 {
        let mut position = self.get_pos();
        if position.x > 1030.0 {
           position.x = 210.0;
        } else if position.x < 210.0 {
            position.x = 1030.0;
        }
        position.x
    }

}
