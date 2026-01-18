/// Number of factories in a 2-player game
pub const FACTORY_COUNT_2P: usize = 5;

/// Number of tiles placed on each factory at the start of a round
pub const TILES_PER_FACTORY: usize = 4;

/// Number of tiles of each color in the game
pub const TILES_PER_COLOR: u8 = 20;

/// Total number of tiles in the game (5 colors × 20 tiles)
pub const TOTAL_TILES: u8 = 100;

/// Number of penalty slots on the floor line
pub const FLOOR_LINE_SLOTS: usize = 7;

/// Number of pattern lines per player
pub const PATTERN_LINE_COUNT: usize = 5;

/// Floor line penalty values for each slot
///
/// Penalties apply to the first 7 tiles on the floor line.
/// The first-player token occupies slot 0 when present.
///
/// # Example Scoring
///
/// - 3 tiles + no token: -1 + -1 + -2 = -4 points
/// - 5 tiles + token: -1 + -1 + -2 + -2 + -2 + -3 = -11 points
///
/// Score cannot go below 0.
pub const FLOOR_PENALTIES: [i32; 7] = [-1, -1, -2, -2, -2, -3, -3];
