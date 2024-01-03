use bevy::ecs::component::Component;
use bevy::prelude::Vec2;

// Common Components
#[derive(Component)]
pub struct Health {
    pub max: f32,
    pub current: f32,
}

#[derive(Component)]
pub struct InputDirection {
    pub direction: Vec2
}

#[derive(Component)]
pub struct Movable {
    pub speed: f32,
    pub auto_despawn: bool,
}

// Player Components
#[derive(Component)]
pub struct Player;
