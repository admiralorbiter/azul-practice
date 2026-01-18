# Sprint 03A — Wall Tiling & Pattern Line Resolution

**Status:** ✅ **COMPLETED**  
**Completion Date:** January 18, 2026  
**Prerequisites:** Sprint 01 (Core Engine) ✅ and Sprint 02 (UI) ✅ complete  
**Dependencies:** State types, wall utilities from Sprint 01  
**Complexity:** Medium

---

## Goal

Implement pattern line resolution logic that correctly moves completed pattern lines to the wall and lid, with proper tile conservation.

## Outcomes

- ✓ Function to detect and resolve complete pattern lines
- ✓ Tiles correctly placed on wall at proper positions
- ✓ Excess tiles discarded to lid
- ✓ Pattern lines reset after resolution
- ✓ Tile conservation maintained throughout process

---

## Pattern Line Resolution Rules

### When to Resolve

Pattern lines are resolved at the **end of the drafting phase**, before scoring occurs.

**Condition:** A pattern line is complete when `count_filled == capacity`

**Process Order:**
1. Iterate through all players (player 0, then player 1)
2. For each player, iterate through pattern lines (rows 0-4 in sequence)
3. For each complete pattern line: place tile, discard excess, reset

---

## Algorithm: resolve_pattern_lines

### Function Signature

```rust
pub fn resolve_pattern_lines(state: &mut State)
```

**Note:** This function mutates the state in-place as part of the larger `resolve_end_of_round` operation.

### Pseudocode

```
function resolve_pattern_lines(state):
    for player_idx in 0..2:
        player = &mut state.players[player_idx]
        
        for row in 0..5:
            pattern_line = &mut player.pattern_lines[row]
            
            // Check if pattern line is complete
            if pattern_line.count_filled == pattern_line.capacity:
                color = pattern_line.color.unwrap()  // Must have color if filled
                
                // 1. Determine wall position
                col = get_wall_column_for_color(row, color)
                
                // 2. Place one tile on wall
                player.wall[row][col] = true
                
                // 3. Discard remaining tiles to lid
                tiles_to_discard = pattern_line.capacity - 1
                *state.lid.entry(color).or_insert(0) += tiles_to_discard as u8
                
                // 4. Reset pattern line
                pattern_line.count_filled = 0
                pattern_line.color = None
```

---

## Detailed Implementation Steps

### Step 1: Check Pattern Line Completion

```rust
if pattern_line.count_filled == pattern_line.capacity {
    // Pattern line is complete, proceed with resolution
}
```

**Invariants to check (debug mode):**
- If `count_filled > 0`, then `color` must be `Some(_)`
- If `count_filled == 0`, then `color` must be `None`

### Step 2: Determine Wall Position

Reuse the wall utility from Sprint 01B:

```rust
let color = pattern_line.color.expect("Complete pattern line must have color");
let col = get_wall_column_for_color(row, color);
```

**Wall Pattern Reference:**
```
     Col 0    Col 1    Col 2    Col 3    Col 4
Row 0: Blue   Yellow   Red      Black    White
Row 1: White  Blue     Yellow   Red      Black
Row 2: Black  White    Blue     Yellow   Red
Row 3: Red    Black    White    Blue     Yellow
Row 4: Yellow Red      Black    White    Blue
```

**Example:** 
- Row 2, Blue tiles → col = 2
- Row 4, Yellow tiles → col = 0

### Step 3: Place Tile on Wall

```rust
player.wall[row][col] = true;
```

**Important:** This should always succeed. If `wall[row][col]` is already `true`, it indicates a bug in the legality checking from Sprint 01B (wall conflict check failed).

**Debug assertion:**
```rust
debug_assert!(
    !player.wall[row][col],
    "Wall position [{}, {}] already filled for color {:?}",
    row, col, color
);
```

### Step 4: Discard Excess Tiles to Lid

```rust
let tiles_to_discard = pattern_line.capacity - 1;
*state.lid.entry(color).or_insert(0) += tiles_to_discard as u8;
```

**Tile Accounting:**
- Pattern line has `capacity` tiles (all same color)
- 1 tile goes to wall
- `capacity - 1` tiles go to lid

**Examples:**
- Row 0 (capacity 1): 1 tile to wall, 0 to lid
- Row 4 (capacity 5): 1 tile to wall, 4 to lid

### Step 5: Reset Pattern Line

```rust
pattern_line.count_filled = 0;
pattern_line.color = None;
```

After resolution, the pattern line is empty and ready for the next round.

---

## Edge Cases with Examples

### Edge Case 1: Multiple Complete Pattern Lines

**Initial State:**
```rust
Player 0:
  pattern_lines[0]: {capacity: 1, color: Blue, count_filled: 1}   // Complete
  pattern_lines[1]: {capacity: 2, color: Red, count_filled: 2}    // Complete
  pattern_lines[2]: {capacity: 3, color: None, count_filled: 0}   // Empty
  pattern_lines[3]: {capacity: 4, color: Yellow, count_filled: 3} // Incomplete
  pattern_lines[4]: {capacity: 5, color: White, count_filled: 5}  // Complete

Lid: {}
Wall: all false
```

**After Resolution:**
```rust
Player 0:
  pattern_lines[0]: {capacity: 1, color: None, count_filled: 0}
  pattern_lines[1]: {capacity: 2, color: None, count_filled: 0}
  pattern_lines[2]: {capacity: 3, color: None, count_filled: 0}   // Unchanged
  pattern_lines[3]: {capacity: 4, color: Yellow, count_filled: 3} // Unchanged
  pattern_lines[4]: {capacity: 5, color: None, count_filled: 0}

Lid: {Blue: 0, Red: 1, White: 4}  // 0 + 1 + 4 = 5 tiles
Wall:
  [0][0] = true  // Blue at row 0, col 0
  [1][3] = true  // Red at row 1, col 3
  [4][3] = true  // White at row 4, col 3
```

**Tile Conservation:**
- Before: 1 + 2 + 5 = 8 tiles in pattern lines
- After: 3 tiles on wall + 5 tiles in lid = 8 tiles
- ✓ Conservation maintained

---

### Edge Case 2: All 5 Pattern Lines Complete

**Initial State:**
```rust
Player 0:
  pattern_lines[0]: {capacity: 1, color: Blue, count_filled: 1}
  pattern_lines[1]: {capacity: 2, color: Yellow, count_filled: 2}
  pattern_lines[2]: {capacity: 3, color: Red, count_filled: 3}
  pattern_lines[3]: {capacity: 4, color: Black, count_filled: 4}
  pattern_lines[4]: {capacity: 5, color: White, count_filled: 5}

Lid: {}
```

**After Resolution:**
```rust
All pattern lines reset to {color: None, count_filled: 0}

Lid: {Blue: 0, Yellow: 1, Red: 2, Black: 3, White: 4}
  // Total: 0 + 1 + 2 + 3 + 4 = 10 tiles

Wall: 5 tiles placed (one per row)
```

**Tile Conservation:**
- Before: 1 + 2 + 3 + 4 + 5 = 15 tiles in pattern lines
- After: 5 tiles on wall + 10 tiles in lid = 15 tiles
- ✓ Conservation maintained

---

### Edge Case 3: No Complete Pattern Lines

**Initial State:**
```rust
Player 0:
  All pattern lines either empty or incomplete

Player 1:
  All pattern lines either empty or incomplete
```

**After Resolution:**
```
No changes to walls or lid
All pattern lines remain in their current state
```

**Behavior:** Function still iterates through all pattern lines but performs no operations.

---

### Edge Case 4: Row 0 (Capacity 1) Special Case

**Initial State:**
```rust
pattern_lines[0]: {capacity: 1, color: Red, count_filled: 1}
```

**After Resolution:**
```rust
pattern_lines[0]: {capacity: 1, color: None, count_filled: 0}
Wall[0][2] = true  // Red at row 0, col 2
Lid: {}  // No tiles discarded (capacity - 1 = 0)
```

**Note:** Row 0 is unique because it discards 0 tiles to the lid (all goes to wall).

---

## Tile Conservation Check

After resolving all pattern lines, the tile conservation invariant must still hold:

```rust
fn check_tile_conservation_after_resolution(state: &State) -> Result<(), String> {
    let mut total = 0u32;
    
    // Count tiles in all locations
    total += count_tiles_in_multiset(&state.bag);
    total += count_tiles_in_multiset(&state.lid);
    total += count_tiles_in_factories(&state.factories);
    total += count_tiles_in_center(&state.center);
    
    for player in &state.players {
        total += count_tiles_in_pattern_lines(&player.pattern_lines);
        total += count_tiles_on_wall(&player.wall);
        total += player.floor_line.tiles.len() as u32;
    }
    
    if total != 100 {
        return Err(format!(
            "Tile conservation violated after pattern line resolution: expected 100, found {}",
            total
        ));
    }
    
    Ok(())
}
```

---

## Test Requirements

### Unit Tests

**Test 1: Single Complete Pattern Line**
```rust
#[test]
fn test_resolve_single_complete_pattern_line() {
    let mut state = create_test_state();
    
    // Set up one complete pattern line
    state.players[0].pattern_lines[2] = PatternLine {
        capacity: 3,
        color: Some(TileColor::Blue),
        count_filled: 3,
    };
    
    resolve_pattern_lines(&mut state);
    
    // Verify tile placed on wall
    assert!(state.players[0].wall[2][2]); // Blue at row 2, col 2
    
    // Verify excess tiles in lid
    assert_eq!(state.lid.get(&TileColor::Blue), Some(&2));
    
    // Verify pattern line reset
    assert_eq!(state.players[0].pattern_lines[2].count_filled, 0);
    assert_eq!(state.players[0].pattern_lines[2].color, None);
}
```

**Test 2: Multiple Complete Pattern Lines**
```rust
#[test]
fn test_resolve_multiple_complete_pattern_lines() {
    let mut state = create_test_state();
    
    // Set up multiple complete pattern lines
    state.players[0].pattern_lines[0] = PatternLine {
        capacity: 1,
        color: Some(TileColor::Blue),
        count_filled: 1,
    };
    state.players[0].pattern_lines[4] = PatternLine {
        capacity: 5,
        color: Some(TileColor::Red),
        count_filled: 5,
    };
    
    resolve_pattern_lines(&mut state);
    
    // Verify both tiles placed
    assert!(state.players[0].wall[0][0]); // Blue
    assert!(state.players[0].wall[4][1]); // Red
    
    // Verify lid contents
    assert_eq!(state.lid.get(&TileColor::Blue), Some(&0)); // 0 discarded
    assert_eq!(state.lid.get(&TileColor::Red), Some(&4));  // 4 discarded
}
```

**Test 3: No Complete Pattern Lines**
```rust
#[test]
fn test_resolve_no_complete_pattern_lines() {
    let mut state = create_test_state();
    
    // Set up incomplete pattern lines
    state.players[0].pattern_lines[2] = PatternLine {
        capacity: 3,
        color: Some(TileColor::Yellow),
        count_filled: 2, // Not complete
    };
    
    let wall_before = state.players[0].wall.clone();
    let lid_before = state.lid.clone();
    
    resolve_pattern_lines(&mut state);
    
    // Verify no changes
    assert_eq!(state.players[0].wall, wall_before);
    assert_eq!(state.lid, lid_before);
    assert_eq!(state.players[0].pattern_lines[2].count_filled, 2); // Unchanged
}
```

**Test 4: Both Players with Complete Pattern Lines**
```rust
#[test]
fn test_resolve_both_players() {
    let mut state = create_test_state();
    
    // Player 0
    state.players[0].pattern_lines[1] = PatternLine {
        capacity: 2,
        color: Some(TileColor::White),
        count_filled: 2,
    };
    
    // Player 1
    state.players[1].pattern_lines[3] = PatternLine {
        capacity: 4,
        color: Some(TileColor::Black),
        count_filled: 4,
    };
    
    resolve_pattern_lines(&mut state);
    
    // Verify player 0
    assert!(state.players[0].wall[1][0]); // White at row 1, col 0
    
    // Verify player 1
    assert!(state.players[1].wall[3][1]); // Black at row 3, col 1
    
    // Verify lid
    assert_eq!(state.lid.get(&TileColor::White), Some(&1));
    assert_eq!(state.lid.get(&TileColor::Black), Some(&3));
}
```

**Test 5: Tile Conservation**
```rust
#[test]
fn test_tile_conservation_after_resolution() {
    let mut state = create_fully_populated_state();
    
    // Count tiles before
    let tiles_before = count_total_tiles(&state);
    assert_eq!(tiles_before, 100);
    
    resolve_pattern_lines(&mut state);
    
    // Count tiles after
    let tiles_after = count_total_tiles(&state);
    assert_eq!(tiles_after, 100);
}
```

**Test 6: Row 0 Special Case (No Discard)**
```rust
#[test]
fn test_row_0_no_discard() {
    let mut state = create_test_state();
    
    state.players[0].pattern_lines[0] = PatternLine {
        capacity: 1,
        color: Some(TileColor::Yellow),
        count_filled: 1,
    };
    
    resolve_pattern_lines(&mut state);
    
    // Verify tile on wall
    assert!(state.players[0].wall[0][1]); // Yellow at row 0, col 1
    
    // Verify NO tiles in lid for Yellow
    assert_eq!(state.lid.get(&TileColor::Yellow).copied().unwrap_or(0), 0);
}
```

---

## Acceptance Criteria

- [ ] Function correctly identifies complete pattern lines
- [ ] Tiles placed on wall at correct positions using wall color pattern
- [ ] Correct number of tiles discarded to lid (capacity - 1)
- [ ] Pattern lines properly reset after resolution
- [ ] Both players' pattern lines processed correctly
- [ ] Tile conservation holds after resolution
- [ ] Row 0 special case handled (0 tiles to lid)
- [ ] Empty/incomplete pattern lines unchanged
- [ ] All unit tests pass
- [ ] Debug assertions catch impossible states

---

## Files to Create/Modify

```
rust/engine/src/
├── rules/
│   ├── mod.rs              (UPDATE: export resolution module)
│   ├── resolution.rs       (NEW: pattern line resolution logic)
│   └── tests/
│       └── resolution_tests.rs (NEW: unit tests)
```

### Module Organization

**`rules/mod.rs`:**
```rust
mod constants;
mod legality;
mod wall_utils;
mod apply;
mod error;
mod invariants;
mod resolution;  // NEW

#[cfg(test)]
mod tests;

pub use constants::*;
pub use legality::*;
pub use wall_utils::*;
pub use apply::*;
pub use error::*;
pub use invariants::*;
pub use resolution::*;  // NEW
```

**`rules/resolution.rs`:**
```rust
use crate::model::{State, TileColor};
use crate::rules::wall_utils::get_wall_column_for_color;

/// Resolve all complete pattern lines for both players.
/// Places tiles on walls and discards excess to lid.
pub fn resolve_pattern_lines(state: &mut State) {
    for player_idx in 0..2 {
        let player = &mut state.players[player_idx];
        
        for row in 0..5 {
            let pattern_line = &mut player.pattern_lines[row];
            
            if pattern_line.count_filled == pattern_line.capacity {
                // Extract color (must exist if filled)
                let color = pattern_line.color.expect(
                    "Complete pattern line must have a color"
                );
                
                // Determine wall position
                let col = get_wall_column_for_color(row, color);
                
                // Debug assertion: wall position should be empty
                debug_assert!(
                    !player.wall[row][col],
                    "Wall position [{}, {}] already filled",
                    row, col
                );
                
                // Place tile on wall
                player.wall[row][col] = true;
                
                // Discard excess tiles to lid
                let tiles_to_discard = (pattern_line.capacity - 1) as u8;
                if tiles_to_discard > 0 {
                    *state.lid.entry(color).or_insert(0) += tiles_to_discard;
                }
                
                // Reset pattern line
                pattern_line.count_filled = 0;
                pattern_line.color = None;
            }
        }
    }
}
```

---

## Design Decisions

### Why In-Place Mutation?

**Decision:** Use `&mut State` instead of functional approach

**Rationale:**
- Pattern line resolution is part of the larger `resolve_end_of_round` function
- Multiple sub-operations need to modify state (tiling, scoring, refill)
- Cloning state multiple times would be wasteful
- Still maintains clear boundaries between sub-operations

**Trade-off:** Less pure, but more efficient for this use case

---

### Why Process Rows in Order?

**Decision:** Always process pattern lines from row 0 to row 4

**Rationale:**
- Deterministic behavior (important for reproducibility)
- Easier to reason about and test
- No functional difference (pattern lines are independent)
- Matches visual top-to-bottom scanning of board

---

### Why Separate Function?

**Decision:** Extract pattern line resolution into its own function

**Rationale:**
- **Modularity:** Can be tested independently
- **Clarity:** Single responsibility (just tile movement)
- **Reusability:** Called from `resolve_end_of_round` in Sprint 03C
- **Debugging:** Easier to isolate issues

---

## Integration with Sprint 03B

Sprint 03B (Scoring) will call this function and then calculate scores based on the newly placed wall tiles:

```rust
// In Sprint 03C's resolve_end_of_round function:
pub fn resolve_end_of_round(state: &mut State) {
    // 1. Resolve pattern lines (Sprint 03A)
    resolve_pattern_lines(state);
    
    // 2. Calculate and apply scores (Sprint 03B)
    for player_idx in 0..2 {
        apply_scoring(state, player_idx);
    }
    
    // 3. Handle round transition (Sprint 03C)
    // ... refill, first player, etc.
}
```

The wall state modified by this function is used by the scoring functions.

---

## Related Documentation

- [Sprint 03: Main Document](Sprint_03_End_of_Round_Scoring_Refill.md)
- [Sprint 03B: Scoring System](Sprint_03B_Scoring_System.md) - Uses wall state from this sprint
- [Sprint 03C: Round Transition](Sprint_03C_Round_Transition_Refill.md) - Orchestrates this function
- [Sprint 01B: Wall Utilities](Sprint_01B_Rules_Legality_Checks.md) - Wall color pattern helpers
- [Rules Specification](../specs/03_rules_and_edge_cases.md) - Section 3: Wall Tiling

---

## Next Steps

After completing Sprint 03A:
- Proceed to [Sprint 03B: Scoring System](Sprint_03B_Scoring_System.md)
- The wall tiles placed here will be used to calculate adjacency scores
- Pattern line resolution is the foundation for all end-of-round logic
