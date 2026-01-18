# Sprint 01A — Data Models & Serialization

**Status:** Draft  
**Prerequisites:** Sprint 0 (WASM Pipeline) complete  
**Dependencies:** None (foundational)  
**Estimated Complexity:** Medium

---

## Goal

Define stable, versioned Rust types for `State`, `Action`, and related structures with JSON serialization support.

## Outcomes

- ✓ Rust structs for State, Action, PlayerBoard, and all sub-components
- ✓ JSON serialization/deserialization with serde
- ✓ Version fields properly tracked in all serialized output
- ✓ Round-trip serialization tests pass (state → JSON → state preserves all data)

---

## Data Model Specifications

### Core Types

#### TileColor Enum

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TileColor {
    Blue,
    Yellow,
    Red,
    Black,
    White,
}
```

**Serialization:** As strings (`"Blue"`, `"Yellow"`, etc.)

**Total colors:** 5  
**Tiles per color:** 20  
**Total tiles in game:** 100

---

#### State Struct

The complete game state for a 2-player Azul game during the draft phase.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct State {
    // Metadata
    pub state_version: u32,
    pub ruleset_id: String,
    pub scenario_seed: Option<String>,
    pub active_player_id: u8,
    pub round_number: u8,
    pub draft_phase_progress: DraftPhase,
    
    // Supply
    pub bag: TileMultiset,
    pub lid: TileMultiset,
    
    // Table
    pub factories: Vec<TileMultiset>,
    pub center: CenterArea,
    
    // Players
    pub players: [PlayerBoard; 2],
}
```

**Field descriptions:**

- `state_version`: Schema version for compatibility (start with `1`)
- `ruleset_id`: E.g., `"azul_v1_2p"` for 2-player standard Azul
- `scenario_seed`: Optional seed for reproducibility (used in scenario generation)
- `active_player_id`: Current player (0 or 1)
- `round_number`: Current round (starts at 1)
- `draft_phase_progress`: Enum indicating `EARLY`, `MID`, or `LATE` draft phase

---

#### DraftPhase Enum

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DraftPhase {
    Early,
    Mid,
    Late,
}
```

Used for scenario generation filters (see Sprint 04).

---

#### TileMultiset Type

Represents a collection of tiles by color.

**Recommendation:** Use `HashMap<TileColor, u8>`

```rust
pub type TileMultiset = std::collections::HashMap<TileColor, u8>;
```

**Rationale:**
- Sparse representation (only store colors present)
- Easy to add/remove tiles
- Clear intent in code
- Alternative: `[u8; 5]` array indexed by color ordinal (more compact but less readable)

**JSON representation:**
```json
{
  "Blue": 3,
  "Red": 2,
  "Yellow": 1
}
```

---

#### CenterArea Struct

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CenterArea {
    pub tiles: TileMultiset,
    pub has_first_player_token: bool,
}
```

The center area accumulates tiles from factories and tracks the first-player token.

---

#### PlayerBoard Struct

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PlayerBoard {
    pub score: i32,
    pub pattern_lines: [PatternLine; 5],
    pub wall: Wall,
    pub floor_line: FloorLine,
}
```

---

#### PatternLine Struct

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PatternLine {
    pub capacity: u8,
    pub color: Option<TileColor>,
    pub count_filled: u8,
}
```

**Invariants:**
- `capacity` values: 1, 2, 3, 4, 5 for rows 0-4 respectively
- `count_filled` ≤ `capacity`
- If `count_filled > 0`, then `color` must be `Some(_)`
- If `count_filled == 0`, then `color` must be `None`

**Examples:**
- Empty row 0: `{ capacity: 1, color: None, count_filled: 0 }`
- Row 2 with 2 Red tiles: `{ capacity: 3, color: Some(Red), count_filled: 2 }`
- Row 4 full with Blue: `{ capacity: 5, color: Some(Blue), count_filled: 5 }`

---

#### Wall Type

The wall is a 5x5 grid where each cell can be filled (true) or empty (false).

**Recommendation:** Use `[[bool; 5]; 5]`

```rust
pub type Wall = [[bool; 5]; 5];
```

**Standard Azul Wall Color Pattern:**

Each row has a fixed color order (rotated by 1 from previous row):

```
Row 0: [Blue,   Yellow, Red,    Black,  White]
Row 1: [White,  Blue,   Yellow, Red,    Black]
Row 2: [Black,  White,  Blue,   Yellow, Red]
Row 3: [Red,    Black,  White,  Blue,   Yellow]
Row 4: [Yellow, Red,    Black,  White,  Blue]
```

**Helper function needed:**
```rust
pub fn get_wall_color(row: usize, col: usize) -> TileColor {
    // Returns the color at wall position [row][col]
    // Based on standard Azul pattern above
}
```

**Rationale for bool array vs color array:**
- Wall colors are fixed by game rules (constant pattern)
- Bool array is simpler and smaller
- Color can be computed from position when needed
- Alternative: `[[Option<TileColor>; 5]; 5]` stores explicit colors (larger, more redundant)

---

#### FloorLine Struct

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FloorLine {
    pub tiles: Vec<TileColor>,
    pub has_first_player_token: bool,
}
```

**Notes:**
- Floor line has 7 "slots" for penalty scoring
- `tiles` vector can exceed 7 (all tiles tracked for discard)
- Penalty values (for Sprint 03): [-1, -1, -2, -2, -2, -3, -3] for first 7 slots
- First-player token conceptually occupies "slot 0" for penalties

---

#### DraftAction Struct

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DraftAction {
    pub source: ActionSource,
    pub color: TileColor,
    pub destination: Destination,
}
```

---

#### ActionSource Enum

```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ActionSource {
    Factory(usize),  // factory index (0-4 for 2-player)
    Center,
}
```

**JSON representation:**
- Factory: `{"Factory": 2}`
- Center: `"Center"`

---

#### Destination Enum

```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum Destination {
    PatternLine(usize),  // row index (0-4)
    Floor,
}
```

**JSON representation:**
- Pattern line: `{"PatternLine": 3}`
- Floor: `"Floor"`

---

## JSON Serialization Requirements

### Field Naming Convention

Use `snake_case` for all JSON field names via:
```rust
#[serde(rename_all = "snake_case")]
```

**Examples:**
- Rust: `active_player_id` → JSON: `"active_player_id"`
- Rust: `has_first_player_token` → JSON: `"has_first_player_token"`

### Round-Trip Stability

The following must hold:
```rust
let state = create_test_state();
let json = serde_json::to_string(&state).unwrap();
let restored: State = serde_json::from_str(&json).unwrap();
assert_eq!(state, restored);
```

All fields must serialize and deserialize without data loss.

### Version Fields

- `state_version` must always appear in JSON
- `ruleset_id` must always appear in JSON
- These enable future compatibility and validation

---

## Example JSON Output

Complete mid-game state example:

```json
{
  "state_version": 1,
  "ruleset_id": "azul_v1_2p",
  "scenario_seed": "test_scenario_001",
  "active_player_id": 0,
  "round_number": 2,
  "draft_phase_progress": "MID",
  "bag": {
    "Blue": 8,
    "Yellow": 10,
    "Red": 7,
    "Black": 9,
    "White": 11
  },
  "lid": {
    "Blue": 2,
    "Red": 3
  },
  "factories": [
    {
      "Blue": 2,
      "Red": 1,
      "Yellow": 1
    },
    {
      "Black": 3,
      "White": 1
    },
    {},
    {
      "Blue": 1,
      "Yellow": 3
    },
    {
      "Red": 2,
      "Black": 2
    }
  ],
  "center": {
    "tiles": {
      "White": 2,
      "Red": 1
    },
    "has_first_player_token": true
  },
  "players": [
    {
      "score": 12,
      "pattern_lines": [
        {
          "capacity": 1,
          "color": "Blue",
          "count_filled": 1
        },
        {
          "capacity": 2,
          "color": "Red",
          "count_filled": 2
        },
        {
          "capacity": 3,
          "color": null,
          "count_filled": 0
        },
        {
          "capacity": 4,
          "color": "Yellow",
          "count_filled": 3
        },
        {
          "capacity": 5,
          "color": null,
          "count_filled": 0
        }
      ],
      "wall": [
        [true, false, false, false, false],
        [false, false, true, false, false],
        [false, false, false, true, false],
        [false, false, false, false, false],
        [false, false, false, false, false]
      ],
      "floor_line": {
        "tiles": ["Black"],
        "has_first_player_token": false
      }
    },
    {
      "score": 15,
      "pattern_lines": [
        {
          "capacity": 1,
          "color": null,
          "count_filled": 0
        },
        {
          "capacity": 2,
          "color": null,
          "count_filled": 0
        },
        {
          "capacity": 3,
          "color": "Blue",
          "count_filled": 3
        },
        {
          "capacity": 4,
          "color": null,
          "count_filled": 0
        },
        {
          "capacity": 5,
          "color": "White",
          "count_filled": 4
        }
      ],
      "wall": [
        [false, true, false, false, false],
        [false, false, false, true, false],
        [false, false, false, false, true],
        [true, false, false, false, false],
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

---

## Acceptance Criteria

- [ ] All structs compile with serde derives (`Serialize`, `Deserialize`)
- [ ] Round-trip test passes: `state -> JSON -> state` preserves all data
- [ ] Field names in JSON use `snake_case` convention
- [ ] Version fields (`state_version`, `ruleset_id`) serialize correctly
- [ ] Optional fields (e.g., `scenario_seed`) handle `None` correctly
- [ ] JSON output is human-readable and properly formatted
- [ ] All public types have rustdoc comments

---

## Testing Requirements

### Essential Tests

**Test 1: Round-trip serialization**
```rust
#[test]
fn test_state_roundtrip() {
    let state = create_test_state();
    let json = serde_json::to_string_pretty(&state).unwrap();
    let restored: State = serde_json::from_str(&json).unwrap();
    assert_eq!(state, restored);
}
```

**Test 2: Optional field handling**
```rust
#[test]
fn test_state_without_scenario_seed() {
    let state = State {
        scenario_seed: None,
        // ... other fields
    };
    let json = serde_json::to_string(&state).unwrap();
    let restored: State = serde_json::from_str(&json).unwrap();
    assert_eq!(restored.scenario_seed, None);
}
```

**Test 3: Field naming convention**
```rust
#[test]
fn test_json_field_names() {
    let state = create_test_state();
    let json = serde_json::to_string(&state).unwrap();
    
    // Verify snake_case fields exist
    assert!(json.contains("\"active_player_id\""));
    assert!(json.contains("\"state_version\""));
    assert!(json.contains("\"draft_phase_progress\""));
}
```

**Test 4: Version fields present**
```rust
#[test]
fn test_version_fields() {
    let state = create_test_state();
    let json_value: serde_json::Value = serde_json::to_value(&state).unwrap();
    
    assert_eq!(json_value["state_version"], 1);
    assert_eq!(json_value["ruleset_id"], "azul_v1_2p");
}
```

---

## Files to Create

```
rust/engine/src/
├── model/
│   ├── mod.rs              (module exports)
│   ├── types.rs            (TileColor, DraftPhase enums)
│   ├── state.rs            (State, CenterArea structs)
│   ├── action.rs           (DraftAction, ActionSource, Destination)
│   ├── player.rs           (PlayerBoard, PatternLine, FloorLine, Wall)
│   └── tests.rs            (serialization tests)
```

### Module Organization

**`mod.rs`:**
```rust
mod types;
mod state;
mod action;
mod player;

#[cfg(test)]
mod tests;

pub use types::*;
pub use state::*;
pub use action::*;
pub use player::*;
```

---

## Design Decisions

### Why HashMap for TileMultiset?

**Pros:**
- Sparse representation (only store colors present)
- Clear semantics: `map.get(&color)` vs `array[color_index]`
- Flexible for adding/removing colors

**Cons:**
- Slightly larger in memory
- Hash overhead for lookups

**Decision:** Clarity and maintainability outweigh minor performance cost at this stage.

---

### Why bool array for Wall?

**Pros:**
- Compact representation
- Wall colors are fixed by game rules (no need to store)
- Simple to check: `wall[row][col]` returns filled status

**Cons:**
- Need helper function to get color from position

**Decision:** Simplicity and memory efficiency. Helper function is trivial.

---

### Why functional (clone) approach?

State transitions in Sprint 01C will use:
```rust
pub fn apply_action(state: &State, action: &DraftAction) -> Result<State, ValidationError>
```

**Pros:**
- Immutable functional approach
- Easy to test (no hidden mutations)
- Safe for concurrent evaluation (Sprint 05)
- Can compare before/after states

**Cons:**
- Cloning overhead

**Decision:** Correctness and testability first. Optimize later if profiling shows issues.

---

## Related Documentation

- [Sprint 01: Master Document](Sprint_01_Core_Engine_v0_State_Actions_Legality.md)
- [State/Action Model Spec](../specs/04_state_action_model_and_serialization.md)
- [Rules Specification](../specs/03_rules_and_edge_cases.md)

---

## Next Steps

After completing Sprint 01A:
- Proceed to [Sprint 01B: Rules & Legality Checks](Sprint_01B_Rules_Legality_Checks.md)
- The data models defined here will be used by all subsequent sprints
