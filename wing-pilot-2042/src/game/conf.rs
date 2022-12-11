use macroquad::prelude::Conf;

pub fn window_conf() -> Conf {
    Conf {
        window_title: "WingMan 2042".to_owned(),
        fullscreen: false,
        window_width: 1024,
        window_height: 768,
        window_resizable: false,
        ..Default::default()
    }
}
