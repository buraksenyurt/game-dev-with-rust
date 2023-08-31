use crate::game::live_data::resources::LiveData;
use crate::game::meteor::components::Meteor;
use crate::game::missile::components::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn move_missile(mut query: Query<(&mut Transform, &Missile)>, time: Res<Time>) {
    for (mut transform, missile) in query.iter_mut() {
        let direction = Vec3::new(missile.direction.x, missile.direction.y, 0.);
        transform.translation += direction * missile.speed * time.delta_seconds();
    }
}

pub fn check_outside_of_the_bounds(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<(Entity, &Transform), With<Missile>>,
) {
    let window = window_query.get_single().unwrap();
    for (entity, transform) in query.iter_mut() {
        if transform.translation.x > window.width() + 51. {
            commands.entity(entity).despawn();
            info!("Füze sınır dışına çıktı.");
        }
    }
}

pub fn detect_collision_with_meteors(
    mut meteors_query: Query<(&Transform, &mut Meteor)>,
    mut missiles_query: Query<(&Transform, &mut Missile)>,
    mut live_data: ResMut<LiveData>,
) {
    for (missile_transform, mut missile) in missiles_query.iter_mut() {
        for (meteor_transform, mut meteor) in meteors_query.iter_mut() {
            let distance = missile_transform
                .translation
                .distance(meteor_transform.translation);
            if distance <= 25. {
                live_data.last_meteor_strength = meteor.strength;
                info!("Meteorun gücü {}", meteor.current_hit_count);
                missile.disposable = true;
                missile.location = missile_transform.translation;
                match meteor.current_hit_count {
                    0 => {
                        live_data.exploded_meteors_count += 1;
                    }
                    _ => {
                        meteor.current_hit_count -= 1;
                    }
                }
            }
        }
    }
}

pub fn claim_hitted(
    mut commands: Commands,
    mut query: Query<(Entity, &Missile)>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    for (entity, missile) in query.iter_mut() {
        if missile.disposable {
            commands.entity(entity).despawn();

            let texture_handle = asset_server.load("sprites/explosion.png");
            let texture_atlas =
                TextureAtlas::from_grid(texture_handle, Vec2::new(60., 52.), 8, 1, None, None);
            let texture_atlas_handle = texture_atlases.add(texture_atlas);
            let animation_indices = ExplosionAnimation {
                first: 0,
                last: 7,
                disposable: false,
            };
            commands.spawn((
                SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle,
                    sprite: TextureAtlasSprite::new(animation_indices.first),
                    transform: Transform::from_translation(missile.location),
                    ..default()
                },
                animation_indices,
                ExplosionAnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
            ));
        }
    }
}

pub fn despawn_missiles(mut commands: Commands, query: Query<Entity, With<Missile>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn animate_explosion_sprites(
    time: Res<Time>,
    mut query: Query<(
        &mut ExplosionAnimation,
        &mut ExplosionAnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (mut indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            if sprite.index < indices.last {
                sprite.index += 1;
            } else {
                indices.disposable = true;
            }
        }
    }
}

pub fn despawn_explosions(
    mut commands: Commands,
    query: Query<(Entity, &ExplosionAnimation)>,
) {
    for (entity, component) in query.iter() {
        if component.disposable {
            commands.entity(entity).despawn();
        }
    }
}
