use crate::bundles::*;
use crate::components::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use std::time::Duration;

pub fn spawn_player(mut commands: Commands) {
    commands.spawn(PlayerBundle {
        player: Player,
        speed: Speed(200.0),
        health: Health {
            current: 10,
            max: 10,
        },
        reload_time: ReloadTime(Timer::new(
            Duration::from_secs_f32(0.5),
            TimerMode::Repeating,
        )),
        sprite_bundle: SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(1.0, 0.0, 0.0),
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        can_shoot: CanShoot,
        collider: Collider {
            size: Vec2::new(50.0, 50.0),
        },
    });

    commands.spawn(Camera2dBundle::default());
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
    mut player_query: Query<(&Transform, &mut ReloadTime, Entity), With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    time: Res<Time>,
) {
    let (player_transform, mut reload_time, player_entity) = player_query.single_mut();

    if reload_time.0.finished() {
        if !buttons.pressed(MouseButton::Left) {
            return;
        }

        let (camera, camera_transform) = camera_query.single();
        let window = window_query.single();

        if let Some(cursor_position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            let player_position = player_transform.translation.truncate();
            let direction = (cursor_position - player_position).normalize();

            super::shoot(
                &mut commands,
                player_position,
                direction,
                player_entity,
                &mut reload_time.0,
                &time,
            );
        }
    } else {
        reload_time.0.tick(time.delta());
    }
}
