use crate::common::position::Position;

pub enum GhostMode {
    Pursue,
    Scatter,
}

pub enum GhostType {
    Blinky,
    Clyde,
    Inky,
    Pinky,
}

pub struct Ghost {
    kind: GhostType,
    pos: Position,
    last_pos: Position,
    house_timer: u16,
}

impl Ghost {
    pub fn new(kind: GhostType) -> Self {
        let start_p = match kind {
            GhostType::Blinky => Position::new(15, 15),
            GhostType::Pinky => Position::new(15, 14),
            GhostType::Inky => Position::new(14, 15),
            GhostType::Clyde => Position::new(14, 14),
        };
        Ghost {
            pos: start_p,
            last_pos: Position::new(i32::MIN, i32::MIN),
            house_timer: match kind {
                GhostType::Blinky => 2,
                GhostType::Pinky => 10,
                GhostType::Inky => 20,
                GhostType::Clyde => 30,
            },
            kind,
        }
    }
}

pub struct GhostController {
    ghosts: [Ghost; 4],
    mode: GhostMode,
    timer: u16,
    num_scatters: u8,
}

impl GhostController {
    pub fn new() -> Self {
        Self {
            ghosts: [
                Ghost::new(GhostType::Blinky),
                Ghost::new(GhostType::Clyde),
                Ghost::new(GhostType::Inky),
                Ghost::new(GhostType::Pinky),
            ],
            mode: GhostMode::Pursue,
            timer: 0,
            num_scatters: 2,
        }
    }
}
