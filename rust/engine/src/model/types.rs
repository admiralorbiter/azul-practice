use serde::{Deserialize, Serialize};

/// The 5 tile colors in Azul
///
/// Each color appears 20 times in the game, for a total of 100 tiles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TileColor {
    Blue,
    Yellow,
    Red,
    Black,
    White,
}

/// Phase of the draft for scenario generation
///
/// Used to categorize scenarios by how much of the drafting phase has progressed.
/// This helps in generating appropriately challenging practice scenarios.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DraftPhase {
    Early,
    Mid,
    Late,
}
