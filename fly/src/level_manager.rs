use crate::constants::*;
use crate::game_play::Level;
use bevy::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub enum PlatformType {
    Ground,
    Platform,
    Crate,
}

#[derive(Clone, Debug, PartialEq)]
pub enum PlatformDirection {
    Vertical,
    Horizontal,
}
#[derive(Clone, Debug)]
pub struct PlatformData {
    pub position: Vec2,
    pub tile_count: usize,
    pub platform_type: PlatformType,
    pub direction: PlatformDirection,
}

#[derive(Component)]
pub struct Platform {
    pub platform_type: PlatformType,
}

pub fn get_level_data(level: Level) -> Option<Vec<PlatformData>> {
    match level {
        Level::FirstGate => Some(vec![
            // Main ground
            PlatformData {
                position: Vec2::new(-600.0, GROUND_LEVEL),
                tile_count: GROUND_TILE_COUNT,
                direction: PlatformDirection::Horizontal,
                platform_type: PlatformType::Ground,
            },
            PlatformData {
                position: Vec2::new(-600.0, GROUND_LEVEL),
                tile_count: 15,
                direction: PlatformDirection::Vertical,
                platform_type: PlatformType::Ground,
            },
            PlatformData {
                position: Vec2::new(-550.0, 100.0),
                tile_count: 3,
                direction: PlatformDirection::Horizontal,
                platform_type: PlatformType::Platform,
            },
            PlatformData {
                position: Vec2::new(-500.0, 180.0),
                tile_count: 4,
                direction: PlatformDirection::Horizontal,
                platform_type: PlatformType::Platform,
            },
            PlatformData {
                position: Vec2::new(-360.0, 140.0),
                tile_count: 2,
                direction: PlatformDirection::Horizontal,
                platform_type: PlatformType::Platform,
            },
            PlatformData {
                position: Vec2::new(-280.0, 60.0),
                tile_count: 4,
                direction: PlatformDirection::Horizontal,
                platform_type: PlatformType::Platform,
            },
            PlatformData {
                position: Vec2::new(-100.0, 80.0),
                tile_count: 3,
                direction: PlatformDirection::Horizontal,
                platform_type: PlatformType::Platform,
            },
            PlatformData {
                position: Vec2::new(20.0, 150.0),
                tile_count: 5,
                direction: PlatformDirection::Horizontal,
                platform_type: PlatformType::Platform,
            },
            PlatformData {
                position: Vec2::new(160.0, 60.0),
                tile_count: 3,
                direction: PlatformDirection::Horizontal,
                platform_type: PlatformType::Platform,
            },
            PlatformData {
                position: Vec2::new(320.0, 60.0),
                tile_count: 3,
                direction: PlatformDirection::Horizontal,
                platform_type: PlatformType::Platform,
            },
            PlatformData {
                position: Vec2::new(440.0, 150.0),
                tile_count: 5,
                direction: PlatformDirection::Horizontal,
                platform_type: PlatformType::Platform,
            },
        ]),
    }
}
