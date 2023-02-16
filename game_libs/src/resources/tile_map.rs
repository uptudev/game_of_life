use crate::{resources::tile::Tile, components::Coordinates};
use std::ops::{Deref, DerefMut};
use bevy::prelude::Component;
use rand::{thread_rng, Rng};

/// Base tile map
#[derive(Component, Debug, Clone)]
pub struct TileMap {
    height: u16,
    width: u16,
    map: Vec<Vec<Tile>>,
}

#[allow(dead_code)]
impl TileMap {
    /// Generates an empty map
    pub fn empty(width: u16, height: u16) -> Self {
        let map = (0..height)
            .into_iter()
            .map(|_| (0..width).into_iter().map(|_| Tile::Dead).collect())
            .collect();
        Self {
            height,
            width,
            map,
        }
    }

    #[cfg(feature = "debug")]
    pub fn console_output(&self) -> String {
        let mut buffer = format!(
            "Map ({}, {}):\n",
            self.width, self.height
        );
        let line: String = (0..=(self.width + 1)).into_iter().map(|_| '-').collect();
        buffer = format!("{}{}\n", buffer, line);
        for line in self.iter().rev() {
            buffer = format!("{}|", buffer);
            for tile in line.iter() {
                buffer = format!("{}{}", buffer, tile.console_output());
            }
            buffer = format!("{}|\n", buffer);
        }
        format!("{}{}", buffer, line)
    }

    // Getter for `width`
    pub fn width(&self) -> u16 {
        self.width
    }

    // Getter for `height`
    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn safe_square_at(&self, coordinates: Coordinates) -> impl Iterator<Item = Coordinates> {
        ORTHO_COORDINATES
            .iter()
            .copied()
            .map(move |tuple| coordinates + tuple)
    }

    pub fn is_alive_at(&self, coordinates: Coordinates) -> bool {
        if coordinates.x >= self.width || coordinates.y >= self.height {
            return false;
        };
        self.map[coordinates.y as usize][coordinates.x as usize].is_alive()
    }

    pub fn alive_count_at(&self, coordinates: Coordinates) -> u8 {
        if self.is_alive_at(coordinates) {
            return 0;
        }
        let res = self
             .safe_square_at(coordinates)
             .filter(|coord| self.is_alive_at(*coord))
             .count();
        res as u8
    }

    /// Randomly places living cells
    pub fn set_initial_conditions(&mut self, alive_init: u16) {
        let mut remaining_alive = alive_init;
        let mut rng = thread_rng();

        //Place alive cells
        while remaining_alive > 0 {
            let (x, y) = (
                rng.gen_range(0..self.width) as usize,
                rng.gen_range(0..self.height) as usize,
            );

            if let Tile::Dead = self.map[y][x] {
                self.map[y][x] = Tile::Alive;
                remaining_alive -=1;
            }
        }
    }
}

impl Deref for TileMap {
    type Target = Vec<Vec<Tile>>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl DerefMut for TileMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}

/// Delta coordinates for all 8 square neighbors
const ORTHO_COORDINATES: [(i8, i8); 8] = [
    // Bottom left
    (-1, -1),
    // Bottom
    (0, -1),
    // Bottom right
    (1, -1),
    // Left
    (-1, 0),
    // Right
    (1, 0),
    // Top Left
    (-1, 1),
    // Top
    (0, 1),
    // Top right
    (1, 1),
];