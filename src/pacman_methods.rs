use crate::constants::*;
use crate::game_objects::PacMan;
use crate::game_objects::*;
use crate::map_operations::*;
use macroquad::prelude::*;

impl PacMan {
    pub fn new(position: Vec2, speed: f32) -> Self {
        Self {
            position,
            pos_in_grid: convert_pos_to_index(&position),
            // size: TILE_SIZE,
            direction: Vec2::ZERO,
            next_direction: Vec2::ZERO,
            speed,
            powered_up: false,
            power_up_timer: 0,
            colliding: false,
            score: 0,
            lives: 3,
        }
    }
    pub fn move_character(&mut self, direction: Vec2) {
        self.position += direction * self.speed * FRAME_TIME;
        self.pos_in_grid = convert_pos_to_index(&self.position);
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

        if self.direction == vec2(-1.0, 0.0) {
            rotation = PI;
            self.animate_sprite(image1, image2, image3, rotation, timer, colliding);
        } else if self.direction == vec2(1.0, 0.0) {
            rotation = 0.0;
            self.animate_sprite(image1, image2, image3, rotation, timer, colliding);
        } else if self.direction == vec2(0.0, 1.0) {
            // PI / 2.0 means a 90-degree rotation
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

    fn animate_sprite(
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
                    rotation,
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
                    rotation,
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
                    rotation,
                    ..Default::default()
                },
            );
        }
    }
    pub fn food_eat(&mut self, map: &mut [[u8; COLS]; ROWS]) {
        let (row, col) = convert_pos_to_index(&self.position);
        match map[col][row] {
            2 => {
                self.score += 10;
                map[col][row] = 0;
            }
            3 => {
                self.score += 50;
                self.powered_up = true;
                self.power_up_timer = 0;
                map[col][row] = 0;
            }
            _ => {}
        }
    }
    pub fn aabb(&self, ghost: &Ghost) -> bool {
        let a_min = self.position - Vec2::splat(TILE_SIZE / 2.0);
        let a_max = self.position + Vec2::splat(TILE_SIZE / 2.0);

        let b_min = ghost.position - Vec2::splat(TILE_SIZE / 2.0);
        let b_max = ghost.position + Vec2::splat(TILE_SIZE / 2.0);

        a_min.x < b_max.x && a_max.x > b_min.x && a_min.y < b_max.y && a_max.y > b_min.y
    }

    pub fn reset_values(&mut self) {
        self.powered_up = false;
        self.direction = vec2(0.0, 0.0);
        self.position = vec2(CENTER.x, CENTER.y + 256.0);
    }
    pub fn change_directions(&mut self, map: [[u8; COLS]; ROWS]) {
        if self.direction != self.next_direction {
            if can_move_to_direction(self.position, self.next_direction, map) {
                self.position = centered_coordinates(self.position);
                self.direction = self.next_direction;
            }
        }
    }

    pub fn colliding(&mut self, map: [[u8; COLS]; ROWS]) {
        self.colliding = false;
        if Entity::PacMan(&self).collision_checking_offset(map) {
            self.colliding = true;
            self.position = centered_coordinates(self.position);
        }
    }

    pub fn draw_score(&self) {
        let score_text = &self.score.to_string();
        let formatted_score = format!("SCORE: {score_text}");
        draw_text(&formatted_score, BOARD_TOP_LEFT_COORDS.x, 35.0, 30.0, WHITE);
    }

    pub fn draw_lives(&self) {
        let lives_text = &self.lives.to_string();
        let formatted_score = format!("LIVES: {lives_text}");
        draw_text(&formatted_score, CENTER.x - 50.0, 35.0, 30.0, WHITE);
    }

    pub fn debug_texts(&self) {
        let pacman_pos_string = &self.position.to_string();
        let (row, col) = convert_pos_to_index(&self.position);
        let col_string = &col.to_string();
        let row_string = &row.to_string();

        if self.colliding {
            let collision_text = "COLLIDING";
            draw_text(&collision_text, 50.0, 25.0, 15.0, YELLOW);
        }

        draw_text(pacman_pos_string, 50.0, 35.0, 15.0, YELLOW);
        draw_text(row_string, 50.0, 50.0, 15.0, YELLOW);
        draw_text(col_string, 50.0, 65.0, 15.0, YELLOW);
    }
}
