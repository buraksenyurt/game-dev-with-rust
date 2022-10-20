/*
Oyun durum bilgilerini tutan State nesnesi ve implementasyonu
*/
use crate::constant::OCEAN_BLUE;
use tetra::graphics::Color;
use tetra::{graphics, Context, State, TetraError};

pub struct GameState {}

impl State for GameState {
    // Çizim işlemlerinin ele alındığı fonksiyon
    fn draw(&mut self, ctx: &mut Context) -> Result<(), TetraError> {
        // Ekranı belirtilen renk ile temizliyor
        graphics::clear(ctx, Color::hex(OCEAN_BLUE));
        Ok(())
    }
}
