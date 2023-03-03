mod common;
mod engine;
mod entities;
mod tests;

use crate::common::contants::{TILE_MAP_HEIGHT, TILE_MAP_WIDTH, TILE_SIZE};
use crate::engine::controller::Controller;
use crate::engine::view::View;
use crate::entities::pacman::Pacman;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::RenderEvent;
use piston_window::WindowSettings;

fn main() {
    // Örnekte grafik çizimleriçin OpenGL kütüphanesinden yararlanılıyor
    let opengl = OpenGL::V4_5;

    // Oyun penceresi ayarlanıyor.
    // Başlık, boyutlar, hangi grafik kütüphanesi ile çizileceği,
    // boyutlandırılabilecek mi, Esc tuşuna basınca çıkılacak mı vs
    let mut window: GlutinWindow = WindowSettings::new(
        "Pacman Clone",
        [
            TILE_MAP_WIDTH as f64 * TILE_SIZE,
            TILE_MAP_HEIGHT as f64 * TILE_SIZE,
        ],
    )
    .graphics_api(opengl)
    .resizable(false)
    .exit_on_esc(true)
    .build()
    .unwrap();
    // Çizim işlerini üstlenen graphic nesnesi
    let mut painter = GlGraphics::new(opengl);
    // Oyunu kontrol eden nesne
    let mut controller = Controller::new(Pacman::default());
    // Ekrandaki çizim işlerini üstlenen nesne
    let view = View::new();
    let mut events = Events::new(EventSettings::new());
    // Pencere üstünde gerçekleşen olaylar olduğu sürece devam edecek döngü
    while let Some(event) = events.next(&mut window) {
        // // bir Render argümanı varsa
        if let Some(render) = event.render_args() {
            // painter nesnesinden de yararlanarak çizim işlemlerini gerçekleştir
            painter.draw(render.viewport(), |c, g| {
                graphics::clear([1.; 4], g);
                // asıl çizim operasyonlarını icra eden nesnenin draw fonksiyonu
                view.draw(&controller, &c, g);
            })
        } else {
            if controller.event(&event) {
                break;
            }
        }
    }
}
