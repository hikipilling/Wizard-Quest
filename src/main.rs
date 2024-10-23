mod bundles;
mod components;
mod events;
mod plugins;
mod systems;

use bevy::prelude::*;
use plugins::{CollisionPlugin, EnemyPlugin, HealthPlugin, PlayerPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(CollisionPlugin)
        .add_plugins(HealthPlugin)
        .run();
}
