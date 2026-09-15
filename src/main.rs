//! Dreamscaper application entry point.

use bevy::app::{AppExit, ScheduleRunnerPlugin};
use bevy::prelude::*;
use dreamscaper::engine::EnginePlugin;
use dreamscaper::game::GamePlugin;
use std::time::Duration;

/// Headless simulation tick tracker and limit controller.
#[derive(Resource)]
struct HeadlessTickController {
    current_tick: u64,
    max_ticks: Option<u64>,
}

/// Checks if an active X11 or Wayland display server is available on Unix.
#[cfg(target_os = "linux")]
fn has_display_server() -> bool {
    std::env::var_os("DISPLAY").is_some() || std::env::var_os("WAYLAND_DISPLAY").is_some()
}

#[cfg(not(target_os = "linux"))]
fn has_display_server() -> bool {
    true
}

/// Configuration options parsed from command-line arguments.
#[derive(Debug, Clone)]
pub struct LaunchConfig {
    pub headless: bool,
    pub max_ticks: Option<u64>,
}

/// Parses CLI arguments to determine headless execution mode and optional tick bounds.
pub fn parse_launch_config() -> LaunchConfig {
    let args: Vec<String> = std::env::args().collect();
    let mut headless_flag = false;
    let mut max_ticks = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--headless" | "-h" => {
                headless_flag = true;
            }
            "--ticks" => {
                if i + 1 < args.len() {
                    if let Ok(n) = args[i + 1].parse::<u64>() {
                        max_ticks = Some(n);
                    }
                    i += 1;
                }
            }
            _ => {}
        }
        i += 1;
    }

    // Automatically fall back to headless simulation if no display server is present
    let headless = headless_flag || !has_display_server();

    LaunchConfig {
        headless,
        max_ticks,
    }
}

fn main() {
    let config = parse_launch_config();

    if config.headless {
        println!("============================================================");
        println!("  Dreamscaper RTS Engine — Headless Simulation Mode");
        println!("  Display server not detected or --headless specified.");
        println!("  Executing ECS, AI compute, and map updates via ScheduleRunner.");
        if let Some(ticks) = config.max_ticks {
            println!("  Simulation bounded to {} ticks.", ticks);
        }
        println!("============================================================");

        App::new()
            .add_plugins(
                MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
                    1.0 / 60.0,
                ))),
            )
            .add_plugins((
                bevy::log::LogPlugin::default(),
                bevy::asset::AssetPlugin::default(),
                bevy::transform::TransformPlugin,
                bevy::hierarchy::HierarchyPlugin,
            ))
            .insert_resource(HeadlessTickController {
                current_tick: 0,
                max_ticks: config.max_ticks,
            })
            .add_systems(Update, headless_tick_system)
            .add_plugins((EnginePlugin, GamePlugin))
            .run();
    } else {
        println!("============================================================");
        println!("  Dreamscaper RTS Engine — Windowed Desktop Client");
        println!("  Display server detected. Initializing Bevy window & renderer.");
        println!("============================================================");

        App::new()
            .add_plugins(
                DefaultPlugins
                    .set(WindowPlugin {
                        primary_window: Some(Window {
                            title: "Dreamscaper — Isometric RTS Engine".into(),
                            resolution: (1280.0_f32, 720.0_f32).into(),
                            resizable: true,
                            ..default()
                        }),
                        ..default()
                    })
                    .set(ImagePlugin::default_nearest()),
            )
            .add_plugins((EnginePlugin, GamePlugin))
            .insert_resource(ClearColor(Color::rgb(0.08, 0.09, 0.11)))
            .run();
    }
}

/// System tracking headless simulation ticks, logging heartbeat, and terminating if bounded.
fn headless_tick_system(
    mut controller: ResMut<HeadlessTickController>,
    time: Res<Time>,
    mut exit: EventWriter<AppExit>,
) {
    controller.current_tick += 1;

    if controller.current_tick % 60 == 0 {
        info!(
            "Tick: {} | In-Game Elapsed: {:.1}s | Delta: {:.4}s",
            controller.current_tick,
            time.elapsed_seconds(),
            time.delta_seconds()
        );
    }

    if let Some(max) = controller.max_ticks {
        if controller.current_tick >= max {
            println!(
                "Headless simulation reached target of {} ticks. Exiting cleanly.",
                max
            );
            exit.send(AppExit);
        }
    }
}
