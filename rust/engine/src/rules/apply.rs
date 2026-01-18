use crate::{State, DraftAction, ActionSource, Destination};
use super::{ValidationError, can_place_in_pattern_line, get_wall_column_for_color, check_tile_conservation};

/// Apply a draft action to the game state
///
/// This function validates the action, then creates a new state with the action applied.
/// It handles:
/// - Tile removal from source
/// - Factory remnants moving to center
/// - First-player token transfer
/// - Tile placement in destination (with overflow)
/// - Active player toggle
///
/// # Arguments
///
/// * `state` - The current game state
/// * `action` - The action to apply
///
/// # Returns
///
/// Ok(new_state) if action is valid, Err(ValidationError) otherwise
///
/// # Example
///
/// ```
/// use engine::{State, DraftAction, ActionSource, Destination, TileColor, apply_action};
///
/// let mut state = State::new_test_state();
/// state.factories[0].insert(TileColor::Blue, 2);
/// // Add remaining tiles to bag for conservation
/// state.bag.insert(TileColor::Blue, 18);
/// state.bag.insert(TileColor::Yellow, 20);
/// state.bag.insert(TileColor::Red, 20);
/// state.bag.insert(TileColor::Black, 20);
/// state.bag.insert(TileColor::White, 20);
/// 
/// let action = DraftAction {
///     source: ActionSource::Factory(0),
///     color: TileColor::Blue,
///     destination: Destination::PatternLine(0),
/// };
///
/// let new_state = apply_action(&state, &action).unwrap();
/// ```
pub fn apply_action(state: &State, action: &DraftAction) -> Result<State, ValidationError> {
    // Step 1: Validate action legality
    let player = &state.players[state.active_player_id as usize];
    
    // Check source exists and has the color
    let tile_count = match &action.source {
        ActionSource::Factory(idx) => {
            if *idx >= state.factories.len() {
                return Err(ValidationError::invalid_source(*idx));
            }
            *state.factories[*idx].get(&action.color).unwrap_or(&0)
        }
        ActionSource::Center => {
            *state.center.tiles.get(&action.color).unwrap_or(&0)
        }
    };
    
    if tile_count == 0 {
        return Err(ValidationError::source_empty(action.source.clone(), action.color));
    }
    
    // Check destination is legal
    match &action.destination {
        Destination::PatternLine(row) => {
            if *row >= 5 {
                return Err(ValidationError::invalid_destination(*row));
            }
            
            if !can_place_in_pattern_line(player, *row, action.color) {
                // Determine specific reason
                let pattern_line = &player.pattern_lines[*row];
                if pattern_line.count_filled == pattern_line.capacity {
                    return Err(ValidationError::pattern_line_complete(*row));
                }
                if pattern_line.count_filled > 0 && pattern_line.color != Some(action.color) {
                    return Err(ValidationError::color_mismatch(
                        *row, 
                        pattern_line.color.unwrap(), 
                        action.color
                    ));
                }
                let wall_col = get_wall_column_for_color(*row, action.color);
                if player.wall[*row][wall_col] {
                    return Err(ValidationError::wall_conflict(*row, action.color));
                }
            }
        }
        Destination::Floor => {
            // Floor is always legal, no check needed
        }
    }
    
    // Action is valid, proceed with state mutation
    // Step 2: Clone state
    let mut new_state = state.clone();
    
    // Step 3: Remove tiles from source
    match &action.source {
        ActionSource::Factory(idx) => {
            new_state.factories[*idx].remove(&action.color);
        }
        ActionSource::Center => {
            new_state.center.tiles.remove(&action.color);
        }
    }
    
    // Step 4: Move factory remnants to center (if taking from factory)
    if let ActionSource::Factory(idx) = &action.source {
        // Get all remaining tiles from factory
        for (color, count) in new_state.factories[*idx].iter() {
            *new_state.center.tiles.entry(*color).or_insert(0) += count;
        }
        
        // Clear the factory
        new_state.factories[*idx].clear();
    }
    
    // Step 5: Handle first-player token
    if action.source == ActionSource::Center && new_state.center.has_first_player_token {
        new_state.center.has_first_player_token = false;
        
        let player = &mut new_state.players[new_state.active_player_id as usize];
        player.floor_line.has_first_player_token = true;
    }
    
    // Step 6: Place tiles in destination (with overflow)
    let player = &mut new_state.players[new_state.active_player_id as usize];
    
    match &action.destination {
        Destination::PatternLine(row) => {
            let pattern_line = &mut player.pattern_lines[*row];
            
            // Calculate how many tiles fit in pattern line
            let space_available = pattern_line.capacity - pattern_line.count_filled;
            let tiles_to_place = std::cmp::min(tile_count, space_available);
            let overflow = tile_count - tiles_to_place;
            
            // Place tiles in pattern line
            pattern_line.count_filled += tiles_to_place;
            if pattern_line.count_filled > 0 {
                pattern_line.color = Some(action.color);
            }
            
            // Overflow tiles go to floor
            for _ in 0..overflow {
                player.floor_line.tiles.push(action.color);
            }
        }
        
        Destination::Floor => {
            // All tiles go directly to floor
            for _ in 0..tile_count {
                player.floor_line.tiles.push(action.color);
            }
        }
    }
    
    // Step 7: Update active player
    new_state.active_player_id = 1 - new_state.active_player_id;
    
    // Step 8: Verify invariants (in debug mode)
    #[cfg(debug_assertions)]
    {
        check_tile_conservation(&new_state)
            .expect("Tile conservation invariant violated");
    }
    
    Ok(new_state)
}
