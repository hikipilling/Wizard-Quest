use crate::bundles::*;
use crate::components::*;
use crate::events::*;
use bevy::prelude::*;

pub mod enemy;
pub mod hud;
pub mod player;

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
    projectile_query: Query<(Entity, &Transform, &Collider, &Projectile)>,
    mut enemy_query: Query<
        (Entity, &mut Transform, &Collider),
        (With<Enemy>, Without<Projectile>, Without<Player>),
    >,
    player_query: Query<(Entity, &Transform, &Collider), With<Player>>,
    mut damage_events: EventWriter<DamageEvent>,
    time: Res<Time>,
) {
    for (projectile_entity, projectile_transform, projectile_collider, projectile) in
        projectile_query.iter()
    {
        let projectile_pos = projectile_transform.translation.truncate();

        if enemy_query.contains(projectile.shot_by) {
            let (player_entity, player_transform, player_collider) = player_query.single();
            let player_pos = player_transform.translation.truncate();

            let collision = (projectile_pos.x - player_pos.x).abs()
                < (projectile_collider.size.x + player_collider.size.x) / 2.0
                && (projectile_pos.y - player_pos.y).abs()
                    < (projectile_collider.size.y + player_collider.size.y) / 2.0;

            if collision {
                damage_events.send(DamageEvent {
                    target: player_entity,
                    amount: 1,
                });

                commands.entity(projectile_entity).despawn();
            }
        } else {
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

    let mut enemies: Vec<_> = enemy_query.iter_mut().collect();

    for i in 0..enemies.len() {
        if let Some((left, right)) = enemies.split_at_mut_checked(i + 1) {
            let enemy_a = left.last_mut();

            if let Some((_, enemy_a_transform, enemy_a_collider)) = enemy_a {
                for (_, enemy_b_transform, enemy_b_collider) in right {
                    let collision =
                        (enemy_a_transform.translation.x - enemy_b_transform.translation.x).abs()
                            < (enemy_a_collider.size.x + enemy_b_collider.size.x) / 2.0
                            && (enemy_a_transform.translation.y - enemy_b_transform.translation.y)
                                .abs()
                                < (enemy_a_collider.size.y + enemy_b_collider.size.y) / 2.0;

                    if collision {
                        let direction = (enemy_a_transform.translation.truncate()
                            - enemy_b_transform.translation.truncate())
                        .normalize();

                        let distance = enemy_a_transform
                            .translation
                            .truncate()
                            .distance(enemy_b_transform.translation.truncate());

                        enemy_a_transform.translation +=
                            direction.extend(0.0) * (distance * 1.5) * time.delta_seconds();
                        enemy_b_transform.translation +=
                            direction.extend(0.0) * -(distance * 1.5) * time.delta_seconds();
                    }
                }
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

pub fn shoot(
    commands: &mut Commands,
    pos: Vec2,
    direction: Vec2,
    shooter: Entity,
    timer: &mut Timer,
    time: &Res<Time>,
) {
    if timer.finished() {
        commands.spawn(ProjectileBundle {
            projectile: Projectile {
                direction,
                shot_by: shooter,
            },
            speed: Speed(400.0),
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(0.0, 1.0, 0.0),
                    custom_size: Some(Vec2::new(10.0, 10.0)),
                    ..default()
                },
                transform: Transform::from_xyz(pos.x, pos.y, 0.0),
                ..default()
            },
            collider: Collider {
                size: Vec2::new(10.0, 10.0),
            },
        });
    }

    timer.tick(time.delta());
}
