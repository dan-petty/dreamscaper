//! RTS user interface, HUD overlays, selection box, and terrain stats.

use crate::engine::ecs::components::{
    Explorer, GridPosition, LandscapeFeature, TerrainDevelopmentState,
};
use crate::engine::resources::{GameTimeOfDay, TerrainConfig};
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
        Text::new("Dreamscaper RTS Engine\nInitializing Genesis Wave..."),
        TextFont {
            font_size: FontSize::Px(15.0),
            ..default()
        },
        TextColor(Color::srgb(0.92, 0.94, 0.96)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            padding: UiRect::all(Val::Px(8.0)),
            ..default()
        },
        HudText,
    ));
}

fn update_hud_text(
    time_of_day: Res<GameTimeOfDay>,
    config: Res<TerrainConfig>,
    explorer_query: Query<&GridPosition, With<Explorer>>,
    tile_query: Query<&TerrainDevelopmentState>,
    features_query: Query<&LandscapeFeature>,
    mut text_query: Query<&mut Text, With<HudText>>,
) {
    let explorer_coords = explorer_query
        .iter()
        .next()
        .map(|p| format!("({}, {})", p.x, p.y))
        .unwrap_or_else(|| "N/A".to_string());

    let total_tiles = (config.width * config.height) as usize;
    let mut detailed_count = 0;
    let mut forming_count = 0;

    for state in tile_query.iter() {
        match state {
            TerrainDevelopmentState::Detailed => detailed_count += 1,
            TerrainDevelopmentState::Forming(_) => forming_count += 1,
            TerrainDevelopmentState::Latent => {}
        }
    }

    let progress_pct = if total_tiles > 0 {
        (detailed_count as f32 / total_tiles as f32) * 100.0
    } else {
        0.0
    };

    let feature_count = features_query.iter().count();
    let is_day = if time_of_day.is_daytime() {
        "Day"
    } else {
        "Night"
    };

    for mut text in text_query.iter_mut() {
        text.0 = format!(
            "Dreamscaper RTS — Genesis Explorer\nTheme: {:?} | Time: {:.1}s ({})\nExplorer: {} | Crystallized: {}/{} ({:.1}%)\nForming: {} | Landscape Features: {}",
            config.theme,
            time_of_day.current_time,
            is_day,
            explorer_coords,
            detailed_count,
            total_tiles,
            progress_pct,
            forming_count,
            feature_count
        );
    }
}
