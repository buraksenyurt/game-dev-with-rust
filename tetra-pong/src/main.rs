mod constant;
mod game_state;

use crate::game_state::GameState;
use tetra::ContextBuilder;

fn main() -> tetra::Result {
    // Oyuna ait Context nesnesi inşa edilir
    // Başlık, genişlik, yükseklik
    ContextBuilder::new("Başka Bir Ping Pong", 640, 480)
        .quit_on_escape(true) // ESC tuşuna basılırsa oyundan çıkılır
        .build()
        .expect("Context oluşturulması sırasında hata.")
        .run(|_| Ok(GameState {})) // Oyun döngüsü State nesnesi ile birlikte başlatılır
}
