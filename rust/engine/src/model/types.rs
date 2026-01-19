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

/// Phase of the draft within a single round (within-round progress)
///
/// Tracks how much of the current round's drafting has progressed based on
/// tiles remaining on the table (factories + center).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RoundStage {
    /// Start of round: many tiles available (14-20 tiles)
    Start,
    /// Mid-round: moderate tiles remaining (7-13 tiles)
    Mid,
    /// End of round: few tiles left (0-6 tiles)
    End,
}

/// Across-game progress stage (game-level progress)
///
/// Tracks overall game progression based on wall development and board state.
/// Used for scenario generation to ensure appropriate challenge levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GameStage {
    /// Early game: ≤8 wall tiles placed per player
    Early,
    /// Mid game: 9-17 wall tiles placed per player
    Mid,
    /// Late game: ≥18 wall tiles placed or near row completion
    Late,
}

/// Legacy alias for backward compatibility
/// This will be deprecated in favor of separate RoundStage and GameStage
pub type DraftPhase = RoundStage;
