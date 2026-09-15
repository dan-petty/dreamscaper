//! RTS user interface, HUD overlays, selection box, and mini-map placeholders.

use crate::engine::ecs::components::Unit;
use crate::engine::resources::GameTimeOfDay;
use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui)
            .add_systems(Update, update_hud_text);
    }
}

#[derive(Component)]
struct HudText;

fn setup_ui(mut commands: Commands) {
    commands.spawn((
        Text::new("Dreamscaper RTS Engine\nTime: 0.0s | Active Units: 0"),
        TextFont {
            font_size: FontSize::Px(16.0),
            ..default()
        },
        TextColor(Color::srgb(0.9, 0.9, 0.9)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
        HudText,
    ));
}

fn update_hud_text(
    time_of_day: Res<GameTimeOfDay>,
    units_query: Query<&Unit>,
    mut text_query: Query<&mut Text, With<HudText>>,
) {
    let unit_count = units_query.iter().count();
    let is_day = if time_of_day.is_daytime() {
        "Day"
    } else {
        "Night"
    };

    for mut text in text_query.iter_mut() {
        text.0 = format!(
            "Dreamscaper RTS Engine\nTime: {:.1}s ({}) | Units: {}",
            time_of_day.current_time, is_day, unit_count
        );
    }
}
