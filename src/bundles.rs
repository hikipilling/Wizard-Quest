use crate::components::*;
use bevy::prelude::*;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub health: Health,
    pub speed: Speed,
    pub reload_time: ReloadTime,
    pub sprite_bundle: SpriteBundle,
    pub can_shoot: CanShoot,
    pub collider: Collider,
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
    pub sidestep_mode: SidestepMode,
    pub sidestep_timer: SidestepTimer,
    pub reload_timer: ReloadTime,
    pub can_shoot: CanShoot,
}
