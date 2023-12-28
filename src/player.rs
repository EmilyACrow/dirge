#![allow(unused)] // TODO: remove this

use crate::components::{Player, Velocity};

use bevy::{prelude::*, window::PrimaryWindow};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, player_movement_system);
    }
}

fn player_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Velocity, With<Player>)>,
    time: Res<Time>,
) {
    if let Ok(mut velocity) = query.get_single_mut() {
        let mut velocity = Vec2::ZERO;
        if keyboard_input.pressed(KeyCode::W) {
            velocity += Vec2::Y;
        }
        if keyboard_input.pressed(KeyCode::A) {
            velocity -= Vec2::X;
        }
        if keyboard_input.pressed(KeyCode::S) {
            velocity -= Vec2::Y;
        }
        if keyboard_input.pressed(KeyCode::D) {
            velocity += Vec2::X;
        }
        velocity = velocity.normalize_or_zero();
    }
}

fn setup(mut commands: Commands, 
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/wizard.png"),
            ..Default::default()
        },
        Player {},
    ));
}
