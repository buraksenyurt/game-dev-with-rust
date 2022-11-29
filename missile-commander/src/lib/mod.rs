pub mod building;
pub mod bullet;
pub mod constant;
pub mod explosion;
pub mod game;
pub mod game_state;
pub mod menu;
pub mod missile;
pub mod score;
pub mod turret;

use crate::lib::missile::Missile;
use crate::{Building, BASE_LENGTH, CURSOR_LENGTH, WINDOW_HEIGHT, WINDOW_WITH};
use macroquad::prelude::*;

pub fn draw_cursor() {
    let (x, y) = mouse_position();
    draw_line(x - CURSOR_LENGTH, y, x + CURSOR_LENGTH, y, 1., RED);
    draw_line(x, y - CURSOR_LENGTH, x, y + CURSOR_LENGTH, 1., RED);

    draw_text(
        format!("{:?}", mouse_position()).as_str(),
        0.,
        screen_height() - 5.,
        20.,
        RED,
    );
}

pub fn create_missiles(quantity: i32) -> Vec<Missile> {
    let mut missiles = Vec::new();
    for _ in 0..quantity {
        let missile = Missile::spawn();
        //println!("{}", &missile);
        missiles.push(missile);
    }
    missiles
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

pub fn window_conf() -> Conf {
    Conf {
        window_title: "Missile Command".to_owned(),
        fullscreen: false,
        window_width: WINDOW_WITH,
        window_height: WINDOW_HEIGHT,
        window_resizable: false,
        ..Default::default()
    }
}

pub fn get_max(v1: f32, v2: f32) -> f32 {
    if v1 > v2 {
        return v1;
    }
    v2
}

pub fn get_min(v1: f32, v2: f32) -> f32 {
    if v1 < v2 {
        return v1;
    }
    v2
}
