use crate::{State, DraftAction, ActionSource, Destination, PlayerBoard, TileColor};
use super::wall_utils::get_wall_column_for_color;

/// List all legal draft actions for the given player in the given state
///
/// This function enumerates all valid moves the player can make during the draft phase.
/// It checks:
/// - Which sources (factories/center) have tiles
/// - Which destinations (pattern lines/floor) are legal for each color
/// - Pattern line constraints (capacity, color consistency, wall conflicts)
///
/// The floor destination is always legal, ensuring at least one action per color/source.
///
/// # Arguments
///
/// * `state` - The current game state
/// * `player_id` - The player to check (0 or 1)
///
/// # Returns
///
/// A vector of all legal draft actions for the player
///
/// # Example
///
/// ```
/// use engine::{State, list_legal_actions};
///
/// let state = State::new_test_state();
/// let actions = list_legal_actions(&state, 0);
/// // Returns all legal moves for player 0
/// ```
pub fn list_legal_actions(state: &State, player_id: u8) -> Vec<DraftAction> {
    let mut actions = Vec::new();
    let player = &state.players[player_id as usize];
    
    // Check all factories
    for (factory_idx, factory) in state.factories.iter().enumerate() {
        for (&color, &count) in factory.iter() {
            if count > 0 {
                // Try placing in each pattern line
                for row in 0..5 {
                    if can_place_in_pattern_line(player, row, color) {
                        actions.push(DraftAction {
                            source: ActionSource::Factory(factory_idx),
                            color,
                            destination: Destination::PatternLine(row),
                        });
                    }
                }
                
                // Floor is always legal
                actions.push(DraftAction {
                    source: ActionSource::Factory(factory_idx),
                    color,
                    destination: Destination::Floor,
                });
            }
        }
    }
    
    // Check center
    for (&color, &count) in state.center.tiles.iter() {
        if count > 0 {
            // Try placing in each pattern line
            for row in 0..5 {
                if can_place_in_pattern_line(player, row, color) {
                    actions.push(DraftAction {
                        source: ActionSource::Center,
                        color,
                        destination: Destination::PatternLine(row),
                    });
                }
            }
            
            // Floor is always legal
            actions.push(DraftAction {
                source: ActionSource::Center,
                color,
                destination: Destination::Floor,
            });
        }
    }
    
    actions
}

/// Check if a color can be legally placed in a pattern line
///
/// Checks three constraints:
/// 1. Pattern line is not complete (count_filled < capacity)
/// 2. Color matches existing tiles in pattern line (if any)
/// 3. Color doesn't conflict with wall (not already placed in that row)
///
/// # Arguments
///
/// * `player` - The player's board to check
/// * `row` - Pattern line row index (0-4)
/// * `color` - The tile color to check
///
/// # Returns
///
/// `true` if the color can be legally placed, `false` otherwise
pub(crate) fn can_place_in_pattern_line(player: &PlayerBoard, row: usize, color: TileColor) -> bool {
    let pattern_line = &player.pattern_lines[row];
    
    // Check 1: Pattern line must not be complete
    if pattern_line.count_filled == pattern_line.capacity {
        return false;
    }
    
    // Check 2: Color consistency (if pattern line has tiles, color must match)
    if pattern_line.count_filled > 0 {
        if let Some(existing_color) = pattern_line.color {
            if existing_color != color {
                return false;
            }
        }
    }
    
    // Check 3: Wall conflict (if wall already has this color in this row)
    let wall_col = get_wall_column_for_color(row, color);
    if player.wall[row][wall_col] {
        return false;
    }
    
    true
}
