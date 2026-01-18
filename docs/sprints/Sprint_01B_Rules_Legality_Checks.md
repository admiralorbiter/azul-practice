# Sprint 01B — Rules & Legality Checks

**Status:** Draft  
**Prerequisites:** Sprint 01A (Data Models) complete  
**Dependencies:** State and Action types from Sprint 01A  
**Estimated Complexity:** Medium-High

---

## Goal

Implement `list_legal_actions` function with full legality validation for draft moves, including pattern line constraints, wall conflicts, and source validation.

## Outcomes

- ✓ `list_legal_actions(state, player_id)` function working correctly
- ✓ All pattern line constraints enforced
- ✓ Wall conflict detection working
- ✓ First-player token handling correct
- ✓ Floor destination always available as fallback

---

## Game Constants (2-Player Azul)

```rust
pub const FACTORY_COUNT_2P: usize = 5;
pub const TILES_PER_FACTORY: usize = 4;
pub const TILES_PER_COLOR: u8 = 20;
pub const TOTAL_TILES: u8 = 100;  // 5 colors × 20 tiles
pub const FLOOR_LINE_SLOTS: usize = 7;
pub const PATTERN_LINE_COUNT: usize = 5;
```

**Floor line penalties** (for reference in Sprint 03):
```rust
pub const FLOOR_PENALTIES: [i32; 7] = [-1, -1, -2, -2, -2, -3, -3];
```

---

## Legality Rules

### Pattern Line Constraints

**Rule 1: Capacity Constraint**
- Row 0 can hold max 1 tile
- Row 1 can hold max 2 tiles
- Row 2 can hold max 3 tiles
- Row 3 can hold max 4 tiles
- Row 4 can hold max 5 tiles

**Rule 2: Color Consistency**
- If pattern line is empty (`count_filled == 0`), any color is allowed
- If pattern line has tiles (`count_filled > 0`), new tiles MUST match existing color

**Rule 3: Wall Conflict Check**
- Determine which wall position the pattern line would tile to
- If that wall position is already filled, placement is ILLEGAL
- Use standard Azul wall pattern for color-to-column mapping

**Rule 4: Cannot Place in Complete Pattern Line**
- If `count_filled == capacity`, the pattern line is complete
- Cannot place more tiles until end-of-round (wall tiling phase)

### Source Constraints

**Factory Source:**
- Factory must contain at least one tile of the chosen color
- Player takes ALL tiles of that color from the factory
- Remaining tiles in factory move to center

**Center Source:**
- Center must contain at least one tile of the chosen color
- Player takes ALL tiles of that color from the center
- If first-player token is present, player receives it

### Floor Destination

- Placing tiles directly to floor is ALWAYS legal (no constraints)
- Even if all pattern lines are blocked, floor is available
- This ensures at least one legal action exists for any color in any source

---

## Standard Azul Wall Pattern

Each row has a different color order (rotated by 1 position):

```
     Col 0    Col 1    Col 2    Col 3    Col 4
Row 0: Blue   Yellow   Red      Black    White
Row 1: White  Blue     Yellow   Red      Black
Row 2: Black  White    Blue     Yellow   Red
Row 3: Red    Black    White    Blue     Yellow
Row 4: Yellow Red      Black    White    Blue
```

**Helper function:**
```rust
pub fn get_wall_column_for_color(row: usize, color: TileColor) -> usize {
    match (row, color) {
        (0, TileColor::Blue)   => 0,
        (0, TileColor::Yellow) => 1,
        (0, TileColor::Red)    => 2,
        (0, TileColor::Black)  => 3,
        (0, TileColor::White)  => 4,
        
        (1, TileColor::White)  => 0,
        (1, TileColor::Blue)   => 1,
        (1, TileColor::Yellow) => 2,
        (1, TileColor::Red)    => 3,
        (1, TileColor::Black)  => 4,
        
        (2, TileColor::Black)  => 0,
        (2, TileColor::White)  => 1,
        (2, TileColor::Blue)   => 2,
        (2, TileColor::Yellow) => 3,
        (2, TileColor::Red)    => 4,
        
        (3, TileColor::Red)    => 0,
        (3, TileColor::Black)  => 1,
        (3, TileColor::White)  => 2,
        (3, TileColor::Blue)   => 3,
        (3, TileColor::Yellow) => 4,
        
        (4, TileColor::Yellow) => 0,
        (4, TileColor::Red)    => 1,
        (4, TileColor::Black)  => 2,
        (4, TileColor::White)  => 3,
        (4, TileColor::Blue)   => 4,
        
        _ => panic!("Invalid row or color"),
    }
}

pub fn get_wall_color(row: usize, col: usize) -> TileColor {
    // Inverse of get_wall_column_for_color
    // Returns the color that belongs at position [row][col]
    match (row, col) {
        (0, 0) => TileColor::Blue,
        (0, 1) => TileColor::Yellow,
        (0, 2) => TileColor::Red,
        (0, 3) => TileColor::Black,
        (0, 4) => TileColor::White,
        
        (1, 0) => TileColor::White,
        (1, 1) => TileColor::Blue,
        (1, 2) => TileColor::Yellow,
        (1, 3) => TileColor::Red,
        (1, 4) => TileColor::Black,
        
        (2, 0) => TileColor::Black,
        (2, 1) => TileColor::White,
        (2, 2) => TileColor::Blue,
        (2, 3) => TileColor::Yellow,
        (2, 4) => TileColor::Red,
        
        (3, 0) => TileColor::Red,
        (3, 1) => TileColor::Black,
        (3, 2) => TileColor::White,
        (3, 3) => TileColor::Blue,
        (3, 4) => TileColor::Yellow,
        
        (4, 0) => TileColor::Yellow,
        (4, 1) => TileColor::Red,
        (4, 2) => TileColor::Black,
        (4, 3) => TileColor::White,
        (4, 4) => TileColor::Blue,
        
        _ => panic!("Invalid row or column"),
    }
}
```

---

## Algorithm: list_legal_actions

### Function Signature

```rust
pub fn list_legal_actions(state: &State, player_id: u8) -> Vec<DraftAction>
```

### Pseudocode

```
function list_legal_actions(state, player_id) -> Vec<DraftAction>:
    actions = []
    player = state.players[player_id]
    
    // Iterate through all factories
    for factory_idx in 0..state.factories.len():
        factory = state.factories[factory_idx]
        
        // For each color present in this factory
        for color in factory.keys():
            if factory[color] > 0:
                // Try placing in each pattern line (0-4)
                for row in 0..5:
                    if can_place_in_pattern_line(player, row, color):
                        actions.push(DraftAction {
                            source: Factory(factory_idx),
                            color: color,
                            destination: PatternLine(row),
                        })
                
                // Floor is always legal
                actions.push(DraftAction {
                    source: Factory(factory_idx),
                    color: color,
                    destination: Floor,
                })
    
    // Iterate through center
    for color in state.center.tiles.keys():
        if state.center.tiles[color] > 0:
            // Try placing in each pattern line (0-4)
            for row in 0..5:
                if can_place_in_pattern_line(player, row, color):
                    actions.push(DraftAction {
                        source: Center,
                        color: color,
                        destination: PatternLine(row),
                    })
            
            // Floor is always legal
            actions.push(DraftAction {
                source: Center,
                color: color,
                destination: Floor,
            })
    
    return actions
```

### Helper: can_place_in_pattern_line

```rust
fn can_place_in_pattern_line(player: &PlayerBoard, row: usize, color: TileColor) -> bool {
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
```

---

## Edge Cases with Examples

### Edge Case 1: Partially Filled Pattern Line

**Initial state:**
- Pattern line row 2 (capacity 3) has 2 Red tiles

**Legal actions:**
- Place Red tiles from any source into row 2
- Place Red tiles into floor

**Illegal actions:**
- Place Blue tiles into row 2 (COLOR_MISMATCH)
- Place Yellow tiles into row 2 (COLOR_MISMATCH)

**Test:**
```rust
#[test]
fn test_color_consistency_in_pattern_line() {
    let mut state = create_test_state();
    // Set up pattern line with 2 Red tiles
    state.players[0].pattern_lines[2] = PatternLine {
        capacity: 3,
        color: Some(TileColor::Red),
        count_filled: 2,
    };
    
    let actions = list_legal_actions(&state, 0);
    
    // Should allow Red into row 2, but not Blue
    let red_to_row2 = actions.iter().any(|a| 
        a.color == TileColor::Red && 
        a.destination == Destination::PatternLine(2)
    );
    let blue_to_row2 = actions.iter().any(|a| 
        a.color == TileColor::Blue && 
        a.destination == Destination::PatternLine(2)
    );
    
    assert!(red_to_row2);
    assert!(!blue_to_row2);
}
```

---

### Edge Case 2: Wall Conflict

**Initial state:**
- Wall[1][2] is filled (Yellow in row 1, col 2)

**Legal actions:**
- Can place Yellow in other rows (if not blocked by wall)
- Can place Yellow to floor
- Can place other colors in row 1 (if not blocked)

**Illegal actions:**
- Cannot place Yellow into pattern line 1 (would go to wall position [1][2] which is already filled)

**Test:**
```rust
#[test]
fn test_wall_conflict() {
    let mut state = create_test_state();
    // Fill wall position where Yellow goes in row 1
    state.players[0].wall[1][2] = true;  // Yellow in row 1
    
    let actions = list_legal_actions(&state, 0);
    
    // Should not allow Yellow into row 1 pattern line
    let yellow_to_row1 = actions.iter().any(|a| 
        a.color == TileColor::Yellow && 
        a.destination == Destination::PatternLine(1)
    );
    
    assert!(!yellow_to_row1);
}
```

---

### Edge Case 3: Complete Pattern Line

**Initial state:**
- Pattern line row 3 (capacity 4) has 4 Blue tiles (complete)

**Legal actions:**
- Can place Blue in other rows (if walls allow)
- Can place Blue to floor

**Illegal actions:**
- Cannot place any color into row 3 pattern line (already complete)

**Test:**
```rust
#[test]
fn test_complete_pattern_line() {
    let mut state = create_test_state();
    // Fill row 3 pattern line completely
    state.players[0].pattern_lines[3] = PatternLine {
        capacity: 4,
        color: Some(TileColor::Blue),
        count_filled: 4,
    };
    
    let actions = list_legal_actions(&state, 0);
    
    // Should not allow any color into row 3
    let any_to_row3 = actions.iter().any(|a| 
        a.destination == Destination::PatternLine(3)
    );
    
    assert!(!any_to_row3);
}
```

---

### Edge Case 4: Taking from Center with First-Player Token

**Note:** This is LEGAL, with the side effect that the player receives the first-player token.

The legality check itself does NOT reject this action. The token transfer is handled in Sprint 01C (apply_action).

```rust
#[test]
fn test_taking_from_center_with_token_is_legal() {
    let mut state = create_test_state();
    state.center.has_first_player_token = true;
    state.center.tiles.insert(TileColor::Blue, 3);
    
    let actions = list_legal_actions(&state, 0);
    
    // Should have actions from center (token doesn't block)
    let center_actions = actions.iter().filter(|a| 
        a.source == ActionSource::Center
    ).count();
    
    assert!(center_actions > 0);
}
```

---

### Edge Case 5: All Pattern Lines Blocked, Floor Available

**Initial state:**
- All 5 pattern lines are either:
  - Complete (count_filled == capacity), OR
  - Have color conflicts with wall, OR
  - Have different color than available tiles

**Legal actions:**
- Floor destination is ALWAYS available for any color

**Test:**
```rust
#[test]
fn test_floor_always_available() {
    let mut state = create_test_state();
    // Block all pattern lines for Red
    for row in 0..5 {
        let wall_col = get_wall_column_for_color(row, TileColor::Red);
        state.players[0].wall[row][wall_col] = true;
    }
    
    // Add Red tiles to a factory
    state.factories[0].insert(TileColor::Red, 2);
    
    let actions = list_legal_actions(&state, 0);
    
    // Should still have Red to Floor action
    let red_to_floor = actions.iter().any(|a| 
        a.color == TileColor::Red && 
        a.destination == Destination::Floor
    );
    
    assert!(red_to_floor);
}
```

---

## Acceptance Criteria

- [ ] `list_legal_actions` returns correct count for hand-crafted test scenarios
- [ ] No illegal actions included in output (all returned actions are valid)
- [ ] Wall conflict detection works correctly (blocked colors not offered)
- [ ] Color mismatch in pattern lines properly detected
- [ ] Complete pattern lines cannot receive more tiles
- [ ] Floor destination always available for any color from any source
- [ ] First-player token presence doesn't block center actions
- [ ] Empty factories/center don't generate actions

---

## Testing Requirements

### Essential Tests

**Test 1: Empty board scenario**
```rust
#[test]
fn test_legal_actions_empty_board() {
    let state = create_empty_board_with_full_factories();
    let actions = list_legal_actions(&state, 0);
    
    // With 5 factories × 4 tiles, expect many actions
    // Each unique color can go to 5 pattern lines + floor
    assert!(actions.len() > 50);
}
```

**Test 2: Partially filled pattern lines**
```rust
#[test]
fn test_partially_filled_pattern_lines() {
    let state = create_state_with_partial_pattern_lines();
    let actions = list_legal_actions(&state, 0);
    
    // Verify only matching colors allowed in partially filled rows
    // (tested per edge case above)
}
```

**Test 3: Wall conflicts**
```rust
#[test]
fn test_wall_conflicts() {
    let state = create_state_with_wall_tiles();
    let actions = list_legal_actions(&state, 0);
    
    // Verify colors with wall conflicts are not offered
    // for the conflicting rows
}
```

**Test 4: Complete pattern lines**
```rust
#[test]
fn test_complete_pattern_lines() {
    let state = create_state_with_complete_pattern_lines();
    let actions = list_legal_actions(&state, 0);
    
    // Verify no actions target complete pattern lines
}
```

**Test 5: Only floor available**
```rust
#[test]
fn test_only_floor_available() {
    let state = create_state_all_patterns_blocked();
    let actions = list_legal_actions(&state, 0);
    
    // Should still have floor actions for all available colors
    assert!(actions.len() > 0);
    assert!(actions.iter().all(|a| a.destination == Destination::Floor));
}
```

**Test 6: Table-driven expected counts**
```rust
#[test]
fn test_known_scenario_action_counts() {
    let scenarios = vec![
        ("early_game", 85),
        ("mid_game", 42),
        ("late_game_few_tiles", 18),
    ];
    
    for (scenario_name, expected_count) in scenarios {
        let state = load_test_scenario(scenario_name);
        let actions = list_legal_actions(&state, 0);
        assert_eq!(
            actions.len(), 
            expected_count,
            "Scenario {} should have {} actions", 
            scenario_name, 
            expected_count
        );
    }
}
```

---

## Files to Create

```
rust/engine/src/
├── rules/
│   ├── mod.rs              (module exports)
│   ├── constants.rs        (game constants)
│   ├── legality.rs         (list_legal_actions, can_place_in_pattern_line)
│   ├── wall_utils.rs       (get_wall_column_for_color, get_wall_color)
│   └── tests.rs            (legality tests)
```

### Module Organization

**`rules/mod.rs`:**
```rust
mod constants;
mod legality;
mod wall_utils;

#[cfg(test)]
mod tests;

pub use constants::*;
pub use legality::*;
pub use wall_utils::*;
```

---

## Performance Considerations

### Expected Action Counts

- **Early game (full factories):** ~80-100 actions
  - 5 factories × ~4 unique colors × 6 destinations (5 rows + floor)
- **Mid game:** ~30-50 actions
- **Late game (few tiles):** ~10-20 actions

### Optimization Notes

- Current approach: O(factories × colors × rows)
- For MVP: This is fast enough (< 1ms)
- Future optimization: Precompute pattern line availability per color

---

## Related Documentation

- [Sprint 01A: Data Models](Sprint_01A_Data_Models_Serialization.md)
- [Sprint 01C: Apply Action](Sprint_01C_Apply_Action_Transitions.md)
- [Rules Specification](../specs/03_rules_and_edge_cases.md)

---

## Next Steps

After completing Sprint 01B:
- Proceed to [Sprint 01C: Action Application & State Transitions](Sprint_01C_Apply_Action_Transitions.md)
- The legality checks will be reused for validation in `apply_action`
