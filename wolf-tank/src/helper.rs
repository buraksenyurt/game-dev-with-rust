use macroquad::prelude::{screen_height, screen_width, Vec2};

pub fn border_check(p: &Vec2, w: f32) -> Vec2 {
    let mut vr = Vec2::new(p.x, p.y);
    if vr.x > screen_width() {
        vr.x = 0. - w;
    }
    if vr.x < 0. - w {
        vr.x = screen_width()
    }
    if vr.y > screen_height() {
        vr.y = 0. - w;
    }
    if vr.y < 0. - w {
        vr.y = screen_height()
    }
    vr
}
