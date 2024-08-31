use bevy::prelude::*;

const DISTANCE_FROM_ORIGIN: f32 = 80.;

#[derive(Component, Debug, Default)]
pub struct Boundary;

pub struct OutOffBoundaryPlugin;
impl Plugin for OutOffBoundaryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, check_boundaries);
    }
}

fn check_boundaries(mut commands: Commands, query: Query<(Entity, &GlobalTransform, &Boundary)>) {
    for (entity, transform, _) in query.iter() {
        let distance_from_origin = transform.translation().distance(Vec3::ZERO);

        if distance_from_origin > DISTANCE_FROM_ORIGIN {
            commands.entity(entity).despawn_recursive();
            info!("{:?} Crate is out of the edge", entity);
        }
    }
}
