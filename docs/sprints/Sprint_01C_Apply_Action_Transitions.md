# Sprint 01C — Action Application & State Transitions

**Status:** Draft  
**Prerequisites:** Sprint 01A (Data Models) and Sprint 01B (Legality) complete  
**Dependencies:** State, Action types, and legality checks  
**Estimated Complexity:** High

---

## Goal

Implement `apply_action` function that correctly mutates game state based on a draft action, including tile movement, overflow handling, first-player token transfer, and invariant validation.

## Outcomes

- ✓ `apply_action(state, action)` function working correctly
- ✓ Tile movement logic correct (source → destination)
- ✓ Overflow handling implemented (pattern line → floor)
- ✓ First-player token transfer working
- ✓ Active player toggle working
- ✓ Tile conservation invariant holds
- ✓ Structured error types with helpful messages

---

## State Transition Logic

### Function Signature

```rust
pub fn apply_action(
    state: &State, 
    action: &DraftAction
) -> Result<State, ValidationError>
```

**Approach:** Functional (returns new state, doesn't mutate input)

**Steps:**
1. Validate action is legal (reuse legality checks from Sprint 01B)
2. Clone state for mutation
3. Count tiles being taken
4. Remove tiles from source
5. Move factory remnants to center (if source was factory)
6. Handle first-player token transfer
7. Place tiles in destination (with overflow)
8. Update active player
9. Verify invariants (tile conservation)
10. Return new state

---

## Detailed Implementation Steps

### Step 1: Validate Action Legality

```rust
pub fn apply_action(state: &State, action: &DraftAction) -> Result<State, ValidationError> {
    // First, validate the action is legal
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
    let mut new_state = state.clone();
    // ... continue with steps below
}
```

---

### Step 2-3: Count and Remove Tiles from Source

```rust
// Count tiles being taken
let tile_count = match &action.source {
    ActionSource::Factory(idx) => {
        state.factories[*idx].get(&action.color).copied().unwrap_or(0)
    }
    ActionSource::Center => {
        state.center.tiles.get(&action.color).copied().unwrap_or(0)
    }
};

// Remove tiles from source
match &action.source {
    ActionSource::Factory(idx) => {
        new_state.factories[*idx].remove(&action.color);
    }
    ActionSource::Center => {
        new_state.center.tiles.remove(&action.color);
    }
}
```

---

### Step 4: Move Factory Remnants to Center

```rust
// If taking from a factory, move remaining tiles to center
if let ActionSource::Factory(idx) = &action.source {
    // Get all remaining tiles from factory
    for (color, count) in new_state.factories[*idx].iter() {
        *new_state.center.tiles.entry(*color).or_insert(0) += count;
    }
    
    // Clear the factory
    new_state.factories[*idx].clear();
}
```

---

### Step 5: Handle First-Player Token

```rust
// If taking from center and first-player token is present
if action.source == ActionSource::Center && new_state.center.has_first_player_token {
    new_state.center.has_first_player_token = false;
    
    let player = &mut new_state.players[new_state.active_player_id as usize];
    player.floor_line.has_first_player_token = true;
}
```

---

### Step 6: Place Tiles in Destination (with Overflow)

```rust
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
```

---

### Step 7: Update Active Player

```rust
// Toggle active player (0 <-> 1)
new_state.active_player_id = 1 - new_state.active_player_id;
```

---

### Step 8: Verify Invariants

```rust
// Check tile conservation
if let Err(e) = check_tile_conservation(&new_state) {
    return Err(ValidationError::invariant_violation(e));
}

Ok(new_state)
```

---

## Overflow Behavior Examples

### Example 1: Exact Fit

**Before:**
- Pattern line row 2 (capacity 3) has 1 Blue tile
- Player takes 2 Blue tiles

**After:**
- Pattern line row 2 has 3 Blue tiles (complete)
- Floor line unchanged

```rust
#[test]
fn test_exact_fit() {
    let mut state = create_test_state();
    state.players[0].pattern_lines[2] = PatternLine {
        capacity: 3,
        color: Some(TileColor::Blue),
        count_filled: 1,
    };
    
    let action = DraftAction {
        source: ActionSource::Factory(0),
        color: TileColor::Blue,
        destination: Destination::PatternLine(2),
    };
    
    let new_state = apply_action(&state, &action).unwrap();
    
    assert_eq!(new_state.players[0].pattern_lines[2].count_filled, 3);
    assert_eq!(new_state.players[0].floor_line.tiles.len(), 0);
}
```

---

### Example 2: Overflow to Floor

**Before:**
- Pattern line row 1 (capacity 2) has 1 Red tile
- Player takes 3 Red tiles

**After:**
- Pattern line row 1 has 2 Red tiles (complete)
- Floor line gains 1 Red tile

```rust
#[test]
fn test_overflow_to_floor() {
    let mut state = create_test_state();
    state.players[0].pattern_lines[1] = PatternLine {
        capacity: 2,
        color: Some(TileColor::Red),
        count_filled: 1,
    };
    state.factories[0].insert(TileColor::Red, 3);
    
    let action = DraftAction {
        source: ActionSource::Factory(0),
        color: TileColor::Red,
        destination: Destination::PatternLine(1),
    };
    
    let new_state = apply_action(&state, &action).unwrap();
    
    assert_eq!(new_state.players[0].pattern_lines[1].count_filled, 2);
    assert_eq!(new_state.players[0].floor_line.tiles.len(), 1);
    assert_eq!(new_state.players[0].floor_line.tiles[0], TileColor::Red);
}
```

---

### Example 3: Empty Pattern Line with Overflow

**Before:**
- Pattern line row 0 (capacity 1) is empty
- Player takes 4 Yellow tiles

**After:**
- Pattern line row 0 has 1 Yellow tile (complete)
- Floor line gains 3 Yellow tiles

```rust
#[test]
fn test_empty_pattern_line_with_overflow() {
    let mut state = create_test_state();
    state.factories[0].insert(TileColor::Yellow, 4);
    
    let action = DraftAction {
        source: ActionSource::Factory(0),
        color: TileColor::Yellow,
        destination: Destination::PatternLine(0),
    };
    
    let new_state = apply_action(&state, &action).unwrap();
    
    assert_eq!(new_state.players[0].pattern_lines[0].count_filled, 1);
    assert_eq!(new_state.players[0].pattern_lines[0].color, Some(TileColor::Yellow));
    assert_eq!(new_state.players[0].floor_line.tiles.len(), 3);
}
```

---

### Example 4: All to Floor

**Before:**
- Player chooses Floor destination
- Player takes 3 Black tiles

**After:**
- Floor line gains 3 Black tiles
- Pattern lines unchanged

```rust
#[test]
fn test_all_to_floor() {
    let mut state = create_test_state();
    state.factories[0].insert(TileColor::Black, 3);
    
    let action = DraftAction {
        source: ActionSource::Factory(0),
        color: TileColor::Black,
        destination: Destination::Floor,
    };
    
    let new_state = apply_action(&state, &action).unwrap();
    
    assert_eq!(new_state.players[0].floor_line.tiles.len(), 3);
    assert!(new_state.players[0].floor_line.tiles.iter().all(|c| *c == TileColor::Black));
}
```

---

## Floor Line Behavior

- Floor line has 7 "slots" for penalty calculation (Sprint 03)
- `tiles` vector is NOT capped at 7 - all tiles are tracked
- Penalty scoring uses: `min(floor_line.tiles.len(), 7)`
- First-player token occupies "slot 0" for penalties

**Example:**
- Player has 5 tiles in floor line
- Player receives first-player token
- Penalty = FLOOR_PENALTIES[0] + sum(FLOOR_PENALTIES[1..6])
- = -1 + (-1 -2 -2 -2 -3) = -11

---

## Error Model

### ValidationError Struct

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ValidationError {
    pub code: String,
    pub message: String,
    pub context: Option<serde_json::Value>,
}
```

### Error Constructors

```rust
impl ValidationError {
    pub fn invalid_player(player_id: u8) -> Self {
        Self {
            code: "INVALID_PLAYER".to_string(),
            message: format!("Player ID {} is out of range", player_id),
            context: Some(json!({"player_id": player_id})),
        }
    }
    
    pub fn invalid_source(factory_idx: usize) -> Self {
        Self {
            code: "INVALID_SOURCE".to_string(),
            message: format!("Factory index {} is out of bounds", factory_idx),
            context: Some(json!({"factory_index": factory_idx})),
        }
    }
    
    pub fn source_empty(source: ActionSource, color: TileColor) -> Self {
        Self {
            code: "SOURCE_EMPTY".to_string(),
            message: format!("Source {:?} does not contain {:?} tiles", source, color),
            context: Some(json!({"source": source, "color": color})),
        }
    }
    
    pub fn color_mismatch(row: usize, existing: TileColor, attempted: TileColor) -> Self {
        Self {
            code: "COLOR_MISMATCH".to_string(),
            message: format!(
                "Cannot place {:?} tiles into pattern line {} which contains {:?}",
                attempted, row, existing
            ),
            context: Some(json!({
                "row": row,
                "existing_color": existing,
                "attempted_color": attempted
            })),
        }
    }
    
    pub fn wall_conflict(row: usize, color: TileColor) -> Self {
        Self {
            code: "WALL_CONFLICT".to_string(),
            message: format!(
                "Color {:?} already exists in wall row {}",
                color, row
            ),
            context: Some(json!({"row": row, "color": color})),
        }
    }
    
    pub fn pattern_line_complete(row: usize) -> Self {
        Self {
            code: "PATTERN_LINE_COMPLETE".to_string(),
            message: format!("Pattern line {} is already complete", row),
            context: Some(json!({"row": row})),
        }
    }
    
    pub fn invalid_destination(row: usize) -> Self {
        Self {
            code: "INVALID_DESTINATION".to_string(),
            message: format!("Pattern line row {} is out of bounds", row),
            context: Some(json!({"row": row})),
        }
    }
    
    pub fn invariant_violation(message: String) -> Self {
        Self {
            code: "INVARIANT_VIOLATION".to_string(),
            message,
            context: None,
        }
    }
}
```

---

## Tile Conservation Invariant

### Check Function

```rust
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
```

### When to Check

- **In tests:** After every `apply_action` call
- **In debug builds:** As assertion after state mutation
- **In production (optional):** Runtime check with panic or error return

```rust
#[cfg(debug_assertions)]
{
    check_tile_conservation(&new_state)
        .expect("Tile conservation invariant violated");
}
```

---

## State Transition Examples

### Example 1: Simple Factory Take

**Before:**
```json
{
  "factories": [
    {"Red": 2, "Blue": 1, "Yellow": 1}
  ],
  "center": {"tiles": {"Black": 1}, "has_first_player_token": false},
  "active_player_id": 0,
  "players": [
    {"pattern_lines": [{"capacity": 3, "color": null, "count_filled": 0}]}
  ]
}
```

**Action:**
```json
{
  "source": {"Factory": 0},
  "color": "Red",
  "destination": {"PatternLine": 2}
}
```

**After:**
```json
{
  "factories": [{}],
  "center": {
    "tiles": {"Black": 1, "Blue": 1, "Yellow": 1},
    "has_first_player_token": false
  },
  "active_player_id": 1,
  "players": [
    {
      "pattern_lines": [
        {"capacity": 3, "color": "Red", "count_filled": 2}
      ]
    }
  ]
}
```

**Changes:**
- Removed 2 Red tiles from Factory 0
- Moved Blue and Yellow to center
- Placed 2 Red tiles in pattern line 2
- Active player changed from 0 to 1

---

### Example 2: Center Take with First-Player Token

**Before:**
```json
{
  "center": {
    "tiles": {"White": 2, "Black": 1},
    "has_first_player_token": true
  },
  "active_player_id": 1,
  "players": [
    {},
    {"floor_line": {"tiles": [], "has_first_player_token": false}}
  ]
}
```

**Action:**
```json
{
  "source": "Center",
  "color": "White",
  "destination": {"PatternLine": 4}
}
```

**After:**
```json
{
  "center": {
    "tiles": {"Black": 1},
    "has_first_player_token": false
  },
  "active_player_id": 0,
  "players": [
    {},
    {
      "pattern_lines": [
        {"capacity": 5, "color": "White", "count_filled": 2}
      ],
      "floor_line": {
        "tiles": [],
        "has_first_player_token": true
      }
    }
  ]
}
```

**Changes:**
- Removed 2 White tiles from center
- Transferred first-player token to player 1's floor line
- Placed 2 White tiles in pattern line 4
- Active player changed from 1 to 0

---

### Example 3: Overflow Scenario

**Before:**
```json
{
  "factories": [
    {},
    {"Blue": 4}
  ],
  "players": [
    {
      "pattern_lines": [
        {},
        {"capacity": 2, "color": "Blue", "count_filled": 1}
      ],
      "floor_line": {"tiles": [], "has_first_player_token": false}
    }
  ]
}
```

**Action:**
```json
{
  "source": {"Factory": 1},
  "color": "Blue",
  "destination": {"PatternLine": 1}
}
```

**After:**
```json
{
  "factories": [{}, {}],
  "center": {"tiles": {}, "has_first_player_token": false},
  "players": [
    {
      "pattern_lines": [
        {},
        {"capacity": 2, "color": "Blue", "count_filled": 2}
      ],
      "floor_line": {
        "tiles": ["Blue", "Blue"],
        "has_first_player_token": false
      }
    }
  ]
}
```

**Changes:**
- Removed 4 Blue tiles from Factory 1
- Placed 1 Blue tile in pattern line 1 (completing it: 1 + 1 = 2)
- 2 Blue tiles overflowed to floor line

---

## Acceptance Criteria

- [ ] `apply_action` correctly mutates all affected state components
- [ ] Overflow from pattern lines to floor works correctly
- [ ] Factory remnants move to center when taking from factory
- [ ] First-player token transfers to player's floor line when taking from center
- [ ] Active player toggles correctly (0 ↔ 1)
- [ ] Tile conservation invariant passes for all test cases
- [ ] Illegal actions return appropriate `ValidationError`
- [ ] Error messages are clear and actionable
- [ ] Error context includes relevant details

---

## Testing Requirements

### Essential Tests

**Test 1: Simple action with no overflow**
```rust
#[test]
fn test_simple_action() {
    // Pattern line has room, no overflow
}
```

**Test 2: Action with overflow**
```rust
#[test]
fn test_action_with_overflow() {
    // More tiles than pattern line capacity
}
```

**Test 3: Factory remnants to center**
```rust
#[test]
fn test_factory_remnants() {
    // Remaining factory tiles move to center
}
```

**Test 4: First-player token transfer**
```rust
#[test]
fn test_first_player_token_transfer() {
    // Token moves from center to player floor line
}
```

**Test 5: Tile conservation**
```rust
#[test]
fn test_tile_conservation_holds() {
    // Apply multiple actions, verify total tiles constant
}
```

**Test 6: Error cases**
```rust
#[test]
fn test_illegal_action_errors() {
    // Test each error type with appropriate action
}
```

---

## Files to Create/Modify

```
rust/engine/src/
├── rules/
│   ├── mod.rs              (update to export apply, error, invariants)
│   ├── apply.rs            (NEW: apply_action implementation)
│   ├── error.rs            (NEW: ValidationError type)
│   └── invariants.rs       (NEW: check_tile_conservation)
```

---

## Related Documentation

- [Sprint 01A: Data Models](Sprint_01A_Data_Models_Serialization.md)
- [Sprint 01B: Legality Checks](Sprint_01B_Rules_Legality_Checks.md)
- [Sprint 01D: WASM Integration](Sprint_01D_WASM_Integration_Tests.md)
- [Rules Specification](../specs/03_rules_and_edge_cases.md)

---

## Next Steps

After completing Sprint 01C:
- Proceed to [Sprint 01D: WASM Integration & Tests](Sprint_01D_WASM_Integration_Tests.md)
- The complete engine functions will be wrapped for WASM boundary
