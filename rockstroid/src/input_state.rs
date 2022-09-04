// Oyuncu sol sağ yön tuşları kendi ekseninde dönerken space tuşu ile ateş edebiliyor
// Bu klavye girdilerini state-based ve driver bağımsız kullanabilmek için
// aşağıdaki veri yapısı kullanılmakta.
pub struct InputState {
    pub x_axis: f32,
    pub y_axis: f32,
    pub fire: bool,
}

impl Default for InputState {
    fn default() -> Self {
        Self {
            x_axis: 0.,
            y_axis: 0.,
            fire: false,
        }
    }
}
