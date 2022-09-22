use macroquad::prelude::*;

pub fn draw_title_text(title: &str) {
    let dimensions = measure_text(title, None, 48, 1.);
    draw_text_ex(
        title,
        screen_width() * 0.5 - dimensions.width * 0.5,
        screen_height() * 0.5 - dimensions.height * 0.5,
        TextParams {
            font_size: 48,
            color: DARKBROWN,
            ..Default::default()
        },
    );
}

pub fn draw_score_box(game_score: &mut i32, player_lives: &mut i32) {
    // Skor kutucuğunun çizildiği kısım
    let score_box = format!("Skor:{}. Kalan Can {} ", game_score, player_lives);
    let score_box_dimension = measure_text(&score_box, None, 24, 1.);
    draw_text_ex(
        &score_box,
        screen_width() * 0.5 - score_box_dimension.width * 0.5,
        20.,
        TextParams {
            color: BLACK,
            font_size: 24,
            ..Default::default()
        },
    );
}
