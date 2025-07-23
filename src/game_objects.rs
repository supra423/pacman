use macroquad::math::Vec2;

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

#[derive(Debug)]
pub struct Ghost {
    pub position: Vec2,
    pub direction: Vec2,
    pub size: f32,
    pub speed: f32,
    pub flee_mode: bool,
}
