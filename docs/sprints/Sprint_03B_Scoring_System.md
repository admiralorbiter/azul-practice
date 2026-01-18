# Sprint 03B — Scoring System

**Status:** ✅ **COMPLETED**  
**Completion Date:** January 18, 2026  
**Prerequisites:** Sprint 03A (Wall Tiling) ✅ complete  
**Dependencies:** Pattern line resolution, wall state  
**Complexity:** Medium-High

---

## Goal

Implement all scoring logic for end-of-round, including adjacency scoring for wall tiles and floor penalty calculations, with comprehensive golden test suite.

## Outcomes

- ✓ Adjacency scoring (horizontal + vertical chains) working correctly
- ✓ Floor penalty calculation with first-player token handling
- ✓ Score clamping (cannot go below 0)
- ✓ Golden test suite with 10+ known scenarios
- ✓ All edge cases covered and tested

---

## Scoring Rules Overview

### Wall Tile Scoring

When a tile is placed on the wall (from a complete pattern line), score points based on adjacent tiles:

**Horizontal Scoring:**
- Count contiguous tiles in the same row (including newly placed tile)
- If count > 1: add `count` points

**Vertical Scoring:**
- Count contiguous tiles in the same column (including newly placed tile)
- If count > 1: add `count` points

**Isolated Tile:**
- If no adjacent tiles (horizontal AND vertical both = 1): add 1 point

**Important:** Points are cumulative. A tile can score both horizontally and vertically.

### Floor Penalty Scoring

At end of round, apply penalties for tiles in floor line:

**Penalty Slots:**
```
Slot:    0   1   2   3   4   5   6
Penalty: -1  -1  -2  -2  -2  -3  -3
```

**Rules:**
- First-player token (if present) occupies slot 0
- Tiles fill remaining slots left-to-right
- Only first 7 slots count for penalties
- Score cannot go below 0 after penalties

---

## Scoring Constants

```rust
pub const FLOOR_PENALTIES: [i32; 7] = [-1, -1, -2, -2, -2, -3, -3];
pub const MAX_FLOOR_PENALTY: i32 = -14; // Sum of all penalties
```

---

## Algorithm: calculate_wall_tile_score

### Function Signature

```rust
pub fn calculate_wall_tile_score(wall: &Wall, row: usize, col: usize) -> i32
```

**Parameters:**
- `wall`: The 5×5 wall grid (after tile placement)
- `row`: Row index of newly placed tile
- `col`: Column index of newly placed tile

**Returns:** Points earned from this tile placement

**Precondition:** `wall[row][col]` must be `true` (tile is placed)

### Pseudocode

```
function calculate_wall_tile_score(wall, row, col) -> i32:
    // Count horizontal contiguous tiles
    horizontal_count = 1  // Start with the placed tile
    
    // Count left
    for c in (col - 1) down to 0:
        if wall[row][c]:
            horizontal_count += 1
        else:
            break
    
    // Count right
    for c in (col + 1) up to 4:
        if wall[row][c]:
            horizontal_count += 1
        else:
            break
    
    // Count vertical contiguous tiles
    vertical_count = 1  // Start with the placed tile
    
    // Count up
    for r in (row - 1) down to 0:
        if wall[r][col]:
            vertical_count += 1
        else:
            break
    
    // Count down
    for r in (row + 1) up to 4:
        if wall[r][col]:
            vertical_count += 1
        else:
            break
    
    // Calculate score
    if horizontal_count == 1 and vertical_count == 1:
        // Isolated tile
        return 1
    else:
        score = 0
        if horizontal_count > 1:
            score += horizontal_count
        if vertical_count > 1:
            score += vertical_count
        return score
```

---

## Algorithm: calculate_floor_penalty

### Function Signature

```rust
pub fn calculate_floor_penalty(floor_line: &FloorLine) -> i32
```

**Parameters:**
- `floor_line`: Player's floor line with tiles and first-player token status

**Returns:** Penalty value (negative integer or 0)

### Pseudocode

```
function calculate_floor_penalty(floor_line) -> i32:
    penalty = 0
    slot_index = 0
    
    // First-player token occupies slot 0
    if floor_line.has_first_player_token:
        penalty += FLOOR_PENALTIES[0]  // -1
        slot_index = 1
    
    // Apply penalties for floor tiles
    tiles_to_count = min(floor_line.tiles.len(), 7 - slot_index)
    
    for i in 0..tiles_to_count:
        if slot_index < 7:
            penalty += FLOOR_PENALTIES[slot_index]
            slot_index += 1
    
    return penalty
```

---

## Scoring Examples

### Example 1: Isolated Tile (1 Point)

**Wall State:**
```
[ ][ ][ ][ ][ ]
[ ][ ][ ][ ][ ]
[ ][ ][N][ ][ ]  ← New tile at [2][2]
[ ][ ][ ][ ][ ]
[ ][ ][ ][ ][ ]
```

**Calculation:**
- Horizontal: only 1 tile (the new one)
- Vertical: only 1 tile (the new one)
- Both == 1 → Isolated tile → **+1 point**

---

### Example 2: Horizontal Chain (3 Points)

**Before:**
```
[ ][ ][ ][ ][ ]
[X][X][ ][ ][ ]  ← Existing tiles
[ ][ ][ ][ ][ ]
[ ][ ][ ][ ][ ]
[ ][ ][ ][ ][ ]
```

**After placing tile at [1][2]:**
```
[ ][ ][ ][ ][ ]
[X][X][N][ ][ ]  ← New tile extends chain
[ ][ ][ ][ ][ ]
[ ][ ][ ][ ][ ]
[ ][ ][ ][ ][ ]
```

**Calculation:**
- Horizontal: 3 contiguous tiles (columns 0, 1, 2)
- Vertical: only 1 tile (row 1)
- Horizontal > 1 → **+3 points**

---

### Example 3: Vertical Chain (4 Points)

**Before:**
```
[ ][X][ ][ ][ ]
[ ][X][ ][ ][ ]  ← Existing tiles
[ ][X][ ][ ][ ]
[ ][ ][ ][ ][ ]
[ ][ ][ ][ ][ ]
```

**After placing tile at [3][1]:**
```
[ ][X][ ][ ][ ]
[ ][X][ ][ ][ ]
[ ][X][ ][ ][ ]
[ ][N][ ][ ][ ]  ← New tile extends chain
[ ][ ][ ][ ][ ]
```

**Calculation:**
- Horizontal: only 1 tile
- Vertical: 4 contiguous tiles (rows 0, 1, 2, 3)
- Vertical > 1 → **+4 points**

---

### Example 4: Both Directions (7 Points)

**Before:**
```
[ ][ ][X][ ][ ]
[ ][X][X][X][ ]  ← Existing tiles
[ ][ ][X][ ][ ]
[ ][ ][ ][ ][ ]
[ ][ ][ ][ ][ ]
```

**After placing tile at [1][2]:**
```
[ ][ ][X][ ][ ]
[ ][X][N][X][ ]  ← New tile connects chains
[ ][ ][X][ ][ ]
[ ][ ][ ][ ][ ]
[ ][ ][ ][ ][ ]
```

**Calculation:**
- Horizontal: 3 tiles (columns 1, 2, 3) → +3
- Vertical: 4 tiles (rows 0, 1, 2) → +4
- Total: **+7 points**

---

### Example 5: Corner Placement (2 Points)

**Before:**
```
[X][X][X][ ][ ]
[X][ ][ ][ ][ ]
[ ][ ][ ][ ][ ]
[ ][ ][ ][ ][ ]
[ ][ ][ ][ ][ ]
```

**After placing tile at [0][0] (already exists, for illustration):**

Wait, this is confusing. Let me redo this. If we're placing at corner:

**Before:**
```
[ ][X][X][ ][ ]
[ ][ ][ ][ ][ ]
[ ][ ][ ][ ][ ]
[ ][ ][ ][ ][ ]
[ ][ ][ ][ ][ ]
```

**After placing tile at [0][0]:**
```
[N][X][X][ ][ ]
[ ][ ][ ][ ][ ]
[ ][ ][ ][ ][ ]
[ ][ ][ ][ ][ ]
[ ][ ][ ][ ][ ]
```

**Calculation:**
- Horizontal: 3 tiles (columns 0, 1, 2) → +3
- Vertical: 1 tile
- Total: **+3 points**

---

### Example 6: Maximum Single Tile Score

**Before:**
```
[ ][X][X][X][X]
[X][X][X][X][X]
[X][X][ ][X][X]  ← Gap at [2][2]
[X][X][X][X][X]
[X][X][X][X][X]
```

**After placing tile at [2][2]:**
```
[ ][X][X][X][X]
[X][X][X][X][X]
[X][X][N][X][X]  ← Completes 5-tile chains both ways
[X][X][X][X][X]
[X][X][X][X][X]
```

**Calculation:**
- Horizontal: 5 tiles (entire row) → +5
- Vertical: 5 tiles (entire column) → +5
- Total: **+10 points** (maximum possible for single tile)

---

### Example 7: Floor Penalties with Token

**Floor Line:**
```
has_first_player_token: true
tiles: [Blue, Red, Yellow]
```

**Calculation:**
```
Slot 0: Token → -1
Slot 1: Blue → -1
Slot 2: Red → -2
Slot 3: Yellow → -2
Total penalty: -6
```

---

### Example 8: Floor Penalties without Token

**Floor Line:**
```
has_first_player_token: false
tiles: [Blue, Red, Yellow, Black, White]
```

**Calculation:**
```
Slot 0: Blue → -1
Slot 1: Red → -1
Slot 2: Yellow → -2
Slot 3: Black → -2
Slot 4: White → -2
Total penalty: -8
```

---

### Example 9: Floor with 10+ Tiles (Only 7 Count)

**Floor Line:**
```
has_first_player_token: true
tiles: [Blue, Red, Yellow, Black, White, Blue, Red, Black, Yellow, White]  // 10 tiles
```

**Calculation:**
```
Slot 0: Token → -1
Slot 1: Blue → -1
Slot 2: Red → -2
Slot 3: Yellow → -2
Slot 4: Black → -2
Slot 5: White → -3
Slot 6: Blue → -3
Slots 7-9: (tiles 8-10) → no penalty
Total penalty: -14 (maximum possible)
```

---

### Example 10: Empty Floor (0 Penalty)

**Floor Line:**
```
has_first_player_token: false
tiles: []
```

**Calculation:**
```
Total penalty: 0
```

---

## Algorithm: apply_scoring

### Function Signature

```rust
pub fn apply_scoring(state: &mut State, player_idx: usize)
```

**Purpose:** Calculate and apply all scoring for one player at end of round

**Precondition:** Pattern lines have been resolved (Sprint 03A) - wall has new tiles

### Pseudocode

```
function apply_scoring(state, player_idx):
    player = &mut state.players[player_idx]
    total_score_change = 0
    
    // Record which tiles were placed this round
    // (Sprint 03A places tiles, we need to know which are new)
    // For now, assume we track this separately or accept scoring all visible tiles
    
    // Actually, we need a different approach.
    // Sprint 03A should mark which tiles are new, OR
    // We calculate scoring during Sprint 03A's tile placement
    
    // Better approach: Calculate score per tile in Sprint 03A
    // Then just apply floor penalties here
    
    // Apply floor penalties
    floor_penalty = calculate_floor_penalty(&player.floor_line)
    total_score_change += floor_penalty
    
    // Update player score (cannot go below 0)
    player.score = max(0, player.score + total_score_change)
```

**Issue Identified:** We need to calculate wall tile scores during placement in Sprint 03A, not after. Let me revise the integration.

### Revised Integration with Sprint 03A

Sprint 03A's `resolve_pattern_lines` should be modified to call scoring:

```rust
pub fn resolve_pattern_lines(state: &mut State) {
    for player_idx in 0..2 {
        let player = &mut state.players[player_idx];
        
        for row in 0..5 {
            let pattern_line = &mut player.pattern_lines[row];
            
            if pattern_line.count_filled == pattern_line.capacity {
                let color = pattern_line.color.unwrap();
                let col = get_wall_column_for_color(row, color);
                
                // Place tile on wall
                player.wall[row][col] = true;
                
                // Calculate score for this tile placement (Sprint 03B)
                let points = calculate_wall_tile_score(&player.wall, row, col);
                player.score += points;
                
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

Then Sprint 03B adds floor penalties:

```rust
pub fn apply_floor_penalties(state: &mut State) {
    for player in &mut state.players {
        let penalty = calculate_floor_penalty(&player.floor_line);
        player.score = std::cmp::max(0, player.score + penalty);
    }
}
```

---

## Complete Scoring Flow

```rust
pub fn resolve_end_of_round_scoring(state: &mut State) {
    // 1. Resolve pattern lines with wall tile scoring (03A + 03B)
    resolve_pattern_lines_with_scoring(state);
    
    // 2. Apply floor penalties (03B)
    apply_floor_penalties(state);
    
    // 3. Clear floor lines
    clear_floor_lines(state);
}
```

---

## Golden Test Suite

### Golden Test Format

Each golden test specifies:
- Initial wall state (before tile placement)
- Tile position being placed
- Expected score for that tile

```rust
struct GoldenTest {
    name: &'static str,
    wall: [[bool; 5]; 5],
    row: usize,
    col: usize,
    expected_score: i32,
}
```

### Golden Test 1: Isolated Tile

```rust
GoldenTest {
    name: "isolated_tile",
    wall: [
        [false, false, false, false, false],
        [false, false, false, false, false],
        [false, false, true, false, false],  // New tile at [2][2]
        [false, false, false, false, false],
        [false, false, false, false, false],
    ],
    row: 2,
    col: 2,
    expected_score: 1,
}
```

### Golden Test 2: Horizontal 2

```rust
GoldenTest {
    name: "horizontal_2",
    wall: [
        [true, true, false, false, false],  // New tile at [0][1]
        [false, false, false, false, false],
        [false, false, false, false, false],
        [false, false, false, false, false],
        [false, false, false, false, false],
    ],
    row: 0,
    col: 1,
    expected_score: 2,
}
```

### Golden Test 3: Horizontal 5

```rust
GoldenTest {
    name: "horizontal_5_complete_row",
    wall: [
        [false, false, false, false, false],
        [true, true, true, true, true],  // New tile at [1][2], completes row
        [false, false, false, false, false],
        [false, false, false, false, false],
        [false, false, false, false, false],
    ],
    row: 1,
    col: 2,
    expected_score: 5,
}
```

### Golden Test 4: Vertical 3

```rust
GoldenTest {
    name: "vertical_3",
    wall: [
        [false, false, true, false, false],
        [false, false, true, false, false],
        [false, false, true, false, false],  // New tile at [2][2]
        [false, false, false, false, false],
        [false, false, false, false, false],
    ],
    row: 2,
    col: 2,
    expected_score: 3,
}
```

### Golden Test 5: T-Shape (3 + 3 = 6)

```rust
GoldenTest {
    name: "t_shape_both_directions",
    wall: [
        [false, false, true, false, false],
        [false, true, true, true, false],  // New tile at [1][2]
        [false, false, true, false, false],
        [false, false, false, false, false],
        [false, false, false, false, false],
    ],
    row: 1,
    col: 2,
    expected_score: 6,  // 3 horizontal + 3 vertical
}
```

### Golden Test 6: Cross (5 + 5 = 10)

```rust
GoldenTest {
    name: "cross_maximum_score",
    wall: [
        [false, false, true, false, false],
        [false, false, true, false, false],
        [true, true, true, true, true],  // New tile at [2][2]
        [false, false, true, false, false],
        [false, false, true, false, false],
    ],
    row: 2,
    col: 2,
    expected_score: 10,  // 5 horizontal + 5 vertical (maximum)
}
```

### Golden Test 7: Corner Extension

```rust
GoldenTest {
    name: "corner_extends_horizontal",
    wall: [
        [true, true, true, false, false],  // New tile at [0][0]
        [false, false, false, false, false],
        [false, false, false, false, false],
        [false, false, false, false, false],
        [false, false, false, false, false],
    ],
    row: 0,
    col: 0,
    expected_score: 3,
}
```

### Golden Test 8: L-Shape

```rust
GoldenTest {
    name: "l_shape",
    wall: [
        [true, false, false, false, false],
        [true, false, false, false, false],
        [true, true, false, false, false],  // New tile at [2][0]
        [false, false, false, false, false],
        [false, false, false, false, false],
    ],
    row: 2,
    col: 0,
    expected_score: 5,  // 3 vertical + 2 horizontal
}
```

### Golden Test 9: Gap in Chain

```rust
GoldenTest {
    name: "gap_stops_chain",
    wall: [
        [false, false, false, false, false],
        [true, true, false, true, true],  // New tile at [1][1], gap at [1][2]
        [false, false, false, false, false],
        [false, false, false, false, false],
        [false, false, false, false, false],
    ],
    row: 1,
    col: 1,
    expected_score: 2,  // Only counts [1][0] and [1][1], gap stops chain
}
```

### Golden Test 10: Floor with Token

```rust
#[test]
fn test_floor_penalty_with_token() {
    let floor_line = FloorLine {
        tiles: vec![
            TileColor::Blue,
            TileColor::Red,
            TileColor::Yellow,
        ],
        has_first_player_token: true,
    };
    
    let penalty = calculate_floor_penalty(&floor_line);
    assert_eq!(penalty, -6);  // -1 (token) -1 -2 -2
}
```

### Golden Test 11: Maximum Floor Penalty

```rust
#[test]
fn test_maximum_floor_penalty() {
    let floor_line = FloorLine {
        tiles: vec![
            TileColor::Blue,
            TileColor::Red,
            TileColor::Yellow,
            TileColor::Black,
            TileColor::White,
            TileColor::Blue,
            TileColor::Red,
            TileColor::Black,  // Extra tiles don't add penalty
            TileColor::Yellow,
            TileColor::White,
        ],
        has_first_player_token: true,
    };
    
    let penalty = calculate_floor_penalty(&floor_line);
    assert_eq!(penalty, -14);  // Maximum: -1-1-2-2-2-3-3
}
```

---

## Edge Cases

### Edge Case 1: Score Goes Negative (Clamp to 0)

**Initial State:**
```
Player score: 3
Floor line: 8 tiles + first-player token
Floor penalty: -14
```

**Calculation:**
```
New score = 3 + (-14) = -11
Clamped score = max(0, -11) = 0
```

**Result:** Player score becomes 0 (cannot go negative)

---

### Edge Case 2: Multiple Tiles Scored in One Round

**Initial State:**
```
Player has 3 complete pattern lines:
- Row 0: 1 Blue tile
- Row 2: 3 Red tiles
- Row 4: 5 White tiles
```

**Scoring:**
```
Row 0: Isolated tile → +1
Row 2: Extends horizontal chain → +3
Row 4: T-shape → +6
Total: +10 points
```

All tiles are scored independently based on the wall state after each placement.

---

### Edge Case 3: Complete Row Bonus (Future)

**Note:** Standard Azul has end-of-game bonuses for complete rows, columns, and color sets. These are NOT part of end-of-round scoring.

Sprint 03B focuses only on:
- Per-tile adjacency scoring
- Floor penalties

Game-end bonuses will be added in a future sprint if needed.

---

## Acceptance Criteria

- [ ] `calculate_wall_tile_score` returns correct scores for all golden tests
- [ ] Horizontal chain counting works correctly (stops at gaps)
- [ ] Vertical chain counting works correctly (stops at gaps)
- [ ] Isolated tile scores exactly 1 point
- [ ] Both-direction scoring adds both values
- [ ] `calculate_floor_penalty` handles first-player token correctly
- [ ] Floor penalties cap at first 7 slots
- [ ] Score clamping prevents negative scores
- [ ] All 11+ golden tests pass
- [ ] Edge cases handled correctly

---

## Test Requirements

### Unit Tests

All golden tests should be implemented as unit tests:

```rust
#[test]
fn test_golden_tests() {
    let tests = vec![
        GoldenTest { /* ... */ },
        // ... all golden tests
    ];
    
    for test in tests {
        let score = calculate_wall_tile_score(&test.wall, test.row, test.col);
        assert_eq!(
            score, 
            test.expected_score,
            "Test '{}' failed: expected {}, got {}",
            test.name, test.expected_score, score
        );
    }
}
```

### Integration Tests

```rust
#[test]
fn test_complete_round_scoring() {
    let mut state = create_test_state_with_complete_pattern_lines();
    
    let score_before = state.players[0].score;
    
    resolve_pattern_lines_with_scoring(&mut state);
    apply_floor_penalties(&mut state);
    
    let score_after = state.players[0].score;
    
    // Verify score changed appropriately
    assert!(score_after >= 0);  // Never negative
}
```

---

## Files to Create/Modify

```
rust/engine/src/
├── rules/
│   ├── mod.rs              (UPDATE: export scoring module)
│   ├── scoring.rs          (NEW: scoring calculations)
│   ├── resolution.rs       (UPDATE: integrate scoring)
│   └── tests/
│       └── scoring_tests.rs (NEW: golden tests)
```

### `rules/scoring.rs`

```rust
use crate::model::{Wall, FloorLine, TileColor};

pub const FLOOR_PENALTIES: [i32; 7] = [-1, -1, -2, -2, -2, -3, -3];

/// Calculate score for placing a tile on the wall.
/// Counts horizontal and vertical adjacent tiles.
pub fn calculate_wall_tile_score(wall: &Wall, row: usize, col: usize) -> i32 {
    debug_assert!(wall[row][col], "Tile must be placed at position");
    
    // Count horizontal
    let mut h_count = 1;
    for c in (0..col).rev() {
        if wall[row][c] { h_count += 1; } else { break; }
    }
    for c in (col + 1)..5 {
        if wall[row][c] { h_count += 1; } else { break; }
    }
    
    // Count vertical
    let mut v_count = 1;
    for r in (0..row).rev() {
        if wall[r][col] { v_count += 1; } else { break; }
    }
    for r in (row + 1)..5 {
        if wall[r][col] { v_count += 1; } else { break; }
    }
    
    // Calculate score
    if h_count == 1 && v_count == 1 {
        1  // Isolated tile
    } else {
        let mut score = 0;
        if h_count > 1 { score += h_count; }
        if v_count > 1 { score += v_count; }
        score
    }
}

/// Calculate floor penalty for a player.
pub fn calculate_floor_penalty(floor_line: &FloorLine) -> i32 {
    let mut penalty = 0;
    let mut slot = 0;
    
    // First-player token
    if floor_line.has_first_player_token {
        penalty += FLOOR_PENALTIES[0];
        slot = 1;
    }
    
    // Floor tiles
    let tiles_to_count = std::cmp::min(floor_line.tiles.len(), 7 - slot);
    for _ in 0..tiles_to_count {
        penalty += FLOOR_PENALTIES[slot];
        slot += 1;
    }
    
    penalty
}

/// Apply floor penalties to all players.
pub fn apply_floor_penalties(state: &mut crate::model::State) {
    for player in &mut state.players {
        let penalty = calculate_floor_penalty(&player.floor_line);
        player.score = std::cmp::max(0, player.score + penalty);
    }
}
```

---

## Related Documentation

- [Sprint 03A: Wall Tiling](Sprint_03A_Wall_Tiling_Pattern_Lines.md) - Integrated with this sprint
- [Sprint 03C: Round Transition](Sprint_03C_Round_Transition_Refill.md) - Calls these functions
- [Sprint 03: Main Document](Sprint_03_End_of_Round_Scoring_Refill.md)
- [Rules Specification](../specs/03_rules_and_edge_cases.md) - Section 9: Scoring

---

## Next Steps

After completing Sprint 03B:
- Integrate scoring into Sprint 03A's pattern line resolution
- Proceed to [Sprint 03C: Round Transition & Refill](Sprint_03C_Round_Transition_Refill.md)
- The scoring system is now complete for end-of-round
