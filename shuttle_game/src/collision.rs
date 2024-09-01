use crate::planner::GameSystemSet;
use bevy::prelude::*;
use bevy::utils::HashMap;

#[derive(Component, Debug)]
pub struct Collider {
    pub entities: Vec<Entity>,
    pub radius: f32,
}

impl Collider {
    pub fn new(radius: f32) -> Self {
        Self {
            entities: vec![],
            radius,
        }
    }
}

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            detect_collisions.in_set(GameSystemSet::CollisionDetections),
        );
    }
}

fn detect_collisions(mut query: Query<(Entity, &GlobalTransform, &mut Collider)>) {
    let mut colliding_entities: HashMap<Entity, Vec<Entity>> = HashMap::new();

    for (entity_1, transform_1, collider_1) in query.iter() {
        for (entity_2, transform_2, collider_2) in query.iter() {
            if entity_1 != entity_2 {
                let distance = transform_1
                    .translation()
                    .distance(transform_2.translation());
                if distance < collider_1.radius + collider_2.radius {
                    colliding_entities
                        .entry(entity_1)
                        .or_insert_with(Vec::new)
                        .push(entity_2);
                }
            }
        }
    }

    for (entity, _, mut collider) in query.iter_mut() {
        collider.entities.clear();
        if let Some(collision) = colliding_entities.get(&entity) {
            collider.entities.extend(collision.iter().copied());
        }
    }
}
