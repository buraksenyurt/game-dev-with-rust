use crate::constant::*;
use macroquad::prelude::*;

pub struct Player {
    pub circle: Circle,
}

impl Player {
    pub fn new() -> Self {
        Self {
            circle: Circle::new(
                screen_width() * 0.5 - PLAYER_CIRCLE_RADIUS * 0.5,
                screen_height() * 0.5 - PLAYER_CIRCLE_RADIUS * 0.5,
                PLAYER_CIRCLE_RADIUS,
            ),
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        let mut movement_x = 0.;
        let mut movement_y = 0.;
        if is_key_down(KeyCode::A) {
            movement_x -= 1.;
        }
        if is_key_down(KeyCode::W) {
            movement_y -= 1.;
        }
        if is_key_down(KeyCode::D) {
            movement_x += 1.;
        }
        if is_key_down(KeyCode::S) {
            movement_y += 1.;
        }
        self.circle.x += movement_x * delta_time * PLAYER_SPEED;
        self.circle.y += movement_y * delta_time * PLAYER_SPEED;

        if self.circle.x < self.circle.r {
            self.circle.x = self.circle.r;
        }
        if self.circle.x > screen_width() - self.circle.r {
            self.circle.x = screen_width() - self.circle.r;
        }

        if self.circle.y < self.circle.r {
            self.circle.y = self.circle.r;
        }
        if self.circle.y > screen_height() - self.circle.r {
            self.circle.y = screen_height() - self.circle.r
        }
    }

    pub fn draw(&self) {
        //draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, SKYBLUE);
        draw_circle(self.circle.x, self.circle.y, self.circle.r, SKYBLUE);
    }
}
