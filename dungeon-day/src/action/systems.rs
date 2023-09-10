use crate::action::models::*;
use crate::action::*;
use crate::board::components::*;
use crate::parts::components::*;
use crate::player::components::*;
use crate::utility::ORTHO_DIRECTIONS;
use bevy::prelude::*;
use rand::prelude::*;
use rand::thread_rng;

pub fn execute_actions(world: &mut World) {
    let Some(mut queue) = world.get_resource_mut::<ActorQueue>() else { return };
    let Some(entity) = queue.0.pop_front() else {
        world.send_event(ActionsCompleteEvent);
        return;
    };
    let Some(mut actor) = world.get_mut::<Actor>(entity) else { return };
    let Some(action) = actor.0.take() else { return };

    if !action.apply(world) && world.get::<Player>(entity).is_some() {
        world.send_event(InvalidPlayerActionEvent);
        return;
    }
    world.send_event(NextActorEvent);
}

pub fn populate_actor_queue(
    query: Query<Entity, (With<Actor>, Without<Player>)>,
    mut queue: ResMut<ActorQueue>,
) {
    queue.0.extend(query.iter());
}

pub fn prepare_walk(mut query: Query<(&Position, &mut Actor), With<Walk>>, queue: Res<ActorQueue>) {
    if let Some(entity) = queue.0.get(0) {
        if let Ok((position, mut actor)) = query.get_mut(*entity) {
            let mut rng = thread_rng();
            let direction = ORTHO_DIRECTIONS.choose(&mut rng).unwrap();
            actor.0 = Some(Box::new(WalkAction {
                entity: *entity,
                loc: position.value + *direction,
            }));
        }
    }
}
