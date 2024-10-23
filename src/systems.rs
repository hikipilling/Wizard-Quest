use crate::bundles::*;
use crate::components::*;
use crate::events::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::Rng;

pub fn spawn_player(mut commands: Commands) {
    commands.spawn(PlayerBundle {
        player: Player,
        speed: Speed(200.0),
        sprite_bundle: SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(1.0, 0.0, 0.0),
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
    });

    commands.spawn(Camera2dBundle::default());
}

pub fn spawn_enemy(commands: &mut Commands, x: f32, y: f32) {
    commands
        .spawn(EnemyBundle {
            enemy: Enemy,
            health: Health {
                current: 10,
                max: 10,
            },
            speed: Speed(200.0),
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(0.0, 0.0, 1.0),
                    custom_size: Some(Vec2::new(50.0, 50.0)),
                    ..default()
                },
                transform: Transform::from_xyz(x, y, 0.0),
                ..default()
            },
            collider: Collider {
                size: Vec2::new(50.0, 50.0),
            },
        })
        .with_children(|parent| {
            parent
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgb(1.0, 0.0, 0.0),
                        custom_size: Some(Vec2::new(60.0, 5.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, 40.0, 999.0),
                    ..default()
                })
                .insert(HealthBarBackground);

            parent
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgb(0.0, 1.0, 0.0),
                        custom_size: Some(Vec2::new(60.0, 5.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, 40.0, 999.1),
                    ..default()
                })
                .insert(HealthBar);
        });
}

pub fn spawn_enemies(mut commands: Commands) {
    for _ in 0..5 {
        spawn_enemy(
            &mut commands,
            rand::thread_rng().gen_range(-400..400) as f32,
            rand::thread_rng().gen_range(-200..200) as f32,
        );
    }
}

pub fn move_player(
    mut query: Query<(&Speed, &mut Transform), With<Player>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let (speed, mut transform) = query.single_mut();

    let mut direction = Vec3::ZERO;

    if keyboard.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    if direction != Vec3::ZERO {
        direction = direction.normalize();
    }

    transform.translation += direction * speed.0 * time.delta_seconds();
}

pub fn handle_shooting(
    mut commands: Commands,
    buttons: Res<ButtonInput<MouseButton>>,
    player_query: Query<&Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }

    let (camera, camera_transform) = camera_query.single();
    let window = window_query.single();
    let player_transform = player_query.single();

    if let Some(cursor_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        let player_position = player_transform.translation.truncate();
        let direction = (cursor_position - player_position).normalize();

        commands.spawn(ProjectileBundle {
            projectile: Projectile { direction },
            speed: Speed(400.0),
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(0.0, 1.0, 0.0),
                    custom_size: Some(Vec2::new(10.0, 10.0)),
                    ..default()
                },
                transform: Transform::from_xyz(player_position.x, player_position.y, 0.0),
                ..default()
            },
            collider: Collider {
                size: Vec2::new(10.0, 10.0),
            },
        });
    }
}

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

pub fn update_health_bars(
    mut health_bar_query: Query<(&mut Sprite, &mut Transform, &Parent), With<HealthBar>>,
    enemy_query: Query<&Health, With<Enemy>>,
) {
    for (mut sprite, mut transform, parent) in health_bar_query.iter_mut() {
        if let Ok(health) = enemy_query.get(parent.get()) {
            let health_percentage = health.current as f32 / health.max as f32;

            sprite.custom_size = Some(Vec2::new(60.0 * health_percentage, 5.0));

            transform.translation.x = -60.0 * (1.0 - health_percentage) / 2.0;
        }
    }
}
