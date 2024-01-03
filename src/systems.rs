use bevy::prelude::*;
use bevy_rapier2d::control::KinematicCharacterController;

use crate::{components::{Movable, InputDirection}, WinSize};

pub fn movable_system(
    mut commands: Commands,
    time: Res<Time>,
    win_size: Res<WinSize>,
    mut player_query: Query<(&mut KinematicCharacterController, &InputDirection, &mut Transform, &Movable)>,
    // mut query: Query<(Entity, &InputDirection, &mut Transform, &Movable)>,
) {
    let delta = time.delta_seconds();

    for (mut controller, input, mut transform, movable) in &mut player_query {
        controller.translation = Some(input.direction);
        //translation.y += direction.y * delta * movable.speed;

        // if movable.auto_despawn && (translation.x < 0.0 || translation.x > win_size.width
        //         || translation.y < 0.0 || translation.y > win_size.height) {
        //     commands.entity(entity).despawn();
        // }
    }
}