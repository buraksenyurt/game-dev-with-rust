use crate::collider::handle_collisions;
use crate::constant::{COMMON_FPS, MAX_RADIUS, MAX_ROCK_COUNT, MIN_RADIUS};
use crate::game_assets::GameAssets;
use crate::input_state::InputState;
use crate::player::{fire_at_will, handle_input};
use crate::sprite::Sprite;
use crate::sprite_builder::{create_random_rocks, create_sprite};
use crate::sprite_type::SpriteType;
use ggez::event::{EventHandler, KeyCode, KeyMods};
use ggez::graphics::{draw, Color, DrawParam, Font, PxScale};
use ggez::mint::Point2;
use ggez::timer::check_update_time;
use ggez::winit::event::VirtualKeyCode;
use ggez::{event, graphics, timer, Context, GameResult};

// Oyunun herhangi bir andaki varlık durumunu tutacak veri yapısı.
// Belli bir anda oyun sahasındaki oyuncu, kayalar, atılan şutlar,
// skor bilgisi, seviye, assetler vs gibi bilgileri taşır.
pub struct MainState {
    pub player: Sprite,
    pub shots: Vec<Sprite>,
    pub rocks: Vec<Sprite>,
    assets: GameAssets,
    screen_width: f32,
    screen_height: f32,
    pub score: i32,
    input_state: InputState,
    pub player_shot_timeout: f32,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {


        let assets = GameAssets::new(ctx)?;
        let player = create_sprite(SpriteType::Player);
        let rocks = create_random_rocks(
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
            score: 0,
            input_state: InputState::default(),
            player_shot_timeout: 0.,
        };
        Ok(ms)
    }
}

// Ana oyun nesnesi için gerekli temel olaylara ait trait'ler kodlanır.
impl EventHandler for MainState {
    // State güncellemelerinin ele alındığı fonksiyondur
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        // Güncel context için Frame Per Second süresi gelmişse
        while check_update_time(ctx, COMMON_FPS) {
            let seconds = 1. / (COMMON_FPS as f32);

            handle_input(&mut self.player, &self.input_state, seconds);
            self.player_shot_timeout -= seconds;
            if self.input_state.fire && self.player_shot_timeout < 0. {
                fire_at_will(self)?;
            }

            self.player.update_position(seconds);
            self.player
                .wrap_position(self.screen_width, self.screen_height);

            for s in &mut self.shots {
                s.update_position(seconds);
                s.wrap_position(self.screen_width, self.screen_height);
                s.life -= seconds;
            }

            for r in &mut self.rocks {
                r.update_position(seconds);
                r.wrap_position(self.screen_width, self.screen_height);
            }

            // Çarpışma hesaplamaları
            handle_collisions(self)?;

            // Çarpışma hesaplamalarına göre kaya ve atışlar state'ten kaldırılır
            self.shots.retain(|shot| shot.life > 0.0);
            self.rocks.retain(|rock| rock.life > 0.0);

            if self.player.life <= 0. {
                println!("Üzgünüm dostum ama kayaya tosladın :D !");
                event::quit(ctx);
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::BLACK);

        let game_assets = &self.assets;
        let coordinates = (self.screen_width, self.screen_height);

        // Oyuncu çizilir
        let hero = &self.player;
        draw_sprite(ctx, game_assets, hero, coordinates)?;

        // Atışlar çizdirilir.
        for s in &self.shots {
            draw_sprite(ctx, game_assets, s, coordinates)?;
        }

        // rastgele konumlanan kayalar çizdirilir.
        // Main State oluşturulurken belirlenen her bir kaya nesnesi için draw operasyonu çağrılır.
        for r in &self.rocks {
            draw_sprite(ctx, game_assets, r, coordinates)?;
        }

        // Skor tabelası çizimi.
        // Metin kutusunu hazırla
        let mut score_box = graphics::Text::new(format!("Skor : {}", &self.score));
        // Fontu ayarla
        score_box.set_font(Font::default(), PxScale::from(24.));
        // İlk pozisyonunu ekrana göre ortalayarak ayarla
        let mut score_box_position = Point2 {
            x: &self.screen_width * 0.5,
            y: 25.,
        };
        // Kutunun boyutlarını hesaba katarak,
        let score_box_dimension = score_box.dimensions(ctx);
        // x,y koordinatlarını yeniden düzenle
        score_box_position.x -= score_box_dimension.w as f32 * 0.5;
        score_box_position.y -= score_box_dimension.h as f32 * 0.5;
        // Metin kutusunu ekrana çiz
        draw(ctx, &score_box, DrawParam::new().dest(score_box_position))?;

        graphics::present(ctx)?;

        // İşletim sistemine şimdilik CPU ile olan işimizin bittiğini
        // ama tekrar geri geleceğimizi söylüyoruz
        timer::yield_now();

        Ok(())
    }

    // Klavye tuşlarına basıldığında input state bilgilerini değiştireceğimiz fonksiyon
    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::W => {
                //println!("W tuşuna basılıyor... y:{}", self.input_state.y_axis);
                self.input_state.y_axis = 1.;
            }
            KeyCode::A => {
                self.input_state.x_axis = -1.;
            }
            KeyCode::D => {
                self.input_state.x_axis = 1.;
            }
            KeyCode::Space => {
                self.input_state.fire = true;
            }
            KeyCode::Escape => {
                event::quit(ctx);
            }
            _ => (),
        }
    }

    // Oyuncu ilgili klavye tuşlarını bıraktığında input state başlangıç konumuna getirilir.
    // Böylece sola, sağa veya yukarı döner durumdan durağan hale,
    // ateş eder konumdan ateş etmeyen hale geçildiğini anlarız.
    fn key_up_event(&mut self, _ctx: &mut Context, keycode: VirtualKeyCode, _keymods: KeyMods) {
        match keycode {
            KeyCode::W => {
                //println!("W tuşu bırakıldı... y:{}", self.input_state.y_axis);
                self.input_state.y_axis = 0.;
            }
            KeyCode::A | KeyCode::D => {
                self.input_state.x_axis = 0.;
            }
            KeyCode::Space => {
                self.input_state.fire = false;
            }
            _ => (),
        }
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
    let drawparams = DrawParam::new()
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
