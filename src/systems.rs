use bevy::prelude::*;

use crate::{components::{Velocity, Movable}, WinSize};

pub fn movable_system(
    mut commands: Commands,
    time: Res<Time>,
    win_size: Res<WinSize>,
    mut query: Query<(Entity, &Velocity, &mut Transform, &Movable)>,
) {
    let delta = time.delta_seconds();

    for (entity, velocity, mut transform, movable) in &mut query {
        let translation = &mut transform.translation;
        translation.x += velocity.x * delta * movable.speed;
        translation.y += velocity.y * delta * movable.speed;
        if (velocity.x!=0. || velocity.y!= 0.) { println!("velocity x: {:?}, velocity y: {:?}", velocity.x, velocity.y); }

        if movable.auto_despawn && (translation.x < 0.0 || translation.x > win_size.width
                || translation.y < 0.0 || translation.y > win_size.height) {
            commands.entity(entity).despawn();
        }
    }
}