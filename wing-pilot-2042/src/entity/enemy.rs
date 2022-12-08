use crate::entity::enemy_type::EnemyType;
use crate::entity::formation::{get_formation, Formation};
use macroquad::color::WHITE;
use macroquad::prelude::{draw_texture, load_texture, Texture2D, Vec2};

pub struct Enemy {
    pub location: Vec2,
    pub velocity: Vec2,
    pub formation: Formation,
    pub is_alive: bool,
    pub enemy_type: EnemyType,
    pub texture: Texture2D,
    pub is_formation_on: bool,
    pub on_stage: bool,
}

impl Enemy {
    pub async fn new(location: Vec2, enemy_type: EnemyType, formation: Formation) -> Self {
        let texture = match enemy_type {
            EnemyType::Bomber => load_texture("resources/bomber.png").await.unwrap(),
            EnemyType::Fighter => load_texture("resources/enemy_fighter.png").await.unwrap(),
            EnemyType::Warship => load_texture("resources/warship.png").await.unwrap(),
        };
        //let formation = get_formation();
        Self {
            location,
            enemy_type,
            is_alive: true,
            velocity: Vec2::default(),
            formation,
            texture,
            is_formation_on: false,
            on_stage: true,
        }
    }
    pub fn draw(&self) {
        draw_texture(self.texture, self.location.x, self.location.y, WHITE);
    }
}
