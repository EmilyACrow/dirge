use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;
use player::PlayerPlugin;
use systems::*;

mod components;
mod player;
mod systems;

// region:    --- Asset Constants
const PLAYER_MOVE_SPEED: f32 = 150.;

// region:    --- Game Constants

// region:    --- Resources
#[derive(Resource)]
pub struct WinSize {
    pub width: f32,
    pub height: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(PreStartup, setup_system)
        .add_plugins(PlayerPlugin)
        .add_systems(Update, movable_system)
        .run();
}

fn setup_system(
    mut commands: Commands,
    query: Query<&Window, With<PrimaryWindow>>,
) {// Set up WinSize resource
    // capture window size
	let Ok(primary) = query.get_single() else {
		return;
	};
	let (win_w, win_h) = (primary.width(), primary.height());
    let win_size = WinSize {width: win_w, height: win_h}; 
    commands.insert_resource(win_size);

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(win_w / 2.0, win_h / 2.0, 10.0),
        ..default()
    });

    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(500.0, 50.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)));

    
}