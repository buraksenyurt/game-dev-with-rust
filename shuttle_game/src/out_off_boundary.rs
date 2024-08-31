use bevy::prelude::*;

const DISTANCE_X_SCALE: f32 = 60.0;
const DISTANCE_Z_SCALE: f32 = 60.0;

#[derive(Component, Debug, Default)]
pub struct Boundary;

pub struct OutOffBoundaryPlugin;
impl Plugin for OutOffBoundaryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, check_boundaries);
    }
}

fn check_boundaries(mut commands: Commands, query: Query<(Entity, &Transform, &Boundary)>) {
    for (entity, transform, _) in query.iter() {
        let distance_x = transform.translation.x.abs();
        let distance_z = transform.translation.z.abs();

        if distance_x > DISTANCE_X_SCALE || distance_z > DISTANCE_Z_SCALE {
            commands.entity(entity).despawn();
            info!("{:?} Crate is out of the edge", entity);
        }
    }
}
