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
