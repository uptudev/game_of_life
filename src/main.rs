use bevy::{prelude::*, DefaultPlugins};
#[allow(unused_imports)]
use game_libs::{GameOfLife, resources::MapOptions};

#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

#[derive(Resource, Default, Clone)]
#[allow(dead_code)]
pub struct WindowDescriptor {
    title: String,
    width: usize,
    height: usize,
}

fn main() {
    let mut app = App::new();
    // Window setup
    app.insert_resource(WindowDescriptor {
        title: "Mine Sweeper!".to_string(),
        width: 700,
        height: 800,
        ..Default::default()
    }).insert_resource(MapOptions {
            map_size: (100, 100),
            alive_count: 5000,
            tile_padding: 3.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_system(bevy::window::close_on_esc)
        .add_plugin(GameOfLife);
    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin);
    app.add_startup_system(camera_setup);
    app.run();
}

fn camera_setup(mut commands: Commands) {
    // 2D orthographic camera
    commands.spawn(Camera2dBundle::default());
}