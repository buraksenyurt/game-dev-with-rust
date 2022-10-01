use crate::constant::TILE_SIZE;
use crate::system::texture::AsciiSheet;
use bevy::prelude::*;

// Plugin
pub struct PlayerPlugin;

// Oyuncu nesnesi için component tanımı
#[derive(Component)]
pub struct Player {
    pub speed: f32,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        // Oyuncu için tasarlanan bu plugin'e oyuncu ile alakalı sistemleri aşağıdaki gibi
        // ekleyebiliriz. Oyuncuyu oluşturan sistem, hareket sistemi vb
        // Böylece main fonksiyonunda birçok sistem tanımlamak yerine oyuncu plugin nesnesini
        // kullanabiliriz.
        app.add_startup_system(spawn_player)
            .add_system(player_movement);
    }
}

// Oyuncu hareket sistemi
// Generic olan ilk parametre Player component'i ve bir Transform nesnesi kullanır.
// Transform nesnesi ile oyuncunun o anki pozisyon bilgisini alabilir ve değiştirebiliriz.
// Böylece bu sistemin player bileşenini kullanması sağlanır.
// Sisteme giren ikinci parametre ise klavye tuşlarını ele almak içindir.
// Son olarak dahil edilen 3ncü parametre delta_time bilgisini kullanmak için bir kaynaktır.
fn player_movement(
    mut player_query: Query<(&Player, &mut Transform)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    // sahada tek bir oyuncu olduğundan single_mut kullandık ve
    let (player, mut transform) = player_query.single_mut();

    // Basılan yön tuşlarına göre transform nesnesi üstünden oyuncuya bir hareket veriyoruz
    if keyboard.pressed(KeyCode::Up) {
        transform.translation.y += player.speed * TILE_SIZE * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::Down) {
        transform.translation.y -= player.speed * TILE_SIZE * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::Left) {
        transform.translation.x -= player.speed * TILE_SIZE * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::Right) {
        transform.translation.x += player.speed * TILE_SIZE * time.delta_seconds();
    }
}

fn spawn_player(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let mut player_texture = TextureAtlasSprite::new(14);
    player_texture.color = Color::rgb(255., 215., 0.);
    player_texture.custom_size = Some(Vec2::splat(TILE_SIZE));
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: player_texture,
            texture_atlas: ascii.0.clone(),
            transform: Transform {
                translation: Vec3::new(2. * TILE_SIZE, -2. * TILE_SIZE, 900.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("AgentSmith"))
        .insert(Player { speed: 5. });
}
