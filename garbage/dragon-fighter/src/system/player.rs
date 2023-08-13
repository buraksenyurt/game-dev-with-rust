use crate::constant::TILE_SIZE;
use crate::system::collision::in_collision;
use crate::system::texture::{spawn_sprite, AsciiSheet};
use crate::system::tiler::TileCollider;
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

// Plugin
pub struct PlayerPlugin;

// Oyuncu nesnesi için component tanımı
// Component olduğunu derive macro'su aracılığıyla belirtiyoruz.
// Ayrıca çalışma zamanında debug aracı tarafında izlenebilmesi içinde
// Inspectable özelliği ile donatıyoruz.
#[derive(Component, Inspectable)]
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
            .add_system(player_movement)
            .add_system(player_teleportation)
            .add_system(trail_camera);
    }
}

// Sembolik bir ışınlanma sistemi diyelim
// Sola veya sağa giderken aynı anda space tuşuna basınca turbo moda geçmiş gibi
// daha hızlı hareket ediyor
fn player_teleportation(
    mut player_query: Query<(&mut Player, &mut Transform)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
    wall_query: Query<&Transform, (With<TileCollider>, Without<Player>)>,
) {
    let (player, mut transform) = player_query.single_mut();
    let mut next_position = Vec2::default();
    if keyboard.pressed(KeyCode::Right) && keyboard.pressed(KeyCode::Space) {
        next_position.x += 3. * (player.speed * TILE_SIZE * time.delta_seconds());
    }
    if keyboard.pressed(KeyCode::Left) && keyboard.pressed(KeyCode::Space) {
        next_position.x += -3. * (player.speed * TILE_SIZE * time.delta_seconds());
    }
    let target = transform.translation + Vec3::new(next_position.x, 0.0, 0.0);
    if in_collision(target, &wall_query) {
        transform.translation = target;
    }
}

// Oyuncu hareket sistemi
// Generic olan ilk parametre Player component'i ve bir Transform nesnesi kullanır.
// Transform nesnesi ile oyuncunun o anki pozisyon bilgisini alabilir ve değiştirebiliriz.
// Böylece bu sistemin player bileşenini kullanması sağlanır.
// Sisteme giren ikinci parametre ise klavye tuşlarını ele almak içindir.
// 3ncü parametre delta_time bilgisini kullanmak için bir kaynaktır.
// 4ncü parametre ile duvarları alıp çarpışma kontrolü yapabiliriz
fn player_movement(
    mut player_query: Query<(&Player, &mut Transform)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
    wall_query: Query<&Transform, (With<TileCollider>, Without<Player>)>,
) {
    // sahada tek bir oyuncu olduğundan single_mut kullandık ve
    let (player, mut transform) = player_query.single_mut();
    let mut next_position = Vec2::default();
    // Basılan yön tuşlarına göre transform nesnesi üstünden oyuncuya bir hareket veriyoruz
    if keyboard.pressed(KeyCode::Up) {
        next_position.y += player.speed * TILE_SIZE * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::Down) {
        next_position.y -= player.speed * TILE_SIZE * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::Left) {
        next_position.x -= player.speed * TILE_SIZE * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::Right) {
        next_position.x += player.speed * TILE_SIZE * time.delta_seconds();
    }

    // Gidilecek bir sonraki lokasyonun x,y değerlerine göre bir duvara denk gelip
    // gelinmediğine bakılıyor.
    let target = transform.translation + Vec3::new(next_position.x, 0.0, 0.0);
    if in_collision(target, &wall_query) {
        transform.translation = target;
    }

    let target = transform.translation + Vec3::new(0.0, next_position.y, 0.0);
    if in_collision(target, &wall_query) {
        transform.translation = target;
    }
}

fn spawn_player(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let player = spawn_sprite(
        &mut commands,
        &ascii,
        14,
        Color::rgb(255., 215., 0.),
        Vec3::new(2. * TILE_SIZE, -2. * TILE_SIZE, 900.),
    );

    commands
        .entity(player)
        .insert(Name::new("AgentSmith"))
        .insert(Player { speed: 5. });
}

// Kamera takip sistemi. Yani oyuncuyu hareket ettirdiğimizde kamera onu merkezde tutacak
// şekilde ekran değişecek
fn trail_camera(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (Without<Player>, With<Camera>)>,
) {
    let player_transform = player_query.single();
    let mut camera_transform = camera_query.single_mut();
    // Kameratnın x,y konumunu oyuncunun x,y konumuna eşitlediğimiz kısım
    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
}
