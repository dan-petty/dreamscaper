//! Game logic, map generation, RTS units, and UI overlays.

pub mod map;
pub mod ui;
pub mod unit;

use bevy::prelude::*;

/// Game plugin aggregating map, unit, and UI subsystems.
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((map::MapPlugin, unit::UnitPlugin, ui::UiPlugin));
    }
}
