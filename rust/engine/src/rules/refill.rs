use crate::model::{State, TileMultiset, TileColor};
use crate::rules::constants::{FACTORY_COUNT_2P, TILES_PER_FACTORY};
use rand::Rng;

/// Draw a random tile from the bag and remove it.
///
/// Returns None if bag is empty.
fn draw_random_tile_from_bag(bag: &mut TileMultiset) -> Option<TileColor> {
    // Calculate total tiles
    let total: u8 = bag.values().sum();
    
    if total == 0 {
        return None;
    }
    
    // Pick random index
    let mut rng = rand::thread_rng();
    let mut target = rng.gen_range(0..total);
    
    // Find and remove that tile
    for (color, count) in bag.iter_mut() {
        if target < *count {
            *count -= 1;
            return Some(*color);
        }
        target -= *count;
    }
    
    None
}

/// Count total tiles in a multiset
fn count_tiles_in_multiset(multiset: &TileMultiset) -> u8 {
    multiset.values().sum()
}

/// Refill factories from bag, transferring lid to bag if needed.
///
/// Clears all factories and center, then attempts to place 4 tiles in each of 5 factories.
/// If bag has fewer than 20 tiles, transfers all lid tiles to bag first.
/// If bag runs out mid-refill, factories are partially filled (legal late-game scenario).
///
/// # Arguments
///
/// * `state` - Mutable reference to game state
pub fn refill_factories(state: &mut State) {
    // Clear existing factories and center
    for factory in &mut state.factories {
        factory.clear();
    }
    state.center.tiles.clear();
    
    // Check if we need to refill bag from lid
    let bag_count = count_tiles_in_multiset(&state.bag);
    let total_needed = (FACTORY_COUNT_2P * TILES_PER_FACTORY) as u8;
    
    if bag_count < total_needed {
        // Transfer all lid tiles to bag
        for (color, count) in state.lid.drain() {
            *state.bag.entry(color).or_insert(0) += count;
        }
    }
    
    // Fill factories
    for factory_idx in 0..FACTORY_COUNT_2P {
        for _ in 0..TILES_PER_FACTORY {
            if let Some(color) = draw_random_tile_from_bag(&mut state.bag) {
                *state.factories[factory_idx].entry(color).or_insert(0) += 1;
            } else {
                // Bag empty - factory partially filled (legal)
                break;
            }
        }
    }
}
