/*
   Özellikle pacman'in oyuncu tarafından kontrolü söz konusu.
   Yön tuşları Direction enum sabiti ile kontrol edilmekte.
   to_vector fonksiyonu ile de hangi tuşa basıldığında ne yöne ne kadarlık bir hareket olacağı
   tuple olarak geriye döndürülmekte.
*/
#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn to_vector(self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}
