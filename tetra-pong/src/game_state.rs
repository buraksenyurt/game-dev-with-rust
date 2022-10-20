/*
Oyun durum bilgilerini tutan State nesnesi ve implementasyonu
*/
use crate::constant::{
    BALL_ACC, BALL_PATH, BALL_SPEED, OCEAN_BLUE, PADDLE1_PATH, PADDLE2_PATH, PADDLE_SPIN,
};
use crate::entity::{Ball, Entity, Player};
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};
use tetra::graphics::{Color, Texture};
use tetra::input::Key;
use tetra::math::Vec2;
use tetra::{graphics, input, Context, State, TetraError};

pub struct GameState {
    pub player1: Player,
    pub player2: Player,
    pub ball: Ball,
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
        // Topun hızı ve yönünü başlangıçta bir vektör olarak tanımladık
        let ball_velocity = Vec2::new(0., BALL_SPEED);

        let game_state = GameState {
            player1: Player {
                core: Entity::new(paddle1_texture, paddle1_position),
            },
            player2: Player {
                core: Entity::new(paddle2_texture, paddle2_position),
            },
            ball: Ball::new(Entity::new(ball_texture, ball_position), ball_velocity),
        };
        Ok(game_state)
    }

    // Oyuncuların raketleri ile topa vurup vurmadıklarını kontrol eden fonksiyon
    // Axis Aligned Bounding Boxes(AABB) tekniği kullanılır
    fn check_collision(&mut self) {
        // raketlerin ve topun dörtgensel sınırlarını bul
        let paddle1_bounds = self.player1.core.get_bounds();
        let paddle2_bounds = self.player2.core.get_bounds();
        let ball_bounds = self.ball.core.get_bounds();

        // Bir kesişme olup olmadığını öğren
        let paddle_hit = if ball_bounds.intersects(&paddle1_bounds) {
            Some(&self.player1)
        } else if ball_bounds.intersects(&paddle2_bounds) {
            Some(&self.player2)
        } else {
            None
        };
        // Kesişme varsa raketle top çarpışmış demektir.
        // Özetle oyuncu topa vurabilmiştir.
        if let Some(paddle) = paddle_hit {
            // Eğer öyleyse topun yönünü değiştiriyoruz ve bunu yaparken hızlanmayı da ayarlıyoruz

            self.ball.velocity.y =
                -(self.ball.velocity.y + (BALL_ACC * self.ball.velocity.y.signum()));
            let offset = (paddle.core.get_centre().x - self.ball.core.get_centre().x)
                / paddle.core.sprite.height() as f32;
            self.ball.velocity.x += PADDLE_SPIN * -offset;
            // self.ball.velocity.x += -offset;
        }
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

        self.ball.core.draw(context);

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

        self.ball.core.position += self.ball.velocity;

        self.check_collision();

        if self.ball.core.position.x <= 0.0
            || self.ball.core.position.x + self.ball.core.sprite.width() as f32 >= SCREEN_WIDTH
        {
            self.ball.velocity.x = -self.ball.velocity.x;
        }

        Ok(())
    }
}
