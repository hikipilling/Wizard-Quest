mod bundles;
mod components;
mod events;
mod plugins;
mod systems;

use bevy::prelude::*;
use plugins::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HudPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(CollisionPlugin)
        .add_plugins(HealthPlugin)
        .run();
}
