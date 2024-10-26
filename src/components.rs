use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Projectile {
    pub direction: Vec2,
    pub shot_by: Entity,
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

#[derive(Component)]
pub struct HealthText;

#[derive(Component)]
pub struct ReloadTime(pub Timer);

#[derive(Component, PartialEq)]
pub enum SidestepMode {
    Stationary,
    Left,
    Right,
}

#[derive(Component)]
pub struct SidestepTimer(pub Timer);

#[derive(Component)]
pub struct CanShoot;
