use crate::game::game::Game;
use macroquad::prelude::{draw_text, measure_text, screen_height, screen_width, WHITE};

pub fn draw_main_menu() {
    let lines = vec![
        "Wellcome Lieutenant".to_string(),
        "".to_string(),
        "Are you ready! Press SPACE to start".to_string(),
        "Press ESC to exit".to_string(),
    ];
    draw_menu(&lines);
}

pub fn draw_dead_menu(game: &Game) {
    let lines = vec![
        "Sorry Lieutenant. Your plane crashed".to_string(),
        "".to_string(),
        game.score_box.to_string(),
        "Try again? Press SPACE".to_string(),
        "Press ESC to exit".to_string(),
    ];

    draw_menu(&lines);
}

// pub fn draw_win_menu(game: &Game) {
//     let lines = vec![
//         "Congratulation Lieutenant. You did it!".to_string(),
//         "".to_string(),
//         game.score.to_string(),
//         "Are you ready to next stage? Press SPACE".to_string(),
//         "Press ESC to exit".to_string(),
//     ];
//
//     draw_menu(&lines);
// }

pub fn draw_end_menu(game: &Game) {
    let lines = vec![
        "Congratulaions Lieutenant.".to_string(),
        "".to_string(),
        "Sky is clear.".to_string(),
        game.score_box.to_string(),
        "Press ENTER for Credits".to_string(),
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
            WHITE,
        );
    }
}
