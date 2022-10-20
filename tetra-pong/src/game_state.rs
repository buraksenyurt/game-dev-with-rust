/*
Oyun durum bilgilerini tutan State nesnesi ve implementasyonu
*/
use crate::constant::{OCEAN_BLUE, PADDLE1_PATH};
use crate::SCREEN_WIDTH;
use tetra::graphics::{Color, Texture};
use tetra::math::Vec2;
use tetra::{graphics, Context, State, TetraError};

pub struct GameState {
    pub paddle1_texture: Texture,
    pub paddle1_position: Vec2<f32>,
}

impl GameState {
    // Constructor fonksiyonumuz
    pub fn new(context: &mut Context) -> tetra::Result<GameState> {
        // Paddle1 imajını alıyoruz
        let paddle1_texture = Texture::new(context, PADDLE1_PATH)?;
        // başlangıç pozisyonunu ayarlıyoruz. Ekranın üstünden ortada.
        let paddle1_position = Vec2::new((SCREEN_WIDTH - paddle1_texture.width() as f32) * 0.5, 0.);
        Ok(GameState {
            paddle1_texture,
            paddle1_position,
        })
    }
}

impl State for GameState {
    // Çizim işlemlerinin ele alındığı fonksiyon
    fn draw(&mut self, context: &mut Context) -> Result<(), TetraError> {
        // Ekranı belirtilen renk ile temizliyor
        graphics::clear(context, Color::hex(OCEAN_BLUE));
        // İlk oyuncunun raketini ekrana çizdiriyoruz
        self.paddle1_texture.draw(context, self.paddle1_position);
        Ok(())
    }
}
