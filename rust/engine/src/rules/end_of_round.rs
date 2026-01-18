use crate::model::State;
use crate::rules::error::ValidationError;
use crate::rules::resolution::resolve_pattern_lines;
use crate::rules::scoring::apply_floor_penalties;
use crate::rules::refill::refill_factories;

/// Check if game has ended (any player has complete horizontal row)
pub fn check_game_end(state: &State) -> bool {
    for player in &state.players {
        for row in &player.wall {
            if row.iter().all(|&filled| filled) {
                return true;
            }
        }
    }
    false
}

/// Resolve end of round: score tiles, apply penalties, cleanup, check end, refill.
///
/// Orchestrates complete end-of-round flow:
/// 1. Pattern line resolution with scoring (Sprint 03A + 03B)
/// 2. Floor penalty application (Sprint 03B)
/// 3. Floor cleanup and first player determination
/// 4. Game end detection
/// 5. Factory refill for next round (if game continues)
///
/// # Arguments
///
/// * `state` - Reference to current game state
///
/// # Returns
///
/// * `Ok(State)` - New state after end-of-round resolution
/// * `Err(ValidationError)` - If state is invalid
///
/// # Example
///
/// ```no_run
/// # use engine::{State, resolve_end_of_round};
/// let state = State::new_test_state();
/// let new_state = resolve_end_of_round(&state).unwrap();
/// assert_eq!(new_state.round_number, state.round_number + 1);
/// ```
pub fn resolve_end_of_round(state: &State) -> Result<State, ValidationError> {
    let mut new_state = state.clone();
    
    // ========== Phase 1: Wall Tiling & Scoring ==========
    
    // Resolve pattern lines and score (Sprint 03A + 03B integrated)
    resolve_pattern_lines(&mut new_state);
    
    // Apply floor penalties (Sprint 03B)
    apply_floor_penalties(&mut new_state);
    
    // ========== Phase 2: Cleanup ==========
    
    // Determine next first player (whoever has token)
    let next_first_player = if new_state.players[0].floor_line.has_first_player_token {
        0
    } else if new_state.players[1].floor_line.has_first_player_token {
        1
    } else {
        // No one has token - keep current (shouldn't happen)
        new_state.active_player_id
    };
    
    // Clear floor lines and discard tiles to lid
    for player in &mut new_state.players {
        // Discard floor tiles to lid
        for tile_color in &player.floor_line.tiles {
            *new_state.lid.entry(*tile_color).or_insert(0) += 1;
        }
        
        // Clear floor line
        player.floor_line.tiles.clear();
        player.floor_line.has_first_player_token = false;
    }
    
    // Move token to center for next round
    new_state.center.has_first_player_token = true;
    new_state.active_player_id = next_first_player;
    
    // ========== Phase 3: Check Game End ==========
    
    if check_game_end(&new_state) {
        // Game is over, do not refill factories
        // Future: add end-of-game bonuses here
        return Ok(new_state);
    }
    
    // ========== Phase 4: Refill for Next Round ==========
    
    new_state.round_number += 1;
    refill_factories(&mut new_state);
    
    Ok(new_state)
}
