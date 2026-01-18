use serde::{Deserialize, Serialize};
use super::TileColor;

/// Source of tiles for a draft action
///
/// A player can take tiles from either a specific factory or the center area.
///
/// # JSON Serialization
///
/// - `Factory(n)` serializes to `{"Factory": n}`
/// - `Center` serializes to `"Center"`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ActionSource {
    /// Take from a specific factory (index 0-4 for 2-player)
    Factory(usize),
    /// Take from the center area
    Center,
}

/// Destination for tiles in a draft action
///
/// Tiles can be placed in a pattern line or directly on the floor.
///
/// # JSON Serialization
///
/// - `PatternLine(n)` serializes to `{"PatternLine": n}`
/// - `Floor` serializes to `"Floor"`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum Destination {
    /// Place tiles in a pattern line (row index 0-4)
    PatternLine(usize),
    /// Place tiles directly on the floor line
    Floor,
}

/// A single draft action in Azul
///
/// Represents a complete player action during the draft phase:
/// 1. Choose a source (factory or center)
/// 2. Choose a color (take ALL tiles of that color)
/// 3. Choose a destination (pattern line or floor)
///
/// # Side Effects
///
/// When applied, a draft action may have side effects:
/// - If taking from a factory: remaining tiles move to center
/// - If taking from center with first-player token: token moves to player's floor
/// - Tiles that don't fit in pattern line overflow to floor
///
/// # Example
///
/// ```
/// use engine::{DraftAction, ActionSource, Destination, TileColor};
///
/// let action = DraftAction {
///     source: ActionSource::Factory(0),
///     color: TileColor::Blue,
///     destination: Destination::PatternLine(2),
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DraftAction {
    pub source: ActionSource,
    pub color: TileColor,
    pub destination: Destination,
}
