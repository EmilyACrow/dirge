#![allow(unused)] // TODO: remove this

use crate::{components::{Player, Velocity}, PLAYER_MOVE_SPEED, WinSize};

use bevy::{prelude::*, window::PrimaryWindow};
use crate::components::Movable;

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
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width / 2.0, window.height / 2.0, 0.0),
            texture: asset_server.load("sprites/wizard.png"),
            ..Default::default()
        },
        Player {},
        Velocity { x: 0., y: 0.},
        Movable { speed: PLAYER_MOVE_SPEED, auto_despawn: false },
    ));
}

fn player_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Velocity, With<Player>)>,
    time: Res<Time>,
) {
    if let Ok(mut velocity) = query.get_single_mut() {
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
        velocity.0.x = input.x;
        velocity.0.y = input.y;
    }
}
