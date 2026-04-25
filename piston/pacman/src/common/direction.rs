/*
   Özellikle pacman'in oyuncu tarafından kontrolü söz konusu.
   Yön tuşları Direction enum sabiti ile kontrol edilmekte.
   to_vector fonksiyonu ile de hangi tuşa basıldığında ne yöne ne kadarlık bir hareket olacağı
   tuple olarak geriye döndürülmekte.
*/
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
