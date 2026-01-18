use crate::State;
use super::constants::TOTAL_TILES;

/// Check that the total number of tiles in the game equals TOTAL_TILES (100)
///
/// This function counts tiles in all locations:
/// - Bag and lid
/// - Factories and center
/// - Player boards (pattern lines, wall, floor line)
///
/// # Returns
///
/// Ok(()) if conservation holds, Err(message) otherwise
///
/// # Example
///
/// ```
/// use engine::{State, TileColor, check_tile_conservation};
///
/// let mut state = State::new_test_state();
/// // Add all 100 tiles to bag for valid conservation
/// state.bag.insert(TileColor::Blue, 20);
/// state.bag.insert(TileColor::Yellow, 20);
/// state.bag.insert(TileColor::Red, 20);
/// state.bag.insert(TileColor::Black, 20);
/// state.bag.insert(TileColor::White, 20);
/// assert!(check_tile_conservation(&state).is_ok());
/// ```
pub fn check_tile_conservation(state: &State) -> Result<(), String> {
    let mut total = 0u32;
    
    // Count tiles in bag
    for count in state.bag.values() {
        total += *count as u32;
    }
    
    // Count tiles in lid
    for count in state.lid.values() {
        total += *count as u32;
    }
    
    // Count tiles in factories
    for factory in &state.factories {
        for count in factory.values() {
            total += *count as u32;
        }
    }
    
    // Count tiles in center
    for count in state.center.tiles.values() {
        total += *count as u32;
    }
    
    // Count tiles on player boards
    for player in &state.players {
        // Pattern lines
        for pattern_line in &player.pattern_lines {
            total += pattern_line.count_filled as u32;
        }
        
        // Wall (count filled positions)
        for row in &player.wall {
            for &filled in row {
                if filled {
                    total += 1;
                }
            }
        }
        
        // Floor line
        total += player.floor_line.tiles.len() as u32;
    }
    
    if total != TOTAL_TILES as u32 {
        return Err(format!(
            "Tile conservation violated: expected {}, found {}",
            TOTAL_TILES, total
        ));
    }
    
    Ok(())
}
