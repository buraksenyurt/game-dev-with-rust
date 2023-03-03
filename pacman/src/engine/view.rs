use crate::common::contants::{DOT_COLOR, POWERUP_COLOR, TILE_SIZE, WALL_COLOR};
use crate::engine::controller::Controller;
use crate::entities::cell::{Tile, TileType};
use graphics::types::Color;
use graphics::{CircleArc, Context, Rectangle, Text};
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
}

impl View {
    pub fn new() -> Self {
        // Blinky, Pinky, Inky, Clyde
        let mut ghosts = vec!['B', 'P', 'I', 'C'];
        Self {
            x_offset: 0.,
            y_offset: 0.,
            ghosts,
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
    }
}
