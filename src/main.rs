use bevy::{prelude::*, window::PrimaryWindow};
use player::PlayerPlugin;

mod components;
mod player;
mod systems;

// region:    --- Asset Constants

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
        .add_plugins(PlayerPlugin)
        .add_systems(Startup, setup_system)
        .run();
}

fn setup_system(
    mut commands: Commands,
    query: Query<&Window, With<PrimaryWindow>>,
) {
    commands.spawn(Camera2dBundle::default());

    // Set up WinSize resource
    // capture window size
	let Ok(primary) = query.get_single() else {
		return;
	};
	let (win_w, win_h) = (primary.width(), primary.height());
    let win_size = WinSize {width: win_w, height: win_h}; 
    commands.insert_resource(win_size);
}