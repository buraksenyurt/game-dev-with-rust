use crate::entity::{Drawable, Entity};
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::any::Any;

#[derive(Debug, Clone)]
pub struct QuestionTower {
    pub idx: u32,
    pub width: u32,
    pub height: u32,
}

impl QuestionTower {
    pub fn new(idx: u32, width: u32, height: u32) -> Self {
        Self { idx, width, height }
    }
}

impl Entity for QuestionTower {
    fn update(&mut self, _delta_time: f32) {
        todo!()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Drawable for QuestionTower {
    fn draw(&self, _canvas: &mut Canvas<Window>) {
        todo!()
    }
}
