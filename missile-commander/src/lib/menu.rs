use crate::Level;
use macroquad::prelude::{draw_text, measure_text, screen_height, screen_width, RED};

pub fn draw_main_menu(_level: &Level) {
    let mut lines = Vec::new();
    lines.push("Wellcome Commander");
    lines.push("");
    lines.push("Are you ready! Press SPACE to start");
    lines.push("Press ESC to exit");

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

pub fn draw_dead_menu() {
    let mut lines = Vec::new();
    //let info = format!("Your Score {}", game.player_hit).as_str();
    lines.push("City down. Sorry Commander.");
    lines.push("");
    //lines.push(info);
    lines.push("Try again? Press SPACE");
    lines.push("Press ESC to exit");

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

pub fn draw_win_menu() {
    let mut lines = Vec::new();
    //let info = format!("Your Score {}", game.player_hit).as_str();
    lines.push("Yow win Commander.");
    lines.push("");
    //lines.push(info);
    lines.push("Are you ready to next stage? Press SPACE");
    lines.push("Press ESC to exit");

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
