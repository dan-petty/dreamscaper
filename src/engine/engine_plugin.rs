//! Root engine plugin aggregating renderer, AI, and resource subsystems.

use bevy::prelude::*;
use crate::engine::ai::AiPlugin;
use crate::engine::renderer::RendererPlugin;
use crate::engine::resources::ResourcePlugin;

/// Root engine plugin that bootstraps core systems.
pub struct EnginePlugin;

impl Plugin for EnginePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ResourcePlugin,
            RendererPlugin,
            AiPlugin,
        ));
    }
}
