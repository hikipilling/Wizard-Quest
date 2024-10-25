use crate::components::*;
use crate::events::*;
use bevy::prelude::*;

pub mod enemy;
pub mod hud;
pub mod player;

pub use enemy::*;
pub use hud::*;
pub use player::*;

pub fn move_projectiles(
    mut projectile_query: Query<(&Projectile, &Speed, &mut Transform)>,
    time: Res<Time>,
) {
    for (projectile, speed, mut transform) in projectile_query.iter_mut() {
        transform.translation += projectile.direction.extend(0.0) * speed.0 * time.delta_seconds();
    }
}

pub fn detect_collisions(
    mut commands: Commands,
    projectile_query: Query<(Entity, &Transform, &Collider), With<Projectile>>,
    mut enemy_query: Query<(Entity, &Transform, &Collider), With<Enemy>>,
    mut damage_events: EventWriter<DamageEvent>,
) {
    for (projectile_entity, projectile_transform, projectile_collider) in projectile_query.iter() {
        let projectile_pos = projectile_transform.translation.truncate();

        for (enemy_entity, enemy_transform, enemy_collider) in enemy_query.iter_mut() {
            let enemy_pos = enemy_transform.translation.truncate();

            let collision = (projectile_pos.x - enemy_pos.x).abs()
                < (projectile_collider.size.x + enemy_collider.size.x) / 2.0
                && (projectile_pos.y - enemy_pos.y).abs()
                    < (projectile_collider.size.y + enemy_collider.size.y) / 2.0;

            if collision {
                damage_events.send(DamageEvent {
                    target: enemy_entity,
                    amount: 1,
                });

                commands.entity(projectile_entity).despawn();
            }
        }
    }
}

pub fn handle_damage(
    mut commands: Commands,
    mut events: EventReader<DamageEvent>,
    mut health_query: Query<&mut Health>,
) {
    for event in events.read() {
        if let Ok(mut health) = health_query.get_mut(event.target) {
            health.current -= event.amount;

            if health.current <= 0 {
                commands.entity(event.target).despawn_recursive();
            }
        }
    }
}
