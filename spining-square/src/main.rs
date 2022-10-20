use crate::app::Application;
use crate::constant::{SCREEN_HEIGHT, SCREEN_WIDTH};
use glutin_window::GlutinWindow as Window;
use glutin_window::OpenGL;
use opengl_graphics::GlGraphics;
use piston::{EventSettings, Events, RenderEvent, UpdateEvent, WindowSettings};

mod app;
mod constant;

/*
   Piston'un hello world örneğinde ekranın ortasında yer alan bir karenin
   kendi ekseni etrafında dönmesi işlemi icra ediliyor.
*/
fn main() {
    // OpenGL Kütüphahensini baz alıyoruz
    let opengl = OpenGL::V4_5;
    // ana pencere nesnesini oluşturuyoruz

    let mut window: Window = WindowSettings::new("Başı dönen kare", [SCREEN_WIDTH, SCREEN_HEIGHT])
        .graphics_api(opengl)
        .exit_on_esc(true) // ESC tuşuna basınca çıkılacak mı?
        .build()
        .unwrap();

    // Uygulama nesnesini oluşturuyoruz (İlerleyen zamanlarda tahminimce bu Game nesnemiz olacak)
    let mut app = Application::new(GlGraphics::new(opengl), 0.);
    let mut events = Events::new(EventSettings::new());

    // Oyun döngüsü
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }
        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}
