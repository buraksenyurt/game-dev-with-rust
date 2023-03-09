use crate::common::contants::{DOT_COLOR, POWERUP_COLOR, TILE_SIZE, WALL_COLOR};
use crate::common::direction::Direction;
use crate::common::utility::{calculate_square, load_texture};
use crate::engine::controller::Controller;
use crate::entities::cell::{Tile, TileType};
use graphics::{CircleArc, Context, Image, Rectangle};
use opengl_graphics::{GlGraphics, Texture};
use std::f64::consts::PI;

/*
   View veri yapısı ekran üstündeki çizim işlerinin ele alındığı yerdir.
   Hayaletlerin, pacman'in duvarların, yemlerin çizileceği kısım burasıdır
*/
pub struct View {
    x_offset: f64,
    y_offset: f64,
    ghosts: Vec<char>,
    pacman_textures: [Texture; 4],
}

impl View {
    pub fn new() -> Self {
        // Blinky, Pinky, Inky, Clyde
        let ghosts = vec!['B', 'P', 'I', 'C'];
        // Pacman hareket ederken hangi yöne döndüyse ona uygun bir texture gösterilecek
        let pacman_textures = [
            load_texture("pacman_right"),
            load_texture("pacman_down"),
            load_texture("pacman_left"),
            load_texture("pacman_up"),
        ];
        Self {
            x_offset: 0.,
            y_offset: 0.,
            ghosts,
            pacman_textures,
        }
    }

    /*
       Pencere üzerindeki çizim işlerinin gerçekleştirildiği yer.
    */
    pub fn draw(&self, controller: &Controller, c: &Context, g: &mut GlGraphics) {
        let offset = |mut a: [f64; 4]| {
            a[0] += self.x_offset;
            a[1] += self.y_offset;
            a
        };
        let mut x = 0.;
        let mut y = 0.;

        // Map nesnesi ile gelen matristeki tüm satırlar dolaşılmalaya başlıyor
        for line in controller.get_map().read_lines() {
            // satırdaki her bir Tile nesnesine bakılacak
            for tile in line.iter() {
                // Matching ile Tile türüne göre farklı bir çizim yapılıyor
                match tile {
                    Tile::Wall => {
                        let sqr = offset([
                            x + TILE_SIZE / 4.,
                            y + TILE_SIZE / 4.,
                            TILE_SIZE / 2.,
                            TILE_SIZE / 2.,
                        ]);
                        Rectangle::new(WALL_COLOR).draw(sqr, &c.draw_state, c.transform, g);
                    }
                    Tile::NotWall(TileType::Dot) => {
                        let sqr = offset([
                            x + TILE_SIZE * 0.4,
                            y + TILE_SIZE * 0.4,
                            TILE_SIZE / 6.,
                            TILE_SIZE / 6.,
                        ]);
                        Rectangle::new(DOT_COLOR).draw(sqr, &c.draw_state, c.transform, g);
                    }
                    Tile::NotWall(TileType::Powerup) => {
                        let sqr = offset([
                            x + TILE_SIZE * 0.375,
                            y + TILE_SIZE * 0.375,
                            TILE_SIZE / 4.,
                            TILE_SIZE / 4.,
                        ]);
                        CircleArc::new(POWERUP_COLOR, TILE_SIZE / 4., 0., 2. * PI).draw(
                            sqr,
                            &c.draw_state,
                            c.transform,
                            g,
                        );
                    }
                    _ => (),
                }
                x += TILE_SIZE;
            }
            y += TILE_SIZE;
            x = 0.;
        }

        // Oyun sahasındaki pacman'in güncel pozisyon ve gitmek istediği yön bilgileri alınır
        let (p, d) = controller.get_pacman();

        // Gitmek istediği yön bilgisine göre uygun texture çekilir
        let pacman_texture = match d {
            Direction::Up => &self.pacman_textures[3],
            Direction::Left => &self.pacman_textures[2],
            Direction::Down => &self.pacman_textures[1],
            Direction::Right => &self.pacman_textures[0],
        };
        // Pacman ilgili konuma gelecek şekilde çizdirilir
        Image::new()
            .rect(offset(calculate_square(TILE_SIZE, p.x, p.y)))
            .draw(pacman_texture, &c.draw_state, c.transform, g);
    }
}
