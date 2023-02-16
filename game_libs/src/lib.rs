pub mod components;
pub mod resources;

use bevy::log;
use bevy::prelude::*;
use resources::{tile_map::TileMap, MapPosition, MapOptions, TileSize};

use crate::components::Coordinates;

pub struct GameOfLife;

#[derive(Resource, Default, Clone)]
#[allow(dead_code)]
pub struct WindowDescriptor {
    title: String,
    width: usize,
    height: usize,
}

impl Plugin for GameOfLife {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::create_map);
        log::info!("Loaded GameOfLife Map Plugin")
    }
}

impl GameOfLife {
#[allow(unused_mut)]
    pub fn create_map(
        mut commands: Commands,
        map_options: Option<Res<MapOptions>>,
        window_options: Option<Res<WindowDescriptor>>
    ) {
        let options = match map_options {
            None => MapOptions::default(), // If no options is set we use the default one
            Some(o) => o.clone(),
        };

        let window = match window_options {
            None => WindowDescriptor::default(), // If no options is set we use the default one
            Some(o) => o.clone(),
        };

        let mut tile_map = TileMap::empty(options.map_size.0, options.map_size.1);
        tile_map.set_initial_conditions(options.alive_count);
        #[cfg(feature = "debug")]
        log::info!("{}", tile_map.console_output());

        let tile_size = match options.tile_size {
            TileSize::Fixed(v) => v,
            TileSize::Adaptive { min, max } => Self::adaptive_tile_size(
                window,
                (min, max),
                (tile_map.width(), tile_map.height()),
            ),
        };

        let map_size = Vec2::new(
            tile_map.width() as f32 * tile_size,
            tile_map.height() as f32 * tile_size,
        );
        #[cfg(feature = "debug")]
        log::info!("Map size (px): {}", map_size);

        let map_position = match options.position {
            MapPosition::Centered { offset } => {
                Vec3::new(-map_size.x/2., -map_size.y/2., 0.) + offset
            }
            MapPosition::Custom(p) => p,
        };

        commands.spawn_empty()
            .insert(Name::new("Map"))
            .insert(Transform::from_translation(map_position))
            .insert(GlobalTransform::default())
            .insert(VisibilityBundle::default())
            .with_children(|parent| {
                // We spawn the board background sprite at the center of the board, since the sprite pivot is centered
                parent
                    .spawn(SpriteBundle {
                        sprite: Sprite {
                            color: Color::WHITE,
                            custom_size: Some(map_size),
                            ..Default::default()
                        },
                        transform: Transform::from_xyz(
                            map_size.x / 2.,
                            map_size.y / 2.,
                            0.
                        ),
                        visibility: Visibility {
                            is_visible: true
                        },
                        ..Default::default()
                    })
                    .insert(ComputedVisibility::default())
                    .insert(Name::new("Background"));

                // Tiles
                for (y, line) in tile_map.iter().enumerate() {
                    for (x, _tile) in line.iter().enumerate() {
                        parent.spawn(SpriteBundle {
                            sprite: Sprite {
                                color: match _tile.is_alive(){
                                    true => Color::GRAY,
                                    false => Color::DARK_GRAY
                                },
                                custom_size: Some(Vec2::splat(
                                    tile_size - options.tile_padding as f32,
                                )),
                                ..Default::default()
                            },
                            transform: Transform::from_xyz(
                                (x as f32 * tile_size) + tile_size / 2.,
                                (y as f32 * tile_size) + tile_size / 2.,
                                1.,
                            ),
                            ..Default::default()
                        })
                        .insert(Name::new(format!("Tile ({}, {})", x, y)))
                        // We add the `Coordinates` component to our tile entity
                        .insert(Coordinates {
                            x: x as u16,
                            y: y as u16,
                        });
                    }
                }
            });
    }

    fn adaptive_tile_size(
        window: WindowDescriptor,
        (min, max): (f32, f32),
        (width, height): (u16, u16)
    ) -> f32 {
        let max_width = window.width as f32 / width as f32;
        let max_height = window.height as f32 / height as f32;
        max_width.min(max_height).clamp(min, max)
    }


}