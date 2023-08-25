use crate::missile::components::Missile;
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
