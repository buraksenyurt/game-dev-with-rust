use crate::constant::TILE_SIZE;
use crate::system::player::{Player};
use bevy::prelude::{Query, Transform, Vec2, Vec3, With, Without};
use bevy::sprite::collide_aabb::collide;
use crate::system::tiler::TileCollider;

pub fn in_collision(
    player_position: Vec3,
    wall_query: &Query<&Transform, (With<TileCollider>, Without<Player>)>,
) -> bool {
    for wt in wall_query.iter() {
        if collide(
            player_position,
            Vec2::splat(TILE_SIZE * 0.9),
            wt.translation,
            Vec2::splat(TILE_SIZE),
        )
        .is_some()
        {
            return false;
        }
    }
    true
}
