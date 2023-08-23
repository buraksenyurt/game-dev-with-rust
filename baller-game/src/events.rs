use bevy::prelude::*;

// Sistemler arası taşınabilecek bir event veri yapısıdır
// Örneğin oyuncu kırmız toplardan birisine yakalanınca
// hit sistem'den game over sisteme bu veri yapısı ile data taşınabilir.
// game over sistemde de son skor bilgisi elde edilebilir.
#[derive(Event)]
pub struct GameOver {
    pub final_score: u32,
}
