mod constant;
mod entity;
mod game_state;
mod scenes;

use crate::constant::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::game_state::GameState;
use tetra::ContextBuilder;

fn main() -> tetra::Result {
    // Oyuna ait Context nesnesi inşa edilir
    // Başlık, genişlik, yükseklik
    ContextBuilder::new(
        "Başka Bir Ping Pong",
        SCREEN_WIDTH as i32,
        SCREEN_HEIGHT as i32,
    )
    .quit_on_escape(true) // ESC tuşuna basılırsa oyundan çıkılır
    .build()
    .expect("Context oluşturulması sırasında hata.")
    .run(GameState::new) // Oyun döngüsü State nesnesi ile birlikte başlatılır
}
