/*
   Oyuna menü, oynanma kısım ve yeniden başlatma gibi state'ler eklemek istiyorum.
   Tetra dokümanlarına göre yollardan birisi bunları birer sahne olarak düşünmek
   ve ana oyun nesnesindeki(GameState) draw ve update fonksiyonlarında sahneler arası
   geçişleri kontrol etmek. Diğer oyun motorlarındaki State Machine kullanımları biraz daha kolay.
*/
use crate::constant::{
    BALL_ACC, BALL_PATH, BALL_SPEED, BRICK, BRIGHT_REDDISH_LILAC, PADDLE1_PATH, PADDLE2_PATH,
    PADDLE_SPIN,
};
use crate::entity::{Ball, Entity, Player};
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};
use std::fmt::{Display, Formatter};
use tetra::graphics::text::{Font, Text};
use tetra::graphics::{Color, Texture};
use tetra::input::Key;
use tetra::math::Vec2;
use tetra::{graphics, input, Context};

pub enum Winner {
    Player1,
    Player2,
}

impl Display for Winner {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Player1 => write!(f, "Player I"),
            Self::Player2 => write!(f, "Player II"),
        }
    }
}

pub enum Transition {
    None,
    Push(Box<dyn Scene>),
    End(Winner),
}

pub trait Scene {
    fn update(&mut self, context: &mut Context) -> tetra::Result<Transition>;
    fn draw(&mut self, context: &mut Context) -> tetra::Result<Transition>;
}

pub struct MainMenuScene {
    content: Text,
}

impl MainMenuScene {
    pub fn new(context: &mut Context) -> tetra::Result<MainMenuScene> {
        Ok(MainMenuScene {
            content: Text::new(
                "Press Enter for Start Game\nESC for exit",
                Font::vector(context, "./assets/Halo3.ttf", 36.)?,
            ),
        })
    }
}

impl Scene for MainMenuScene {
    fn update(&mut self, context: &mut Context) -> tetra::Result<Transition> {
        if input::is_key_pressed(context, Key::Enter) {
            Ok(Transition::Push(Box::new(GameScene::new(context))))
        } else {
            Ok(Transition::None)
        }
    }

    fn draw(&mut self, context: &mut Context) -> tetra::Result<Transition> {
        graphics::clear(context, Color::hex(BRICK));
        self.content
            .draw(context, Vec2::new(10., SCREEN_HEIGHT * 0.5));
        Ok(Transition::None)
    }
}

pub struct EndScene {
    content: Text,
}

impl EndScene {
    pub fn new(context: &mut Context, winner: Winner) -> tetra::Result<EndScene> {
        Ok(EndScene {
            content: Text::new(
                format!("Winner is {}\nWould you play again?", winner),
                Font::vector(context, "./assets/Halo3.ttf", 36.)?,
            ),
        })
    }
}

impl Scene for EndScene {
    fn update(&mut self, context: &mut Context) -> tetra::Result<Transition> {
        if input::is_key_pressed(context, Key::Enter) {
            Ok(Transition::Push(Box::new(GameScene::new(context))))
        } else {
            Ok(Transition::None)
        }
    }

    fn draw(&mut self, context: &mut Context) -> tetra::Result<Transition> {
        graphics::clear(context, Color::hex(BRIGHT_REDDISH_LILAC));
        self.content
            .draw(context, Vec2::new(10., SCREEN_HEIGHT * 0.5));
        Ok(Transition::None)
    }
}

pub struct GameScene {
    pub player1: Player,
    pub player2: Player,
    pub ball: Ball,
    //pub score_board: Scoreboard,
}

impl GameScene {
    pub fn new(context: &mut Context) -> Self {
        let paddle1_texture = Texture::new(context, PADDLE1_PATH).unwrap();
        let paddle1_position = Vec2::new((SCREEN_WIDTH - paddle1_texture.width() as f32) * 0.5, 0.);
        let paddle2_texture = Texture::new(context, PADDLE2_PATH).unwrap();
        let paddle2_position = Vec2::new(
            (SCREEN_WIDTH - paddle2_texture.width() as f32) * 0.5,
            SCREEN_HEIGHT - paddle2_texture.height() as f32,
        );

        let ball_texture = Texture::new(context, BALL_PATH).unwrap();
        let ball_position = Vec2::new(
            (SCREEN_WIDTH - ball_texture.width() as f32) * 0.5,
            (SCREEN_HEIGHT - ball_texture.height() as f32) * 0.5,
        );
        let ball_velocity = Vec2::new(0., BALL_SPEED);

        let game = GameScene {
            player1: Player {
                core: Entity::new(paddle1_texture, paddle1_position),
            },
            player2: Player {
                core: Entity::new(paddle2_texture, paddle2_position),
            },
            ball: Ball::new(Entity::new(ball_texture, ball_position), ball_velocity),
            //score_board: Scoreboard::default(),
        };

        game
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

impl Scene for GameScene {
    fn draw(&mut self, context: &mut Context) -> tetra::Result<Transition> {
        // İlk oyuncunun raketini ekrana çizdiriyoruz
        self.player1.core.draw(context);
        // İkinci oyuncunun raketi çizdirilir
        self.player2.core.draw(context);
        // Top sahaya çizdirilir
        self.ball.core.draw(context);

        // let mut score_board = Text::new(
        //     self.score_board.to_string(),
        //     Font::vector(context, "./assets/Halo3.ttf", 28.)?,
        // );
        // score_board.draw(context, Vec2::new(0., SCREEN_HEIGHT * 0.5));
        Ok(Transition::None)
    }
    fn update(&mut self, context: &mut Context) -> tetra::Result<Transition> {
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

        if self.ball.core.position.x <= 0.
            || self.ball.core.position.x + self.ball.core.sprite.width() as f32 >= SCREEN_WIDTH
        {
            self.ball.velocity.x = -self.ball.velocity.x;
        }

        if self.ball.core.position.y < 0. {
            // self.score_board.p1_point += 10;
            // return Ok(Transition::Push(Box::new(EndScene::new(
            //     context, "Player 1",
            // ))));
            return Ok(Transition::End(Winner::Player1));
        }

        if self.ball.core.position.y > SCREEN_HEIGHT {
            //self.score_board.p2_point += 10;
            // return Ok(Transition::Push(Box::new(EndScene::new(
            //     context, "Player 2",
            // ))));
            return Ok(Transition::End(Winner::Player2));
        }

        Ok(Transition::None)
    }
}
