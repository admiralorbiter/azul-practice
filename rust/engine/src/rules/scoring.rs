use crate::model::{State, Wall, FloorLine};

/// Calculate score for placing a tile on the wall.
///
/// Counts horizontal and vertical contiguous tiles (including the newly placed tile).
/// 
/// # Scoring Rules
/// 
/// - **Isolated tile** (no adjacent tiles): 1 point
/// - **Horizontal chain only**: Number of tiles in horizontal chain
/// - **Vertical chain only**: Number of tiles in vertical chain  
/// - **Both directions**: Sum of horizontal and vertical chain lengths
///
/// # Arguments
///
/// * `wall` - The 5×5 wall grid (after tile placement)
/// * `row` - Row index of newly placed tile (0-4)
/// * `col` - Column index of newly placed tile (0-4)
///
/// # Returns
///
/// Points earned (1-10, where 10 is maximum for completing full row + column)
///
/// # Panics
///
/// Panics in debug mode if `wall[row][col]` is false (tile not placed)
///
/// # Examples
///
/// ```
/// use engine::{calculate_wall_tile_score, Wall};
///
/// // Isolated tile scores 1 point
/// let mut wall: Wall = [[false; 5]; 5];
/// wall[2][2] = true;
/// assert_eq!(calculate_wall_tile_score(&wall, 2, 2), 1);
///
/// // Horizontal chain of 3 scores 3 points
/// wall[1][0] = true;
/// wall[1][1] = true;
/// wall[1][2] = true;
/// assert_eq!(calculate_wall_tile_score(&wall, 1, 1), 3);
/// ```
pub fn calculate_wall_tile_score(wall: &Wall, row: usize, col: usize) -> i32 {
    debug_assert!(
        wall[row][col],
        "Tile must be placed at position [{}, {}]",
        row, col
    );
    
    // Count horizontal contiguous tiles
    let mut h_count = 1; // Start with the placed tile
    
    // Count left
    for c in (0..col).rev() {
        if wall[row][c] {
            h_count += 1;
        } else {
            break;
        }
    }
    
    // Count right
    #[allow(clippy::needless_range_loop)]
    for c in (col + 1)..5 {
        if wall[row][c] {
            h_count += 1;
        } else {
            break;
        }
    }
    
    // Count vertical contiguous tiles
    let mut v_count = 1; // Start with the placed tile
    
    // Count up
    for r in (0..row).rev() {
        if wall[r][col] {
            v_count += 1;
        } else {
            break;
        }
    }
    
    // Count down
    #[allow(clippy::needless_range_loop)]
    for r in (row + 1)..5 {
        if wall[r][col] {
            v_count += 1;
        } else {
            break;
        }
    }
    
    // Calculate final score
    if h_count == 1 && v_count == 1 {
        // Isolated tile: no adjacent tiles in either direction
        1
    } else {
        let mut score = 0;
        if h_count > 1 {
            score += h_count;
        }
        if v_count > 1 {
            score += v_count;
        }
        score
    }
}

/// Calculate floor penalty for a player's floor line.
///
/// Penalties apply to the first 7 "slots" on the floor line:
/// - Slot 0: -1 (occupied by first-player token if present)
/// - Slots 1-6: -1, -2, -2, -2, -3, -3
///
/// If the player has the first-player token, it occupies slot 0 and tiles
/// start at slot 1. Otherwise, tiles start at slot 0.
///
/// Tiles beyond the 7th slot incur no additional penalty.
///
/// # Arguments
///
/// * `floor_line` - Player's floor line with tiles and token status
///
/// # Returns
///
/// Penalty value (always ≤ 0, range: -14 to 0)
///
/// # Examples
///
/// ```
/// use engine::{calculate_floor_penalty, FloorLine, TileColor};
///
/// // 3 tiles + token = -6 points
/// let floor = FloorLine {
///     tiles: vec![TileColor::Blue, TileColor::Red, TileColor::Yellow],
///     has_first_player_token: true,
/// };
/// assert_eq!(calculate_floor_penalty(&floor), -6);
///
/// // Empty floor = 0 penalty
/// let empty_floor = FloorLine {
///     tiles: vec![],
///     has_first_player_token: false,
/// };
/// assert_eq!(calculate_floor_penalty(&empty_floor), 0);
/// ```
pub fn calculate_floor_penalty(floor_line: &FloorLine) -> i32 {
    use crate::rules::constants::FLOOR_PENALTIES;
    
    let mut penalty = 0;
    let mut slot = 0;
    
    // First-player token occupies slot 0
    if floor_line.has_first_player_token {
        penalty += FLOOR_PENALTIES[0];
        slot = 1;
    }
    
    // Apply penalties for floor tiles (only first 7 slots count)
    let tiles_to_count = std::cmp::min(floor_line.tiles.len(), 7 - slot);
    for _ in 0..tiles_to_count {
        penalty += FLOOR_PENALTIES[slot];
        slot += 1;
    }
    
    penalty
}

/// Apply floor penalties to all players.
///
/// Calculates floor penalties for each player and subtracts from their score.
/// Ensures scores never go below 0.
///
/// # Arguments
///
/// * `state` - Mutable reference to game state
///
/// # Example
///
/// ```
/// use engine::{State, TileColor, apply_floor_penalties};
///
/// let mut state = State::new_test_state();
/// state.players[0].score = 5;
/// state.players[0].floor_line.tiles.push(TileColor::Blue);
/// state.players[0].floor_line.tiles.push(TileColor::Red);
///
/// apply_floor_penalties(&mut state);
///
/// // Score: 5 - 1 - 1 = 3
/// assert_eq!(state.players[0].score, 3);
/// ```
pub fn apply_floor_penalties(state: &mut State) {
    for player in &mut state.players {
        let penalty = calculate_floor_penalty(&player.floor_line);
        player.score = std::cmp::max(0, player.score + penalty);
    }
}
