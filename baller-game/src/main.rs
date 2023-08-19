use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::random;

pub const PLAYER_SIZE: f32 = 64.;
pub const PLAYER_SPEED: f32 = 500.;
pub const ENEMY_SIZE: f32 = 64.;
pub const ENEMEY_SPEED: f32 = 200.;
pub const STAR_SIZE: f32 = 30.;
pub const NUMBER_OF_ENEMIES: usize = 6;
pub const NUMBER_OF_STARS: usize = 8;
pub const STAR_SPAWN_TIME: f32 = 1.;
pub const ENEMY_SPAWN_TIME: f32 = 2.5;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Score>()
        .init_resource::<HighScores>()
        .init_resource::<StarSpawnTimer>()
        .init_resource::<EnemySpawnTimer>()
        .add_event::<GameOver>()
        .add_systems(
            Startup,
            (
                spawn_camera_system,
                spawn_player_system,
                spawn_enemies_system,
                spawn_stars_system,
            ),
        )
        .add_systems(
            Update,
            (
                player_movement_system,
                check_player_movement_system,
                enemy_movement_system,
                enemy_bounces_system,
                check_enemy_movement_system,
                enemy_hit_player_system,
                refresh_enemies_system,
                player_hits_star_system,
                update_score_system,
                tick_star_spawn_timer_system,
                enemy_spawn_timer_system,
                spawn_enemy_after_time_finished_system,
                exit_game_system,
                handle_game_over_system,
                update_high_score_system,
                high_scores_updated_system,
            ),
        )
        .run();
}

#[derive(Resource, Debug)]
pub struct Score {
    pub value: u32,
}

impl Default for Score {
    fn default() -> Self {
        Self { value: 0 }
    }
}

#[derive(Resource)]
pub struct StarSpawnTimer {
    pub timer: Timer,
}

impl Default for StarSpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(STAR_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}

#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer,
}

impl Default for EnemySpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}

#[derive(Resource, Debug)]
pub struct HighScores {
    pub scores: Vec<(String, u32)>,
}

impl Default for HighScores {
    fn default() -> Self {
        Self { scores: Vec::new() }
    }
}

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Enemy {
    direction: Vec2, // Kırmızı topların anlık konumlarını saklamak için eklendi
}

// Sistemler arası taşınabilecek bir event veri yapısıdır
// Örneğin oyuncu kırmız toplardan birisine yakalanınca
// hit sistem'den game over sisteme bu veri yapısı ile data taşınabilir.
// game over sistemde de son skor bilgisi elde edilebilir.
#[derive(Event)]
pub struct GameOver {
    pub final_score: u32,
}

// AssetServer bir Resource'dur.
pub fn spawn_player_system(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    // Mavi topu ekranın tam ortasına yerleştirmek için pencerenin ölçüleri gerekcektir
    // Bu nedenle PrimaryWindows componentini taşıyan window nesnesi sorgulanır

    // Bir Entity nesnesine birden fazla component eklemek veya çıkarmak için Bundle'lar kullanılır.
    // Mavi topu konum bilgisi ile yükleyen bileşenleri Player ile ilişkilendirdik.
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 0.),
            texture: asset_server.load("sprites/ball_blue_large.png"),
            ..default()
        },
        Player {},
    ));
}

// Enemy olarak adledilen kırmızı topların üretildiği sistem
pub fn spawn_enemies_system(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    spawn_enemies(&mut commands, &window_query, &asset_server);
}

fn spawn_enemies(
    commands: &mut Commands,
    window_query: &Query<&Window, With<PrimaryWindow>>,
    asset_server: &Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    // Bu sefer rastgele x,y konumlarında 6 adet düşman nesnesi örneklenmekte
    for _ in 0..NUMBER_OF_ENEMIES {
        spawn_enemy(commands, asset_server, window);
    }
}

fn spawn_enemy(commands: &mut Commands, asset_server: &Res<AssetServer>, window: &Window) {
    let x = random::<f32>() * window.width();
    let y = random::<f32>() * window.height();

    let direction = Vec2::new(random::<f32>(), random::<f32>()).normalize();
    info!("Direction {:?}", direction);
    // Kırmızı top üretilirken rastgele bir konuma konur ve ayrıca,
    // rastgele x,y değerlerini baz olan bir yöne gidecek şekilde ayarlanır.
    // Direction değerini işaret eden vektörün birim vektöre dönüştürüldüğüne dikkat.
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(x, y, 0.),
            texture: asset_server.load("sprites/ball_red_large.png"),
            ..default()
        },
        Enemy { direction },
    ));
}

// Kırmızı topları(enemy) hareket ettiren sistem.
// Bu sefer Enemy olan bileşenlerin transform özelliklerini değiştirmemiz gerekiyor.
pub fn enemy_movement_system(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    // Ortamdaki enemy, transform bileşen eşleri mutable olarak dolaşılır
    for (mut transform, enemy) in enemy_query.iter_mut() {
        // Enemy spawn'lanırken bir direction verilmişti. Buna göre 3 boyutlu vektör örneklenir
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.);
        // translation bilgisi ENEMY_SPEED ve delta time değerleri kullanılarak değiştirilir.
        transform.translation += direction * ENEMEY_SPEED * time.delta_seconds();
    }
}

pub fn enemy_bounces_system(
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    // Kırmızı topun sınır x,y değerleri bulunur
    let window = window_query.get_single().unwrap();
    let (x_min, x_max) = (ENEMY_SIZE / 2., window.width() - ENEMY_SIZE / 2.);
    let (y_min, y_max) = (ENEMY_SIZE / 2., window.height() - ENEMY_SIZE / 2.);
    for (transform, mut enemy) in enemy_query.iter_mut() {
        let translation = transform.translation;
        // X ekseninde sınırlara gelindiyse enemy bileşeninin x yönü tersine çevrilir
        if translation.x < x_min || translation.x > x_max {
            enemy.direction.x *= -1.;
        }
        // y ekseninde sınırlara gelindiyse enemy bileşeninin y yönü tersine çevrilir
        if translation.y < y_min || translation.y > y_max {
            enemy.direction.y *= -1.;
        }
    }
}

pub fn check_enemy_movement_system(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let x_min = ENEMY_SIZE / 2.;
    let x_max = window.width() - ENEMY_SIZE / 2.;
    let y_min = ENEMY_SIZE / 2.;
    let y_max = window.height() - ENEMY_SIZE / 2.;

    for mut transform in enemy_query.iter_mut() {
        let mut translation = transform.translation;

        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }

        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }
        transform.translation = translation;
    }
}

pub fn spawn_camera_system(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    // Kameranın konumunu ayarlamak için yine ekran bilgisine ihtiyacımız olacak
    let window = window_query.get_single().unwrap();

    // 2D kamerayı da mavi topun olduğu koordinata göre konumlandırıyoruz
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 0.),
        ..default()
    });
}

// Oyuncunun hareket ettirildiği sistem
// Klavye yön tuşlarına göre hareket edeceğinden Input<KeyCode> bir Resource olarak alınır
// Hareket hesaplamasında delta time kullanılacağından Time verisi de Resource olarak alınır.
// Tabi Transform bileşenlerinden oyuncu Entity'si ile yüklenenler sorgulanır
pub fn player_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    // Eğer ortamda hareket edebilecek bir player nesnesi varsa
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;
        // Basılan tuşa göre yeni bir Vektör hareketi ayarlanır
        if keyboard_input.pressed(KeyCode::Left) {
            direction += Vec3::new(-1., 0., 0.);
        }
        if keyboard_input.pressed(KeyCode::Right) {
            direction += Vec3::new(1., 0., 0.);
        }
        if keyboard_input.pressed(KeyCode::Up) {
            direction += Vec3::new(0., 1., 0.);
        }
        if keyboard_input.pressed(KeyCode::Down) {
            direction += Vec3::new(0., -1., 0.);
        }

        // Bir hareket söz konusu ise vektör birim vektöre çevirilir(Normalize)
        if direction.length() > 0. {
            direction = direction.normalize();
        }

        // bileşenin konumu direction, player_speed ve delta time süresine göre tekrardan ayarlanır.
        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

pub fn check_player_movement_system(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();
        let (x_min, x_max) = (PLAYER_SIZE / 2., window.width() - PLAYER_SIZE / 2.);
        let (y_min, y_max) = (PLAYER_SIZE / 2., window.height() - PLAYER_SIZE / 2.);
        let mut translation = player_transform.translation;

        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }

        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        player_transform.translation = translation;
    }
}

// Mavi topun kırmızı toplara çarptığı durumları ele alan sistem.
// Oyuncuyu temsil eden mavi topun bir kırmızı topa çarpması oyunun bitmesi anlamına da geliyor.
pub fn enemy_hit_player_system(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    mut game_over_event_writer: EventWriter<GameOver>,
    score: Res<Score>,
) {
    // Sahadaki oyuncuyu transform bileşeni ile birlikte ele alıyoruz
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
        // tüm kırmızı toplar için bir iterasyon var ve transform bileşenlerini ele alıyoruz
        for enemy_transform in enemy_query.iter() {
            // çarpışma kontrolü yapılıyor.
            // toplar birer daire olduğundan aralarındaki mesafenin yarıçapları toplamı ile olan
            // mukayesesi sayesinde çarpışma olup olmadığını anlayabiliriz.
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);
            let player_radius = PLAYER_SIZE / 2.;
            let enemy_radius = ENEMY_SIZE / 2.;
            if distance < player_radius + enemy_radius {
                info!("Game over!Catched with red ball");
                // Oyuncunun entity nesnesi sistemden kaldırılıyor
                commands.entity(player_entity).despawn();
                // Oyuncu kırmız toplardan birisine yakalnırsa
                // bu sistemden GameOver isimli bir event fırlatıyoruz.
                // Bunu handle_game_over_system isimli sistem dinliyor olacak.
                // Ayrıca event ile birlikte güncel skor değerini de taşımaktayız.
                // Böylece bu değeri Reader event içinden okuyabiliriz
                game_over_event_writer.send(GameOver {
                    final_score: score.value,
                });
            }
        }
    }
}

pub fn refresh_enemies_system(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut enemies_query: Query<Entity, With<Enemy>>,
    mut stars_query: Query<Entity, With<Star>>,
    player_query: Query<Entity, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>,
) {
    if keyboard_input.pressed(KeyCode::F5) {
        score.value = 0;
        for enemy in enemies_query.iter_mut() {
            commands.entity(enemy).despawn();
        }
        for star in stars_query.iter_mut() {
            commands.entity(star).despawn();
        }
        if let Ok(player) = player_query.get_single() {
            commands.entity(player).despawn();
        }
        let window = window_query.get_single().unwrap();
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 0.),
                texture: asset_server.load("sprites/ball_blue_large.png"),
                ..default()
            },
            Player {},
        ));
        spawn_enemies(&mut commands, &window_query, &asset_server);
        spawn_stars(&mut commands, &window_query, &asset_server);
    }
}

#[derive(Component)]
pub struct Star {}

pub fn spawn_stars_system(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    spawn_stars(&mut commands, &window_query, &asset_server);
}

fn spawn_stars(
    commands: &mut Commands,
    window_query: &Query<&Window, With<PrimaryWindow>>,
    asset_server: &Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    for _ in 0..NUMBER_OF_STARS {
        spawn_star(commands, asset_server, window);
    }
}

fn spawn_star(commands: &mut Commands, asset_server: &Res<AssetServer>, window: &Window) {
    let x = random::<f32>() * window.width();
    let y = random::<f32>() * window.height();
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(x, y, 0.),
            texture: asset_server.load("sprites/star.png"),
            ..default()
        },
        Star {},
    ));
}

pub fn player_hits_star_system(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    star_query: Query<(Entity, &Transform), With<Star>>,
    mut score: ResMut<Score>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (star_entity, star_transformation) in star_query.iter() {
            let distance = player_transform
                .translation
                .distance(star_transformation.translation);
            let player_radius = PLAYER_SIZE / 2.;
            let star_radius = STAR_SIZE / 2.;
            if distance < player_radius + star_radius {
                score.value += 10;
                // info!(
                //     "Player catch a start! Current Score {}",
                //     score.value.to_string()
                // );
                commands.entity(star_entity).despawn();
            }
        }
    }
}

pub fn update_score_system(score: Res<Score>) {
    if score.is_changed() {
        info!("Current score {}", score.value.to_string());
    }
}

// Sistemdeki yıldızlar için bir timer nesnesi uygulanıyor
pub fn tick_star_spawn_timer_system(mut star_timer: ResMut<StarSpawnTimer>, time: Res<Time>) {
    star_timer.timer.tick(time.delta());
}

pub fn spawn_star_after_time_finished_system(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    star_timer: Res<StarSpawnTimer>,
) {
    // star saati için belirlenen süre geçmişse
    if star_timer.timer.finished() {
        let window = window_query.get_single().unwrap();
        spawn_star(&mut commands, &asset_server, window);
    }
}

// Sistemdeki kırmızı toplar için bir timer nesnesi uygulanıyor
pub fn enemy_spawn_timer_system(mut star_timer: ResMut<EnemySpawnTimer>, time: Res<Time>) {
    star_timer.timer.tick(time.delta());
}

pub fn spawn_enemy_after_time_finished_system(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    enemy_timer: Res<EnemySpawnTimer>,
) {
    // enemy için belirlenen süre geçmişse
    if enemy_timer.timer.finished() {
        let window = window_query.get_single().unwrap();
        spawn_enemy(&mut commands, &asset_server, window);
    }
}

// ESC tuşuna basıldığında oyundan çıkılması için kullanılan sistemdir.
// EventWriter ile başka bir sisteme event bildirimi yapabiliriz.
// Buna benzer şekilde EventReader ile de sisteme gelen event'leri yakalayabiliriz.
// Bevy tarafında event'ler sistemler arasında veri taşımak için kullanılır.
// Böylece bir sistemde meydana gelen bir aksiyon karşılığında bir event verisi oluşturup
// bunu kullanacak başka bir sisteme yollayabiliriz. Cool!
pub fn exit_game_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        event_writer.send(AppExit);
    }
}

// GameOver isimli bir event oluştuğunda ele alınan sistem
pub fn handle_game_over_system(mut game_over_event_reader: EventReader<GameOver>) {
    for event in game_over_event_reader.iter() {
        info!("Player's final score is {}", event.final_score.to_string());
    }
}

// Bu sistem de GameOver event'lerini dinler.
// Dolayısıyla bir event birden fazla sistem tarafından okunabilir.
pub fn update_high_score_system(
    mut game_over_event_reader: EventReader<GameOver>,
    mut high_scores: ResMut<HighScores>,
) {
    for event in game_over_event_reader.iter() {
        high_scores
            .scores
            .push(("Player".to_string(), event.final_score));
    }
}

pub fn high_scores_updated_system(high_scores: Res<HighScores>) {
    if high_scores.is_changed() {
        info!("High scores changed to {:?}", high_scores);
    }
}
