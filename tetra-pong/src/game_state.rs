/*
Oyun durum bilgilerini tutan State nesnesi ve implementasyonu
*/
use crate::constant::{BALL_PATH, OCEAN_BLUE, PADDLE1_PATH, PADDLE2_PATH};
use crate::entity::{Entity, Player};
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};
use tetra::graphics::{Color, Texture};
use tetra::input::Key;
use tetra::math::Vec2;
use tetra::{graphics, input, Context, State, TetraError};

pub struct GameState {
    pub player1: Player,
    pub player2: Player,
    pub ball: Entity, // pub paddle1_texture: Texture,
                      // pub paddle1_position: Vec2<f32>,
                      // pub paddle2_texture: Texture,
                      // pub paddle2_position: Vec2<f32>,
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

        let ball_texture = Texture::new(context, BALL_PATH)?;
        let ball_position = Vec2::new(
            (SCREEN_WIDTH - ball_texture.width() as f32) * 0.5,
            (SCREEN_HEIGHT - ball_texture.height() as f32) * 0.5,
        );

        let game_state = GameState {
            player1: Player {
                core: Entity::new(paddle1_texture, paddle1_position),
            },
            player2: Player {
                core: Entity::new(paddle2_texture, paddle2_position),
            },
            ball: Entity::new(ball_texture, ball_position),
        };
        Ok(game_state)

        // Ok(GameState {
        //     paddle1_texture,
        //     paddle1_position,
        //     paddle2_texture,
        //     paddle2_position,
        // })
    }
}

impl State for GameState {
    // Çizim işlemlerinin ele alındığı fonksiyon
    fn draw(&mut self, context: &mut Context) -> Result<(), TetraError> {
        // Ekranı belirtilen renk ile temizliyor
        graphics::clear(context, Color::hex(OCEAN_BLUE));
        // İlk oyuncunun raketini ekrana çizdiriyoruz
        self.player1.core.draw(context);
        // İkinci oyuncunun raketi çizdirilir
        self.player2.core.draw(context);

        self.ball.draw(context);
        Ok(())
    }

    // Klavye hareketlerine göre raket pozisyonlarının değiştirilmesi gibi
    // güncellemelerin yapıldığı fonksiyon
    fn update(&mut self, context: &mut Context) -> Result<(), TetraError> {
        if input::is_key_down(context, Key::Left) {
            self.player1.go_left();
        }
        if input::is_key_down(context, Key::Right) {
            self.player1.go_right();
        }
        if input::is_key_down(context, Key::A) {
            self.player2.go_left();
        }
        if input::is_key_down(context, Key::D) {
            self.player2.go_right();
        }

        Ok(())
    }
}
