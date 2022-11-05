use crate::{
    get_texture, rand, Bullet, GameState, Garrison, Tank, TextureType, DEFAULT_MARGIN,
    MAX_GARRISON_COUNT,
};
use macroquad::prelude::{get_time, screen_height, screen_width, vec2};

#[derive(Clone)]
pub struct Game {
    pub state: GameState,
    pub bullets: Vec<Bullet>,
    pub army: Vec<Garrison>,
    pub player: Tank,
    pub last_shot: f64,
    pub score: usize,
    pub counter: usize,
}

impl Game {
    pub async fn init() -> Self {
        let tank_texture = get_texture(TextureType::Tank).await;
        let garrison_texture = get_texture(TextureType::Garrison).await;
        let bullet_texture = get_texture(TextureType::Bullet).await;

        let mut new_army: Vec<Garrison> = Vec::new();
        let mut g_counter = 0;
        loop {
            if g_counter == MAX_GARRISON_COUNT {
                break;
            }

            let p = vec2(
                rand::gen_range(
                    DEFAULT_MARGIN,
                    screen_width() - bullet_texture.width() + DEFAULT_MARGIN,
                ),
                rand::gen_range(
                    DEFAULT_MARGIN,
                    screen_height() - bullet_texture.height() + DEFAULT_MARGIN,
                ),
            );
            //println!("{}", p);
            let g = Garrison::new(g_counter, p, garrison_texture);
            if new_army
                .iter()
                .any(|s: &Garrison| (s.position - g.position).length() < g.texture.width())
            {
                continue;
            }
            new_army.push(g);
            g_counter += 1;
        }

        Self {
            state: GameState::Menu,
            bullets: Vec::new(),
            army: new_army,
            player: Tank::new(tank_texture),
            last_shot: get_time(),
            score: 0,
            counter: 0,
        }
    }
}
