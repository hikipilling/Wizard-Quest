use crate::events::DamageEvent;
use crate::systems;
use crate::systems::enemy;
use crate::systems::hud;
use crate::systems::player;
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, player::spawn_player).add_systems(
            Update,
            (
                player::move_player,
                player::handle_shooting,
                systems::move_projectiles,
            ),
        );
    }
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, enemy::spawn_enemies);
        app.add_systems(Update, enemy::move_enemies);
        app.add_systems(Update, enemy::handle_shooting);
    }
}

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, systems::detect_collisions);
    }
}

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DamageEvent>();
        app.add_systems(Update, systems::handle_damage);
        app.add_systems(Update, enemy::update_health_bars);
    }
}

pub struct HudPlugin;
impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, hud::spawn_hud);
        app.add_systems(Update, hud::update_hud);
    }
}
