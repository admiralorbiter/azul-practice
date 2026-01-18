use serde::{Deserialize, Serialize};
use super::TileColor;

/// Wall is a 5x5 grid of filled/empty positions
///
/// The wall uses a fixed color pattern (standard Azul layout).
/// Each position can be either filled (true) or empty (false).
pub type Wall = [[bool; 5]; 5];

/// A single pattern line row
///
/// Pattern lines have fixed capacities (1, 2, 3, 4, 5 for rows 0-4).
/// Once a pattern line is complete, it places one tile on the wall during
/// the end-of-round scoring phase.
///
/// # Invariants
///
/// - `count_filled` ≤ `capacity`
/// - If `count_filled > 0`, then `color` must be `Some(_)`
/// - If `count_filled == 0`, then `color` must be `None`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PatternLine {
    pub capacity: u8,
    pub color: Option<TileColor>,
    pub count_filled: u8,
}

impl PatternLine {
    /// Create an empty pattern line for a given row
    ///
    /// Row indices are 0-4, corresponding to capacities 1-5.
    pub fn new(row_index: usize) -> Self {
        Self {
            capacity: (row_index + 1) as u8,
            color: None,
            count_filled: 0,
        }
    }
}

/// Floor line holds tiles that incur penalties
///
/// The floor line has 7 penalty slots, but can hold more than 7 tiles.
/// All tiles are tracked for discard, but only the first 7 incur penalties.
///
/// Penalty values: [-1, -1, -2, -2, -2, -3, -3] for slots 0-6.
/// The first-player token occupies slot 0 if present.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FloorLine {
    pub tiles: Vec<TileColor>,
    pub has_first_player_token: bool,
}

/// A player's board state
///
/// Contains all components of a player's board: score, pattern lines,
/// wall, and floor line.
///
/// # Example
///
/// ```
/// use engine::PlayerBoard;
/// let board = PlayerBoard::new();
/// assert_eq!(board.score, 0);
/// assert_eq!(board.pattern_lines.len(), 5);
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PlayerBoard {
    pub score: i32,
    pub pattern_lines: [PatternLine; 5],
    pub wall: Wall,
    pub floor_line: FloorLine,
}

impl PlayerBoard {
    /// Create a new player board with empty state
    ///
    /// Initializes all pattern lines with appropriate capacities,
    /// an empty wall, and an empty floor line.
    pub fn new() -> Self {
        Self {
            score: 0,
            pattern_lines: [
                PatternLine::new(0),
                PatternLine::new(1),
                PatternLine::new(2),
                PatternLine::new(3),
                PatternLine::new(4),
            ],
            wall: [[false; 5]; 5],
            floor_line: FloorLine {
                tiles: Vec::new(),
                has_first_player_token: false,
            },
        }
    }
}

impl Default for PlayerBoard {
    fn default() -> Self {
        Self::new()
    }
}
