use crate::bundles::*;
use crate::components::*;
use bevy::prelude::*;
use rand::prelude::*;

pub fn spawn_enemy(commands: &mut Commands, x: f32, y: f32) {
    commands
        .spawn(EnemyBundle {
            enemy: Enemy,
            health: Health {
                current: 10,
                max: 10,
            },
            speed: Speed(100.0),
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

pub fn move_enemies(
    mut enemy_query: Query<(&mut Transform, &Speed), With<Enemy>>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
    time: Res<Time>,
) {
    let player_transform = player_query.single();
    let player_position = player_transform.translation.truncate();

    for (mut enemy_transform, enemy_speed) in enemy_query.iter_mut() {
        let enemy_position = enemy_transform.translation.truncate();
        let direction = (player_position - enemy_position).normalize();

        enemy_transform.translation += direction.extend(0.0) * enemy_speed.0 * time.delta_seconds();
    }
}
