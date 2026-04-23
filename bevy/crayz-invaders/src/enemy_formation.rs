use crate::point::Point;
use crate::{WinSize, BASE_SPEED, MAX_FORMATION_MEMBERS};
use bevy::prelude::{Component, Resource};
use rand::RngExt;

#[derive(Clone, Component)]
pub struct Formation {
    pub start: Point,
    pub radius: Point,
    pub pivot: Point,
    pub speed: f32,
    pub angle: f32,
}

#[derive(Default, Resource)]
pub struct FormationMaker {
    template: Option<Formation>,
    members: u8,
}

impl FormationMaker {
    pub fn build(&mut self, win_size: &WinSize) -> Formation {
        match (&self.template, self.members >= MAX_FORMATION_MEMBERS) {
            (Some(tmpl), false) => {
                self.members += 1;
                tmpl.clone()
            }

            (None, _) | (_, true) => {
                let mut rng = rand::rng();

                let w_span = win_size.width / 2. + 100.;
                let h_span = win_size.height / 2. + 100.;
                let x = if rng.random_bool(0.5) { w_span } else { -w_span };
                let y = rng.random_range(-h_span..h_span) as f32;
                let start = Point::new(x, y);

                let w_span = win_size.width / 4.;
                let h_span = win_size.height / 3. - 50.;
                let pivot = Point::new(
                    rng.random_range(-w_span..w_span),
                    rng.random_range(0.0..h_span),
                );
                let radius = Point::new(rng.random_range(80.0..150.), 100.);
                let angle = (y - pivot.y).atan2(x - pivot.x);
                let speed = BASE_SPEED;

                let formation = Formation {
                    start,
                    radius,
                    pivot,
                    speed,
                    angle,
                };

                self.template = Some(formation.clone());
                self.members = 1;

                formation
            }
        }
    }
}
