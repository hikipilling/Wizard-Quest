use crate::components::*;
use bevy::prelude::*;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub speed: Speed,
    pub sprite_bundle: SpriteBundle,
}

#[derive(Bundle)]
pub struct ProjectileBundle {
    pub projectile: Projectile,
    pub speed: Speed,
    pub sprite_bundle: SpriteBundle,
    pub collider: Collider,
}

#[derive(Bundle)]
pub struct EnemyBundle {
    pub enemy: Enemy,
    pub health: Health,
    pub speed: Speed,
    pub sprite_bundle: SpriteBundle,
    pub collider: Collider,
}
