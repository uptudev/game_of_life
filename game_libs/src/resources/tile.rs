// tile.rs
#[cfg(feature = "debug")]
use colored::Colorize;

/// Enum describing a single tile
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum State{
    /// Is alive
    Alive,
    /// Dead
    Dead,
}

impl Tile {
    /// Is the tile alive?
    pub const fn is_alive(&self) -> bool {
        matches!(self, Self::Alive)
    }

    #[cfg(feature = "debug")]
    pub fn console_output(&self) -> String {
        format!(
            "{}",
            match self {
                Tile::Alive => "*".bright_red(),
                Tile::Dead => " ".normal(),
            }
        )
    }
}
