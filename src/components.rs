use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Projectile {
    pub direction: Vec2,
}

#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

#[derive(Component)]
pub struct Collider {
    pub size: Vec2,
}

#[derive(Component)]
pub struct HealthBar;

#[derive(Component)]
pub struct HealthBarBackground;
