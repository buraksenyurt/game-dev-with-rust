/*
Oyun durum bilgilerini tutan State nesnesi ve implementasyonu
*/
use crate::constant::{OCEAN_BLUE, PADDLE1_PATH, PADDLE2_PATH};
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};
use tetra::graphics::{Color, Texture};
use tetra::math::Vec2;
use tetra::{graphics, Context, State, TetraError};

pub struct GameState {
    pub paddle1_texture: Texture,
    pub paddle1_position: Vec2<f32>,
    pub paddle2_texture: Texture,
    pub paddle2_position: Vec2<f32>,
}

impl GameState {
    // Constructor fonksiyonumuz
    pub fn new(context: &mut Context) -> tetra::Result<GameState> {
        // Paddle1 imajını alıyoruz
        let paddle1_texture = Texture::new(context, PADDLE1_PATH)?;
        // Paddle1 için başlangıç pozisyonunu ayarlıyoruz. Ekranın üstünden ortada.
        let paddle1_position = Vec2::new((SCREEN_WIDTH - paddle1_texture.width() as f32) * 0.5, 0.);

        // Paddle2 içinde resim yüklemesi ve başlangıç konumlarını ayarlıyoruz.
        let paddle2_texture = Texture::new(context, PADDLE2_PATH)?;
        let paddle2_position = Vec2::new(
            (SCREEN_WIDTH - paddle2_texture.width() as f32) * 0.5,
            SCREEN_HEIGHT - paddle2_texture.height() as f32,
        );

        Ok(GameState {
            paddle1_texture,
            paddle1_position,
            paddle2_texture,
            paddle2_position,
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
        // İkinci oyuncunun raketi çizdirilir
        self.paddle2_texture.draw(context, self.paddle2_position);
        Ok(())
    }
}
