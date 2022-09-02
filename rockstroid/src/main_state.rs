use crate::game_assets::GameAssets;
use crate::sprite::Sprite;
use crate::sprite_builder::{create_random_rocks, create_sprite};
use crate::sprite_type::SpriteType;
use ggez::event::EventHandler;
use ggez::graphics::Color;
use ggez::{graphics, Context, GameError, GameResult};
use oorandom::Rand32;

// Oyunun herhangi bir andaki varlık durumunu tutacak veri yapısı.
// Belli bir anda oyun sahasındaki oyuncu, kayalar, atılan şutlar,
// skor bilgisi, seviye, assetler vs gibi bilgileri taşır.
pub struct MainState {
    player: Sprite,
    shots: Vec<Sprite>,
    rocks: Vec<Sprite>,
    assets: GameAssets,
    screen_width: f32,
    screen_height: f32,
    randomizer: Rand32,
    score: i32,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let mut seed: [u8; 8] = [0; 8];
        getrandom::getrandom(&mut seed[..]).expect("Randomizer oluşturulurken hata!");
        let mut randomizer = Rand32::new(u64::from_ne_bytes(seed));

        let assets = GameAssets::new(ctx)?;
        let player = create_sprite(SpriteType::Player);
        let rocks = create_random_rocks(&mut randomizer, 10, player.position, 100., 360.);

        let (w, h) = graphics::drawable_size(ctx);

        let ms = MainState {
            player,
            shots: Vec::new(),
            rocks,
            assets,
            screen_width: w,
            screen_height: h,
            randomizer,
            score: 0,
        };
        Ok(ms)
    }
}

// Ana oyun nesnesi için gerekli temel olaylara ait trait'ler kodlanır.
impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::Color::from(Color::BLACK));

        graphics::present(ctx)?;
        Ok(())
    }
}
