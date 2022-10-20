use crate::constant::{GOLD, OCEAN_BLUE};
use opengl_graphics::GlGraphics;
use piston::{RenderArgs, UpdateArgs};

// Open GL grafik kütüphanesini kullanacak olan ve döndürme bilgisi tutan veri yapısı
pub struct Application {
    graphic: GlGraphics,
    rotation: f64,
}

impl Application {
    pub fn new(graphic: GlGraphics, rotation: f64) -> Self {
        Self { graphic, rotation }
    }

    // Ekrana çizdirme işlemlerinin yürütüldüğü render fonksiyonu
    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        // 0,0 koordinatlarına 64 pixel boyunda bir kare tanımlanıyor
        let square = rectangle::square(0., 0., 64.);
        let rotation = self.rotation;
        // kareyi ekranın tam ortasına yerleştirme için x,y koordinatları hesaplanıyor
        let (x, y) = (args.window_size[0] * 0.5, args.window_size[1] * 0.5);

        // çizdirme işlemi yapılıyor
        self.graphic.draw(args.viewport(), |context, gl| {
            // Ekran okyanus mavisi rengi ile temizleniyor
            clear(OCEAN_BLUE, gl);
            // radyan cinsinden gelen rotasyon değeri ve x,y bilgilerine göre bir yer değişimi
            // ve döndürme bilgisi ayarlanıyor
            let transform = context
                .transform
                .trans(x, y) // konum yeni yerine ayarlanıyor
                .rot_rad(rotation) // döndürme işlemi
                .trans(-32., 32.);
            // dörten çiziliyor
            rectangle(GOLD, square, transform, gl);
        });
    }

    // FPS diliminde güncelleme işini üstlenen fonksiyon
    pub fn update(&mut self, args: &UpdateArgs) {
        // döndürme değeri hesaplıyor. dt=delta time
        self.rotation += 2. * args.dt;
    }
}
