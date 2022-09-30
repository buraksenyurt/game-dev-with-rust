use crate::RESOLUTION;
use bevy::prelude::{Camera2dBundle, Commands};
use bevy::render::camera::*;

pub fn spawn_camera(mut commands: Commands) {
    // Bu örnekte diz üzdüşüm temelli bir kamera kullanılmakta
    // Orthographic Projection, dik izdüşüm anlamındadır.

    let mut cmr = Camera2dBundle::default();
    cmr.projection = OrthographicProjection::default();
    cmr.projection.top = 1.;
    cmr.projection.bottom = -1.;
    cmr.projection.right = 1. * RESOLUTION;
    cmr.projection.left = -1. * RESOLUTION;
    cmr.projection.scaling_mode = ScalingMode::None;
    commands.spawn_bundle(cmr);
}
