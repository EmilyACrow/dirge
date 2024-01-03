#![allow(unused)] // TODO: remove this
use bevy_rapier2d::prelude::*;

use crate::{components::{Player}, PLAYER_MOVE_SPEED, WinSize};

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::geometry::Collider;
use crate::components::{Health, InputDirection, Movable};

#[derive(Bundle)]
struct PlayerBundle {
    health: Health,
    player: Player,
    collider: Collider,
    sprite: SpriteBundle,
    rigid_body: RigidBody,
    controller: KinematicCharacterController,
    movable: Movable,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, player_movement_system);
    }
}

fn setup(mut commands: Commands, 
    window: Res<WinSize>,
    asset_server: Res<AssetServer>,
) {
    let player_bundle = PlayerBundle {
        health: Health {current: 100., max: 100. },
        sprite: SpriteBundle {
            transform: Transform::from_xyz(window.width / 2.0, window.height / 2.0, 0.0),
            texture: asset_server.load("sprites/wizard.png"),
            ..Default::default()
        },
        rigid_body: RigidBody::KinematicPositionBased,
        player: Player{},
        collider: Collider::cuboid(1.0, 1.0),
        controller: KinematicCharacterController::default(),
        movable: Movable { speed: PLAYER_MOVE_SPEED, auto_despawn: false },
    };

    commands
        .spawn(player_bundle)
        .insert(ActiveEvents::COLLISION_EVENTS);

}

fn player_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut InputDirection, With<Player>)>,
    time: Res<Time>,
) {
    if let Ok(mut direction) = query.get_single_mut() {
        let mut input = Vec2::ZERO;
        if keyboard_input.pressed(KeyCode::W) {
            input += Vec2::Y;
        }
        if keyboard_input.pressed(KeyCode::A) {
            input -= Vec2::X;
        }
        if keyboard_input.pressed(KeyCode::S) {
            input -= Vec2::Y;
        }
        if keyboard_input.pressed(KeyCode::D) {
            input += Vec2::X;
        }
        input = input.normalize_or_zero();
        direction.0.direction = input;
    }
}
