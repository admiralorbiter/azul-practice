# Sprint 03C — Round Transition & Refill

**Status:** Draft  
**Prerequisites:** Sprint 03A (Wall Tiling) and Sprint 03B (Scoring) complete  
**Dependencies:** Pattern line resolution, scoring functions  
**Estimated Complexity:** High

---

## Goal

Implement complete end-of-round resolution including floor line cleanup, factory refill with bag/lid mechanics, first player determination, game end detection, and WASM integration.

## Outcomes

- ✓ Complete `resolve_end_of_round` function orchestrating all subsystems
- ✓ Factory refill from bag working correctly
- ✓ Bag refill from lid when insufficient tiles
- ✓ First player token handling for next round
- ✓ Game end detection (complete horizontal row)
- ✓ Floor lines properly cleared and tiles discarded
- ✓ WASM export and TypeScript wrapper
- ✓ Dev UI button for testing

---

## Complete End-of-Round Flow

### High-Level Algorithm

```
function resolve_end_of_round(state) -> Result<State, Error>:
    // Phase 1: Wall Tiling & Scoring (Sprint 03A + 03B)
    1. For each player:
        a. Resolve complete pattern lines → place tiles on wall
        b. Calculate and add wall tile scores
    2. For each player:
        a. Calculate floor penalties
        b. Apply penalties (score cannot go below 0)
    
    // Phase 2: Cleanup
    3. For each player:
        a. Discard floor tiles to lid
        b. Clear floor line
    4. Determine first player for next round
    5. Move first-player token to center
    
    // Phase 3: Game State Check
    6. Check if game has ended
    7. If game ended:
        return state with game_ended flag
    
    // Phase 4: Next Round Setup
    8. If game continues:
        a. Increment round number
        b. Refill factories from bag (with lid refill if needed)
    
    9. Return updated state
```

---

## Function: resolve_end_of_round

### Function Signature

```rust
pub fn resolve_end_of_round(state: &State) -> Result<State, ValidationError>
```

**Approach:** Functional (returns new state, doesn't mutate input)

**Returns:** 
- `Ok(State)` - Successfully resolved, game continues or ended
- `Err(ValidationError)` - Internal error (should not happen with valid states)

### Complete Pseudocode

```rust
pub fn resolve_end_of_round(state: &State) -> Result<State, ValidationError> {
    let mut new_state = state.clone();
    
    // ========== Phase 1: Wall Tiling & Scoring ==========
    
    // Resolve pattern lines and calculate wall scores (Sprint 03A + 03B)
    for player_idx in 0..2 {
        let player = &mut new_state.players[player_idx];
        
        for row in 0..5 {
            let pattern_line = &mut player.pattern_lines[row];
            
            if pattern_line.count_filled == pattern_line.capacity {
                let color = pattern_line.color.unwrap();
                let col = get_wall_column_for_color(row, color);
                
                // Place tile on wall
                player.wall[row][col] = true;
                
                // Score this placement
                let points = calculate_wall_tile_score(&player.wall, row, col);
                player.score += points;
                
                // Discard excess to lid
                let tiles_to_discard = (pattern_line.capacity - 1) as u8;
                if tiles_to_discard > 0 {
                    *new_state.lid.entry(color).or_insert(0) += tiles_to_discard;
                }
                
                // Reset pattern line
                pattern_line.count_filled = 0;
                pattern_line.color = None;
            }
        }
    }
    
    // Apply floor penalties (Sprint 03B)
    for player in &mut new_state.players {
        let penalty = calculate_floor_penalty(&player.floor_line);
        player.score = std::cmp::max(0, player.score + penalty);
    }
    
    // ========== Phase 2: Cleanup ==========
    
    // Determine next first player (whoever has token)
    let next_first_player = if new_state.players[0].floor_line.has_first_player_token {
        0
    } else if new_state.players[1].floor_line.has_first_player_token {
        1
    } else {
        // No one has token? Keep current first player (shouldn't happen)
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
    new_state.active_player_id = next_first_player as u8;
    
    // ========== Phase 3: Check Game End ==========
    
    let game_ended = check_game_end(&new_state);
    
    if game_ended {
        // Game is over, do not refill factories
        // (In future: add end-of-game bonuses here)
        return Ok(new_state);
    }
    
    // ========== Phase 4: Refill for Next Round ==========
    
    new_state.round_number += 1;
    refill_factories(&mut new_state)?;
    
    Ok(new_state)
}
```

---

## Floor Line Cleanup

### Discard Tiles to Lid

All floor line tiles are discarded to the lid:

```rust
for player in &mut state.players {
    for tile_color in &player.floor_line.tiles {
        *state.lid.entry(*tile_color).or_insert(0) += 1;
    }
    player.floor_line.tiles.clear();
}
```

**Tile Conservation:**
- Before: N tiles in floor lines
- After: N tiles in lid, 0 tiles in floor lines

### First-Player Token Handling

```rust
// Determine who has the token
let next_first_player = if state.players[0].floor_line.has_first_player_token {
    0
} else if state.players[1].floor_line.has_first_player_token {
    1
} else {
    state.active_player_id  // Default (shouldn't happen)
};

// Remove token from floors
for player in &mut state.players {
    player.floor_line.has_first_player_token = false;
}

// Place token in center
state.center.has_first_player_token = true;
state.active_player_id = next_first_player as u8;
```

---

## Game End Detection

### Function: check_game_end

```rust
pub fn check_game_end(state: &State) -> bool {
    for player in &state.players {
        for row in &player.wall {
            if row.iter().all(|&filled| filled) {
                // Found a complete horizontal row
                return true;
            }
        }
    }
    false
}
```

**Game End Condition:** Any player has a complete horizontal row (all 5 tiles in a row)

**Examples:**

Complete row (game ends):
```
[X][X][X][X][X]  ← All 5 tiles filled
[ ][ ][X][ ][ ]
[ ][X][ ][X][ ]
[X][ ][ ][ ][ ]
[ ][ ][ ][ ][ ]
```

Not complete (game continues):
```
[X][X][X][X][ ]  ← Missing one tile
[X][ ][X][ ][ ]
[ ][X][ ][X][X]
[X][ ][ ][X][ ]
[ ][ ][X][ ][X]
```

---

## Factory Refill Mechanics

### Constants

```rust
pub const FACTORY_COUNT_2P: usize = 5;
pub const TILES_PER_FACTORY: usize = 4;
pub const TOTAL_FACTORY_TILES: usize = FACTORY_COUNT_2P * TILES_PER_FACTORY; // 20
```

### Function: refill_factories

```rust
pub fn refill_factories(state: &mut State) -> Result<(), String> {
    // Clear existing factories and center
    for factory in &mut state.factories {
        factory.clear();
    }
    state.center.tiles.clear();
    
    // Check if we need to refill bag from lid
    let bag_count = count_tiles_in_multiset(&state.bag);
    if bag_count < TOTAL_FACTORY_TILES {
        // Transfer all lid tiles to bag
        for (color, count) in state.lid.iter() {
            *state.bag.entry(*color).or_insert(0) += count;
        }
        state.lid.clear();
    }
    
    // Fill factories
    for factory_idx in 0..FACTORY_COUNT_2P {
        for _ in 0..TILES_PER_FACTORY {
            // Draw random tile from bag
            if let Some(color) = draw_random_tile_from_bag(&mut state.bag) {
                *state.factories[factory_idx].entry(color).or_insert(0) += 1;
            } else {
                // Bag is empty, factory is partially filled
                // This is legal (late game scenario)
                break;
            }
        }
    }
    
    Ok(())
}
```

### Helper: draw_random_tile_from_bag

```rust
fn draw_random_tile_from_bag(bag: &mut TileMultiset) -> Option<TileColor> {
    // Calculate total tiles in bag
    let total: u8 = bag.values().sum();
    
    if total == 0 {
        return None;
    }
    
    // Pick random index
    let mut target = rand::thread_rng().gen_range(0..total);
    
    // Find and remove that tile
    for (color, count) in bag.iter_mut() {
        if target < *count {
            *count -= 1;
            if *count == 0 {
                // Remove color from map if depleted
                let color_to_remove = *color;
                bag.remove(&color_to_remove);
                return Some(color_to_remove);
            }
            return Some(*color);
        }
        target -= *count;
    }
    
    None  // Should not reach here
}
```

**Note:** Requires `rand` crate for randomness. For deterministic testing, use seeded RNG.

---

## Refill Edge Cases

### Edge Case 1: Normal Refill (Bag Has Enough Tiles)

**Before:**
```
Bag: {Blue: 10, Red: 8, Yellow: 12, Black: 9, White: 11}  // Total: 50 tiles
Lid: {Blue: 2, Red: 3}
Factories: all empty
```

**After:**
```
Bag: {Blue: 8, Red: 7, Yellow: 10, Black: 6, White: 9}  // 20 tiles drawn
Lid: {Blue: 2, Red: 3}  // Unchanged
Factories: Each has 4 tiles (20 total)
```

---

### Edge Case 2: Bag Refill from Lid

**Before:**
```
Bag: {Blue: 5, Red: 3}  // Total: 8 tiles (< 20 needed)
Lid: {Yellow: 7, Black: 6, White: 8}  // Total: 21 tiles
Factories: all empty
```

**Process:**
```
1. Check: bag_count (8) < 20 → Need refill
2. Transfer all lid to bag:
   Bag: {Blue: 5, Red: 3, Yellow: 7, Black: 6, White: 8}  // Total: 29 tiles
   Lid: {}
3. Draw 20 tiles from bag to factories
4. Bag: 9 tiles remaining
```

**After:**
```
Bag: ~9 tiles (exact colors depend on randomness)
Lid: {}
Factories: Full (20 tiles total)
```

---

### Edge Case 3: Insufficient Tiles (Late Game)

**Before:**
```
Bag: {Blue: 3, Red: 2}  // Total: 5 tiles
Lid: {Yellow: 1}  // Total: 1 tile
Factories: all empty
```

**Process:**
```
1. Check: bag_count (5) < 20 → Need refill
2. Transfer lid to bag:
   Bag: {Blue: 3, Red: 2, Yellow: 1}  // Total: 6 tiles
   Lid: {}
3. Try to fill factories:
   - Factory 0: 4 tiles (Blue, Red, Blue, Yellow) ← Full
   - Factory 1: 2 tiles (Red, Blue) ← Partial
   - Factory 2: 0 tiles ← Empty
   - Factory 3: 0 tiles ← Empty
   - Factory 4: 0 tiles ← Empty
4. Bag depleted
```

**After:**
```
Bag: {}
Lid: {}
Factories: Partially filled (6 tiles total)
```

**Note:** This is legal! Game can continue with partial factories. This typically happens late in the game after many wall placements.

---

### Edge Case 4: First Round Refill

**Before:**
```
Bag: {Blue: 20, Red: 20, Yellow: 20, Black: 20, White: 20}  // Total: 100
Lid: {}
Factories: all empty
Round: 1
```

**After:**
```
Bag: 80 tiles (20 drawn)
Lid: {}
Factories: All full (20 tiles total)
Round: 1
```

---

## State Transition Example

### Complete End-of-Round Transition

**Before End-of-Round:**
```json
{
  "round_number": 2,
  "active_player_id": 0,
  "bag": {"Blue": 8, "Red": 7, "Yellow": 10, "Black": 9, "White": 11},
  "lid": {"Blue": 2, "Red": 3},
  "factories": [{}, {}, {}, {}, {}],  // Empty (drafting finished)
  "center": {
    "tiles": {},
    "has_first_player_token": false
  },
  "players": [
    {
      "score": 12,
      "pattern_lines": [
        {"capacity": 1, "color": "Blue", "count_filled": 1},   // Complete
        {"capacity": 2, "color": null, "count_filled": 0},
        {"capacity": 3, "color": "Red", "count_filled": 3},    // Complete
        {"capacity": 4, "color": null, "count_filled": 0},
        {"capacity": 5, "color": null, "count_filled": 0}
      ],
      "wall": [
        [false, false, false, false, false],
        [false, false, false, false, false],
        [false, false, false, false, false],
        [false, false, false, false, false],
        [false, false, false, false, false]
      ],
      "floor_line": {
        "tiles": ["Black", "Yellow"],
        "has_first_player_token": true
      }
    },
    {
      "score": 15,
      "pattern_lines": [
        {"capacity": 1, "color": null, "count_filled": 0},
        {"capacity": 2, "color": null, "count_filled": 0},
        {"capacity": 3, "color": null, "count_filled": 0},
        {"capacity": 4, "color": "White", "count_filled": 4},  // Complete
        {"capacity": 5, "color": null, "count_filled": 0}
      ],
      "wall": [
        [false, true, false, false, false],
        [false, false, false, false, false],
        [false, false, false, false, false],
        [false, false, false, false, false],
        [false, false, false, false, false]
      ],
      "floor_line": {
        "tiles": [],
        "has_first_player_token": false
      }
    }
  ]
}
```

**After End-of-Round:**
```json
{
  "round_number": 3,  // Incremented
  "active_player_id": 0,  // Player 0 had token
  "bag": {"Blue": 6, "Red": 5, "Yellow": 8, "Black": 7, "White": 9},  // 20 drawn
  "lid": {"Blue": 2, "Red": 5, "Black": 1, "Yellow": 1},  // Floor tiles + pattern excess
  "factories": [  // Refilled
    {"Blue": 2, "Red": 1, "Yellow": 1},
    {"Black": 1, "White": 2, "Blue": 1},
    {"Yellow": 2, "Red": 1, "White": 1},
    {"Black": 2, "Blue": 1, "White": 1},
    {"Red": 1, "Yellow": 1, "Black": 1, "White": 1}
  ],
  "center": {
    "tiles": {},
    "has_first_player_token": true  // Returned from floor
  },
  "players": [
    {
      "score": 12,  // +1 (isolated Blue) +1 (isolated Red) -3 (floor) = 11... wait let me recalc
      // Actually: 12 + 1 (Blue isolated) + 1 (Red isolated) - 1 (token) - 1 (Black) - 2 (Yellow) = 10
      "score": 10,
      "pattern_lines": [
        {"capacity": 1, "color": null, "count_filled": 0},  // Cleared
        {"capacity": 2, "color": null, "count_filled": 0},
        {"capacity": 3, "color": null, "count_filled": 0},  // Cleared
        {"capacity": 4, "color": null, "count_filled": 0},
        {"capacity": 5, "color": null, "count_filled": 0}
      ],
      "wall": [
        [true, false, false, false, false],   // Blue placed
        [false, false, false, false, false],
        [false, false, false, true, false],   // Red placed
        [false, false, false, false, false],
        [false, false, false, false, false]
      ],
      "floor_line": {
        "tiles": [],  // Cleared
        "has_first_player_token": false
      }
    },
    {
      "score": 16,  // 15 + 1 (isolated White)
      "pattern_lines": [
        {"capacity": 1, "color": null, "count_filled": 0},
        {"capacity": 2, "color": null, "count_filled": 0},
        {"capacity": 3, "color": null, "count_filled": 0},
        {"capacity": 4, "color": null, "count_filled": 0},  // Cleared
        {"capacity": 5, "color": null, "count_filled": 0}
      ],
      "wall": [
        [false, true, false, false, false],   // Existing
        [false, false, false, false, false],
        [false, false, false, false, false],
        [false, false, false, true, false],   // White placed
        [false, false, false, false, false]
      ],
      "floor_line": {
        "tiles": [],
        "has_first_player_token": false
      }
    }
  ]
}
```

**Changes Summary:**
1. ✓ Pattern lines resolved (3 complete lines)
2. ✓ Wall tiles placed (Blue at [0][0], Red at [2][3], White at [3][3])
3. ✓ Scores updated (+2 for P0, +1 for P1)
4. ✓ Floor penalties applied (-4 for P0, 0 for P1)
5. ✓ Floor tiles discarded to lid
6. ✓ First player token returned to center
7. ✓ Round incremented (2 → 3)
8. ✓ Factories refilled with 20 tiles

---

## WASM Integration

### WASM Export

```rust
#[wasm_bindgen]
pub fn resolve_end_of_round(state_json: &str) -> String {
    // Parse state
    let state: State = match serde_json::from_str(state_json) {
        Ok(s) => s,
        Err(e) => {
            return serialize_error(
                "INVALID_STATE_JSON",
                &format!("Failed to parse state: {}", e),
                None
            );
        }
    };
    
    // Resolve end of round
    match resolve_end_of_round_internal(&state) {
        Ok(new_state) => {
            match serde_json::to_string(&new_state) {
                Ok(json) => json,
                Err(e) => serialize_error(
                    "SERIALIZATION_ERROR",
                    &format!("Failed to serialize state: {}", e),
                    None
                )
            }
        }
        Err(e) => {
            let error = json!({
                "error": {
                    "code": e.code,
                    "message": e.message,
                    "context": e.context,
                }
            });
            serde_json::to_string(&error).unwrap()
        }
    }
}
```

### TypeScript Wrapper

```typescript
// web/src/wasm/engine.ts

/**
 * Resolve end-of-round: score tiles, apply penalties, refill factories.
 * 
 * @param state - Current game state (drafting phase must be complete)
 * @returns Updated state for next round or error
 */
export function resolveEndOfRound(
  state: GameState
): GameState | EngineError {
  try {
    const resultJson = wasm.resolve_end_of_round(JSON.stringify(state));
    const result = JSON.parse(resultJson);
    
    if (isError(result)) {
      console.error('Engine error:', result.error);
    }
    
    return result;
  } catch (e) {
    return {
      error: {
        code: 'JS_ERROR',
        message: `JavaScript error: ${e}`,
        context: { exception: String(e) }
      }
    };
  }
}
```

---

## Dev UI Integration

### Add Button to DevPanel

```typescript
// web/src/components/dev/DevPanel.tsx

export function DevPanel({ state, onStateChange }: DevPanelProps) {
  const handleResolveRound = () => {
    if (!state) return;
    
    const result = resolveEndOfRound(state);
    
    if (isError(result)) {
      alert(`Error: ${result.error.message}`);
    } else {
      onStateChange(result);
    }
  };
  
  return (
    <div className="dev-panel">
      {/* ... existing content ... */}
      
      <button 
        onClick={handleResolveRound}
        disabled={!state}
        className="resolve-round-btn"
      >
        Resolve End of Round
      </button>
    </div>
  );
}
```

### CSS Styling

```css
/* web/src/components/dev/DevPanel.css */

.resolve-round-btn {
  background: #4caf50;
  color: white;
  padding: 10px 20px;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-weight: bold;
  margin: 10px 0;
}

.resolve-round-btn:hover {
  background: #45a049;
}

.resolve-round-btn:disabled {
  background: #cccccc;
  cursor: not-allowed;
}
```

---

## Acceptance Criteria

- [ ] `resolve_end_of_round` successfully orchestrates all subsystems
- [ ] Pattern lines resolved and scored correctly (Sprint 03A + 03B)
- [ ] Floor penalties applied correctly (Sprint 03B)
- [ ] Floor tiles discarded to lid
- [ ] First player token returned to center
- [ ] Next first player determined correctly
- [ ] Round number increments
- [ ] Factories refill from bag (20 tiles)
- [ ] Bag refills from lid when insufficient
- [ ] Partial factory fill handled (late game)
- [ ] Game end detected correctly (complete horizontal row)
- [ ] Tile conservation maintained throughout
- [ ] WASM export works from browser
- [ ] Dev UI button triggers end-of-round
- [ ] State updates correctly in UI

---

## Test Requirements

### Integration Tests

**Test 1: Complete End-of-Round**
```rust
#[test]
fn test_complete_end_of_round() {
    let state = create_state_ready_for_end_of_round();
    
    let result = resolve_end_of_round(&state).unwrap();
    
    // Verify round incremented
    assert_eq!(result.round_number, state.round_number + 1);
    
    // Verify factories refilled
    assert!(result.factories.iter().any(|f| !f.is_empty()));
    
    // Verify floor lines cleared
    for player in &result.players {
        assert_eq!(player.floor_line.tiles.len(), 0);
        assert!(!player.floor_line.has_first_player_token);
    }
    
    // Verify token in center
    assert!(result.center.has_first_player_token);
}
```

**Test 2: Bag Refill from Lid**
```rust
#[test]
fn test_bag_refill_from_lid() {
    let mut state = create_test_state();
    state.bag = create_multiset(&[("Blue", 5), ("Red", 3)]);  // 8 tiles
    state.lid = create_multiset(&[("Yellow", 10), ("Black", 8)]);  // 18 tiles
    
    let result = resolve_end_of_round(&state).unwrap();
    
    // Verify lid was used to refill bag
    assert_eq!(count_tiles_in_multiset(&result.lid), 0);
    
    // Verify factories filled
    let factory_count = count_tiles_in_factories(&result.factories);
    assert_eq!(factory_count, 20);
}
```

**Test 3: Game End Detection**
```rust
#[test]
fn test_game_end_detection() {
    let mut state = create_test_state();
    
    // Set up complete horizontal row
    state.players[0].wall[2] = [true, true, true, true, true];
    
    let result = resolve_end_of_round(&state).unwrap();
    
    // Game should end, no factory refill
    assert!(check_game_end(&result));
    assert!(result.factories.iter().all(|f| f.is_empty()));
}
```

**Test 4: Tile Conservation**
```rust
#[test]
fn test_tile_conservation_through_end_of_round() {
    let state = create_fully_populated_state();
    
    let tiles_before = count_total_tiles(&state);
    assert_eq!(tiles_before, 100);
    
    let result = resolve_end_of_round(&state).unwrap();
    
    let tiles_after = count_total_tiles(&result);
    assert_eq!(tiles_after, 100);
}
```

**Test 5: First Player Determination**
```rust
#[test]
fn test_first_player_determination() {
    let mut state = create_test_state();
    
    // Player 1 has token
    state.players[1].floor_line.has_first_player_token = true;
    
    let result = resolve_end_of_round(&state).unwrap();
    
    // Player 1 should be first player next round
    assert_eq!(result.active_player_id, 1);
    
    // Token should be in center
    assert!(result.center.has_first_player_token);
    assert!(!result.players[1].floor_line.has_first_player_token);
}
```

---

## Files to Create/Modify

```
rust/engine/src/
├── rules/
│   ├── mod.rs              (UPDATE: export end_of_round module)
│   ├── end_of_round.rs     (NEW: orchestrates all subsystems)
│   ├── refill.rs           (NEW: factory refill logic)
│   └── tests/
│       └── end_of_round_tests.rs (NEW: integration tests)
├── wasm_api.rs             (UPDATE: add resolve_end_of_round export)
└── lib.rs                  (UPDATE: export new functions)

web/src/
├── wasm/
│   └── engine.ts           (UPDATE: add resolveEndOfRound wrapper)
└── components/
    └── dev/
        └── DevPanel.tsx    (UPDATE: add resolve button)
```

---

## Design Decisions

### Why Functional Approach for Main Function?

**Decision:** Use `&State -> Result<State>` pattern

**Rationale:**
- Consistent with `apply_action` from Sprint 01C
- Allows for error handling and validation
- Easier to test (compare before/after)
- Safe for future parallel evaluation

**Trade-off:** More cloning, but worthwhile for clarity

---

### Why Randomness for Factory Refill?

**Decision:** Use random tile draw from bag

**Rationale:**
- Matches real Azul gameplay (tiles drawn randomly)
- Provides variety in scenarios
- Essential for scenario generation (Sprint 04)

**Determinism:** Use seeded RNG for reproducible tests

---

### Why Not End-of-Game Bonuses?

**Decision:** Defer end-of-game bonuses to future sprint

**Rationale:**
- MVP focus: core gameplay loop complete without them
- Bonuses are only scored once at game end
- Can be added later without affecting other systems
- Sprint 03 already has significant scope

---

## Related Documentation

- [Sprint 03A: Wall Tiling](Sprint_03A_Wall_Tiling_Pattern_Lines.md) - Phase 1 of end-of-round
- [Sprint 03B: Scoring System](Sprint_03B_Scoring_System.md) - Phase 1 of end-of-round
- [Sprint 03: Main Document](Sprint_03_End_of_Round_Scoring_Refill.md)
- [Sprint 01C: Apply Action](Sprint_01C_Apply_Action_Transitions.md) - Similar functional pattern
- [Rules Specification](../specs/03_rules_and_edge_cases.md) - Complete round structure

---

## Next Steps

After completing Sprint 03C:
- Sprint 03 is complete! All end-of-round logic implemented
- Can proceed to [Sprint 04: Scenario Generation](Sprint_04_Scenario_Generation_Phases_Filters.md)
- Full game loop is now functional (draft → end-of-round → refill)
- Foundation ready for AI evaluation (Sprint 05)
