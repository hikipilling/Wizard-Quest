use crate::components::*;
use bevy::prelude::*;

pub fn spawn_hud(mut commands: Commands) {
    let style = TextStyle {
        font_size: 30.0,
        color: Color::WHITE,
        ..default()
    };

    commands.spawn((
        TextBundle::from_sections(vec![
            TextSection::new("Health:\n", style.clone()),
            TextSection::new("Damage:\n", style.clone()),
            TextSection::new("Fire rate:\n", style.clone()),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
        HealthText,
    ));
}

pub fn update_hud(
    mut text_query: Query<&mut Text, With<HealthText>>,
    player_query: Query<(&Health, &ReloadTime), With<Player>>,
) {
    if let Ok((health, reload_time)) = player_query.get_single() {
        if let Ok(mut text) = text_query.get_single_mut() {
            text.sections[0].value = format!("Health: {}/{}\n", health.current, health.max);
            text.sections[1].value = format!("Damage: {}\n", "placeholder");
            text.sections[2].value = format!(
                "Fire rate: {}\n",
                1.0 / reload_time.0.duration().as_secs_f32()
            );
        }
    }
}
