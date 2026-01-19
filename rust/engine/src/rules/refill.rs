use crate::model::{State, TileMultiset, TileColor};
use crate::rules::constants::{ALL_COLORS, FACTORY_COUNT_2P, TILES_PER_FACTORY};
use rand::Rng;

/// Draw a random tile from the bag and remove it.
///
/// Uses the provided RNG for deterministic behavior when needed.
/// Iterates over colors in a fixed order to ensure determinism.
///
/// Returns None if bag is empty.
fn draw_random_tile_from_bag<R: Rng>(bag: &mut TileMultiset, rng: &mut R) -> Option<TileColor> {
    // Calculate total tiles
    let total: u8 = bag.values().sum();
    
    if total == 0 {
        return None;
    }
    
    // Pick random index
    let mut target = rng.gen_range(0..total);
    
    // Iterate in a fixed, rules-defined order for determinism.
    // This avoids HashMap iteration order affecting reproducibility.
    for &color in &ALL_COLORS {
        let count = bag.get(&color).copied().unwrap_or(0);
        if count == 0 {
            continue;
        }

        if target < count {
            // decrement chosen color
            let entry = bag.get_mut(&color).expect("count>0 implies key exists");
            *entry -= 1;
            if *entry == 0 {
                bag.remove(&color);
            }
            return Some(color);
        }

        target -= count;
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
/// * `rng` - Random number generator (use seeded RNG for deterministic behavior)
pub fn refill_factories_with_rng<R: Rng>(state: &mut State, rng: &mut R) {
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
            if let Some(color) = draw_random_tile_from_bag(&mut state.bag, rng) {
                *state.factories[factory_idx].entry(color).or_insert(0) += 1;
            } else {
                // Bag empty - factory partially filled (legal)
                break;
            }
        }
    }
}

/// Refill factories using thread-local RNG (non-deterministic).
///
/// This is a convenience wrapper for backward compatibility with existing code
/// that doesn't need reproducible scenarios (like end-of-round resolution in normal play).
///
/// For deterministic behavior (e.g., scenario generation), use `refill_factories_with_rng`
/// with a seeded RNG instead.
///
/// # Arguments
///
/// * `state` - Mutable reference to game state
pub fn refill_factories(state: &mut State) {
    let mut rng = rand::thread_rng();
    refill_factories_with_rng(state, &mut rng);
}
