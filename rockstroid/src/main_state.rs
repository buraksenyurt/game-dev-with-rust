use crate::constant::{MAX_RADIUS, MAX_ROCK_COUNT, MIN_RADIUS};
use crate::game_assets::GameAssets;
use crate::sprite::Sprite;
use crate::sprite_builder::{create_random_rocks, create_sprite};
use crate::sprite_type::SpriteType;
use ggez::event::EventHandler;
use ggez::graphics::{draw, Color, Drawable};
use ggez::mint::Point2;
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
        let rocks = create_random_rocks(
            &mut randomizer,
            MAX_ROCK_COUNT,
            player.position,
            MIN_RADIUS,
            MAX_RADIUS,
        );

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

        let game_assets = &self.assets;
        let coordinates = (self.screen_width, self.screen_height);

        // Oyuncu çizilir
        let hero = &self.player;
        draw_sprite(ctx, &game_assets, &hero, coordinates)?;

        // rastgele konumlanan kayalar çizdirilir
        for r in &self.rocks {
            draw_sprite(ctx, game_assets, &r, coordinates)?;
        }

        graphics::present(ctx)?;

        Ok(())
    }
}

// Hareket edebilen bir nesneyi çizmek için kullanılan fonksiyon
fn draw_sprite(
    ctx: &mut Context,
    assets: &GameAssets,
    sprite: &Sprite,
    world_coords: (f32, f32),
) -> GameResult {
    // Ekran genişlik ve yükseliğini al
    let (screen_w, screen_h) = world_coords;
    // nesnenin konumuna göre koordinatları hesaplar
    let pos = find_screen_coordinates(screen_w, screen_h, sprite.position);
    // asset'e ait çizilebilir imgeyi al(yani resmini :D)
    let image = assets.get_sprite_image(sprite);
    // pozisyon, rotasyon bilgilerini kullanarak parametreleri ayarla
    let drawparams = graphics::DrawParam::new()
        .dest(pos)
        .rotation(sprite.facing as f32)
        .offset(Point2 { x: 0.5, y: 0.5 });
    // nesneyi güncel context üstüne çiz
    draw(ctx, image, drawparams)?;

    Ok(())
}

fn find_screen_coordinates(
    screen_width: f32,
    screen_height: f32,
    point: Point2<f32>,
) -> Point2<f32> {
    let x = point.x + screen_width / 2.;
    let y = screen_height - (point.y + screen_height / 2.);
    Point2 { x, y }
}
