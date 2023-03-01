use bevy::{prelude::*, DefaultPlugins};

use shaderlibs::{setup, GameOfLifeComputePlugin};

#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    let mut app = App::new();
    app.insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                // uncomment for unthrottled FPS
                // present_mode: bevy::window::PresentMode::AutoNoVsync,
                title: "Game of Life".to_string(),
                width: 1920.0,
                height: 1080.0,
                ..Default::default()
            },
            ..default()
        }));
    // Window setup
    app.add_system(bevy::window::close_on_esc)
        .add_plugin(GameOfLifeComputePlugin)
        .add_startup_system(setup);
    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin);
    app.run();
}
