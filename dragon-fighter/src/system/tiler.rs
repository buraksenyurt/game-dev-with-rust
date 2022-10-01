use crate::constant::TILE_SIZE;
use crate::system::texture::{spawn_sprite, AsciiSheet};
use crate::{App, Commands};
use bevy::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct TilerPlugin;

impl Plugin for TilerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_room);
    }
}

// Oyun sahasının zemin dokusunu hazırlayan fonksiyon
fn create_room(mut commands: Commands, ascii: Res<AsciiSheet>) {
    // Fonksiyon assets klasöründeki room_map isimli bir text dosyayı kullanıyor
    // Dosya içerisindeki # karakterlerini yakalayıp bunlar için birer tile nesnesi oluşturuyoruz.
    // # olanlar duvar . olanlar ise yol gibi düşünülebilir.
    let f = File::open("assets/room_map.txt").expect("Dosya okunamadı");
    let mut tiles = Vec::new();

    // Önce satırları dolaşalım
    for (y, line) in BufReader::new(f).lines().enumerate() {
        // Bir satır söz konusu ise de
        if let Ok(line) = line {
            // satırdaki karakterleri dolaşalım
            for (x, c) in line.chars().enumerate() {
                // Bir tile oluşturalım
                let tile = spawn_sprite(
                    &mut commands,
                    &ascii,
                    c as usize,
                    Color::rgb(0.5, 0.5, 0.5),
                    Vec3::new(x as f32 * TILE_SIZE, -(y as f32) * TILE_SIZE, 100.),
                );
                tiles.push(tile);
            }
        }
    }
}
