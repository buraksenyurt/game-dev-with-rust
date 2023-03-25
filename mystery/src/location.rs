// Oyundaki nesnelerin konumunu tutan veri yapısı.
#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Location(pub usize);

pub const LOC_ROOM: usize = 0;
pub const LOC_SPACESHIP: usize = 1;
pub const LOC_LAKEHOUSE: usize = 2;
pub const LOC_RADIO: usize = 100;
pub const LOC_LOGBOOK: usize = 101;
pub const LOC_ROBOT: usize = 102;
pub const LOC_BOOK: usize = 103;
pub const LOC_DOG: usize = 104;
