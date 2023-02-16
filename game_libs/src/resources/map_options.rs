/* //TODO 
 * Add all tile map options (width, height, and initial alive count)
 * Add custom padding between tile sprites
 * Add custom tile size / adaptive window size
 * Add custom board world position or window-centred with optional offset
 */

use bevy::prelude::{Vec3, Resource};
use serde::{Deserialize, Serialize};

///Tile size options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TileSize {
    ///Fixed tile size
    Fixed(f32),
    ///Adaptive tile size
    Adaptive {min: f32, max: f32}
}

///Board position customization options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MapPosition {
    /// Centered board
    Centered { offset: Vec3 },
    /// Custom position
    Custom(Vec3),
}

/// Board generation options. Must be used as a resource
#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct MapOptions {
    /// Tile map size
    pub map_size: (u16, u16),
    /// bomb count
    pub alive_count: u16,
    /// Board world position
    pub position: MapPosition,
    /// Tile world size
    pub tile_size: TileSize,
    /// Padding between tiles
    pub tile_padding: f32,
}

impl Default for TileSize {
    fn default() -> Self {
        Self::Adaptive {
            min: (10.0),
            max: (50.0) 
        }
    }
}

impl Default for MapPosition {
    fn default() -> Self {
        Self::Centered { offset: Default::default() }
    }
}

impl Default for MapOptions {
    fn default() -> Self {
        Self {
            map_size: (25, 25),
            alive_count: 30,
            position: Default::default(),
            tile_size: Default::default(),
            tile_padding: 0.
        }
    }
}