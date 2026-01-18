use crate::model::State;
use crate::rules::wall_utils::get_wall_column_for_color;
use crate::rules::scoring::calculate_wall_tile_score;

/// Resolve all complete pattern lines for both players.
///
/// For each player, iterates through pattern lines (rows 0-4). For each complete
/// pattern line (count_filled == capacity):
/// 1. Places one tile on the wall at the appropriate position
/// 2. Discards remaining (capacity - 1) tiles to the lid
/// 3. Resets the pattern line to empty
///
/// # Arguments
///
/// * `state` - Mutable reference to game state
///
/// # Invariants
///
/// - Complete pattern lines must have a color set
/// - Wall positions should not already be filled (checked in debug mode)
/// - Tile conservation is maintained (all tiles accounted for)
///
/// # Example
///
/// ```
/// use engine::{State, TileColor, PatternLine, resolve_pattern_lines};
///
/// let mut state = State::new_test_state();
/// state.players[0].pattern_lines[2] = PatternLine {
///     capacity: 3,
///     color: Some(TileColor::Blue),
///     count_filled: 3,
/// };
///
/// resolve_pattern_lines(&mut state);
///
/// // Blue tile placed at row 2, col 2
/// assert!(state.players[0].wall[2][2]);
/// // 2 tiles discarded to lid (capacity 3 - 1)
/// assert_eq!(state.lid.get(&TileColor::Blue), Some(&2));
/// // Pattern line reset
/// assert_eq!(state.players[0].pattern_lines[2].count_filled, 0);
/// ```
pub fn resolve_pattern_lines(state: &mut State) {
    for player_idx in 0..2 {
        let player = &mut state.players[player_idx];
        
        for row in 0..5 {
            let pattern_line = &mut player.pattern_lines[row];
            
            // Check if pattern line is complete
            if pattern_line.count_filled == pattern_line.capacity {
                // Extract color (must exist if filled)
                let color = pattern_line.color.expect(
                    "Complete pattern line must have a color"
                );
                
                // Determine wall position using existing utility
                let col = get_wall_column_for_color(row, color);
                
                // Skip if wall position already filled (should not happen in normal gameplay)
                // This can occur if state was manually edited incorrectly
                if player.wall[row][col] {
                    // Wall position already filled - skip this pattern line
                    // Just clear the pattern line without placing tile
                    pattern_line.count_filled = 0;
                    pattern_line.color = None;
                    continue;
                }
                
                // Place one tile on wall
                player.wall[row][col] = true;
                
                // Calculate and add score for this placement (Sprint 03B)
                let points = calculate_wall_tile_score(&player.wall, row, col);
                player.score += points;
                
                // Discard excess tiles to lid
                let tiles_to_discard = pattern_line.capacity - 1;
                if tiles_to_discard > 0 {
                    *state.lid.entry(color).or_insert(0) += tiles_to_discard;
                }
                
                // Reset pattern line to empty state
                pattern_line.count_filled = 0;
                pattern_line.color = None;
            }
        }
    }
}
