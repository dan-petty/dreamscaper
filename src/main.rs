//! Dreamscaper application entry point.

use bevy::prelude::*;
use dreamscaper::engine::EnginePlugin;
use dreamscaper::game::GamePlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Dreamscaper — Isometric RTS Engine".into(),
                        resolution: (1280.0, 720.0).into(),
                        resizable: true,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()), // Sharp pixel / texture rendering without interpolation
        )
        .add_plugins((EnginePlugin, GamePlugin))
        .insert_resource(ClearColor(Color::rgb(0.08, 0.09, 0.11)))
        .run();
}
