use crate::game_data::Score;
use crate::planner::GameSystemSet;
use crate::shuttle::Damage;
use bevy::prelude::*;
use bevy::utils::HashMap;

#[derive(Component, Debug)]
pub struct CollisionDamage {
    pub amount: u8,
}
#[derive(Event, Debug)]
pub struct CollisionEvent {
    pub source: Entity,
    pub collided: Entity,
}

impl CollisionEvent {
    pub fn new(source: Entity, collided: Entity) -> Self {
        Self { source, collided }
    }
}

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
        )
        .add_systems(
            Update,
            apply_collision_changes.in_set(GameSystemSet::DespawnEntities),
        )
        .add_event::<CollisionEvent>();
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

fn apply_collision_changes(
    mut collision_event_reader: EventReader<CollisionEvent>,
    query: Query<&Damage>,
    mut score: ResMut<Score>,
) {
    for &CollisionEvent { source, collided } in collision_event_reader.read() {
        let Ok(damage) = query.get(collided) else {
            continue;
        };

        score.player_damage -= damage.value;
    }
}
