use crate::constant::BASE_LENGTH;
use macroquad::prelude::*;

pub struct Building {
    position: Vec2,
    size: Vec2,
}

impl Building {
    pub fn new(position: Vec2, size: Vec2) -> Self {
        Self { position, size }
    }
    pub fn draw(&self) {
        draw_rectangle(
            self.position.x,
            self.position.y,
            self.size.x,
            self.size.y,
            LIGHTGRAY,
        );
    }
}

pub fn create_buildings() -> Vec<Building> {
    let heights = [30., 25., 60., 40., 45., 55., 70., 20.];
    let mut buildings: Vec<Building> = Vec::new();
    let mut position = Vec2::new(0., screen_height());
    let build_count = screen_width() / BASE_LENGTH;
    for _ in 0..build_count as i32 {
        let r = rand::gen_range(0, heights.len());
        position.y = screen_height() - heights[r];
        let building = Building::new(position, Vec2::new(BASE_LENGTH, heights[r]));
        buildings.push(building);
        position.x += BASE_LENGTH;
    }
    buildings
}

pub fn draw_buildings(buildings: &Vec<Building>) {
    for b in buildings {
        b.draw();
    }
}
