use crate::{Game, Stage};
use macroquad::prelude::{draw_text, measure_text, screen_height, screen_width, RED};

pub fn draw_main_menu(stage: &Stage) {
    let lines = vec![
        "Wellcome Commander".to_string(),
        "".to_string(),
        format!("Stage {}", stage.level),
        "Are you ready! Press SPACE to start".to_string(),
        "Press ESC to exit".to_string(),
    ];
    draw_menu(&lines);
}

pub fn draw_dead_menu(game: &Game) {
    let lines = vec![
        "City down. Sorry Commander.".to_string(),
        "".to_string(),
        game.to_string(),
        "Try again? Press SPACE".to_string(),
        "Press ESC to exit".to_string(),
    ];

    draw_menu(&lines);
}

pub fn draw_win_menu(game: &Game) {
    let lines = vec![
        "Yow win Commander.".to_string(),
        "".to_string(),
        game.to_string(),
        "Are you ready to next stage? Press SPACE".to_string(),
        "Press ESC to exit".to_string(),
    ];

    draw_menu(&lines);
}

fn draw_menu(lines: &[String]) {
    for (i, line) in lines.iter().enumerate() {
        let size = measure_text(line, None, 32, 1.);
        draw_text(
            line,
            screen_width() * 0.5 - size.width * 0.5,
            screen_height() * 0.5 - size.height + (36. * i as f32),
            32.,
            RED,
        );
    }
}
