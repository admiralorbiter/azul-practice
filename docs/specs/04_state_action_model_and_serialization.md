# State / Action Model & Serialization (v0)

## Goals
- Stable and versioned schema for:
  - Rust engine internals
  - WASM boundary
  - scenario packs
  - replay/debug dumps

## Versioning
- `state_version`: integer (current: `1`)
- `ruleset_id`: string (e.g., `"azul_v1_2p"`)

---

## Complete State Definition

### Game Constants (2-Player Azul)

```
Factories: 5
Tiles per factory (initial): 4
Total tiles per color: 20
Total tiles in game: 100 (5 colors × 20)
Pattern lines: 5 (capacities 1, 2, 3, 4, 5)
Wall: 5×5 grid
Floor line slots: 7
```

### State Structure

A complete game state for 2-player Azul:

```rust
pub struct State {
    // Metadata
    pub state_version: u32,              // Schema version (currently 1)
    pub ruleset_id: String,              // "azul_v1_2p"
    pub scenario_seed: Option<String>,   // For reproducibility
    pub active_player_id: u8,            // 0 or 1
    pub round_number: u8,                // Starts at 1
    pub draft_phase_progress: DraftPhase, // EARLY, MID, or LATE
    
    // Supply
    pub bag: TileMultiset,               // Tiles available to draw
    pub lid: TileMultiset,               // Discarded tiles
    
    // Table
    pub factories: Vec<TileMultiset>,    // 5 factories for 2-player
    pub center: CenterArea,              // Center + first-player token
    
    // Players
    pub players: [PlayerBoard; 2],       // Exactly 2 players
}
```

### Sub-Structures

**DraftPhase:**
```rust
pub enum DraftPhase {
    Early,  // Many tiles on table
    Mid,    // Moderate tiles remaining
    Late,   // Few tiles remaining
}
```

**TileColor:**
```rust
pub enum TileColor {
    Blue,
    Yellow,
    Red,
    Black,
    White,
}
```

**TileMultiset:**
```rust
pub type TileMultiset = HashMap<TileColor, u8>;
```

**CenterArea:**
```rust
pub struct CenterArea {
    pub tiles: TileMultiset,
    pub has_first_player_token: bool,
}
```

**PlayerBoard:**
```rust
pub struct PlayerBoard {
    pub score: i32,
    pub pattern_lines: [PatternLine; 5],
    pub wall: Wall,
    pub floor_line: FloorLine,
}
```

**PatternLine:**
```rust
pub struct PatternLine {
    pub capacity: u8,              // 1, 2, 3, 4, or 5
    pub color: Option<TileColor>,  // None if empty
    pub count_filled: u8,          // 0 to capacity
}
```

**Wall:**
```rust
pub type Wall = [[bool; 5]; 5];  // true = filled, false = empty
```

**Standard Azul Wall Pattern:**
```
     Col 0    Col 1    Col 2    Col 3    Col 4
Row 0: Blue   Yellow   Red      Black    White
Row 1: White  Blue     Yellow   Red      Black
Row 2: Black  White    Blue     Yellow   Red
Row 3: Red    Black    White    Blue     Yellow
Row 4: Yellow Red      Black    White    Blue
```

**FloorLine:**
```rust
pub struct FloorLine {
    pub tiles: Vec<TileColor>,       // All tiles on floor (not capped at 7)
    pub has_first_player_token: bool,
}
```

---

## Action Definition

### DraftAction Structure

```rust
pub struct DraftAction {
    pub source: ActionSource,
    pub color: TileColor,
    pub destination: Destination,
}

pub enum ActionSource {
    Factory(usize),  // Factory index (0-4 for 2-player)
    Center,
}

pub enum Destination {
    PatternLine(usize),  // Row index (0-4)
    Floor,
}
```

### Action Semantics

A `DraftAction` represents:
1. **Take** all tiles of `color` from `source`
2. **Place** them in `destination` (with overflow to floor if needed)
3. **Side effects:**
   - If source is Factory: remaining factory tiles move to center
   - If source is Center with first-player token: token moves to player's floor

---

## JSON Serialization

### Field Naming Convention

All JSON fields use `snake_case`:
- Rust: `active_player_id` → JSON: `"active_player_id"`
- Rust: `has_first_player_token` → JSON: `"has_first_player_token"`

Applied via: `#[serde(rename_all = "snake_case")]`

### Format Details

**TileColor serialization:**
- Format: String
- Values: `"Blue"`, `"Yellow"`, `"Red"`, `"Black"`, `"White"`

**TileMultiset serialization:**
- Format: Object with color keys
- Example: `{"Blue": 3, "Red": 2}`
- Empty multiset: `{}`

**ActionSource serialization:**
- Factory: `{"Factory": 0}`
- Center: `"Center"`

**Destination serialization:**
- Pattern line: `{"PatternLine": 2}`
- Floor: `"Floor"`

**Wall serialization:**
- Format: Array of 5 arrays of 5 booleans
- Example: `[[true, false, false, false, false], ...]`

---

## Complete Example: Mid-Game State

```json
{
  "state_version": 1,
  "ruleset_id": "azul_v1_2p",
  "scenario_seed": "mid_game_scenario_001",
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

### Example Action

```json
{
  "source": {"Factory": 0},
  "color": "Blue",
  "destination": {"PatternLine": 2}
}
```

**Interpretation:**
- Take all Blue tiles from Factory 0
- Place in pattern line row 2 (capacity 3)
- Remaining factory tiles (1 Red, 1 Yellow) move to center

---

## Design Decisions

### TileMultiset: HashMap vs Array

**Choice:** `HashMap<TileColor, u8>`

**Rationale:**
- **Sparse representation:** Only store colors present (common: 1-3 colors per factory)
- **Clear semantics:** `multiset.get(&color)` is more readable than `multiset[color_index]`
- **Flexibility:** Easy to add/remove colors
- **Trade-off:** Slightly larger in memory, hash overhead for lookups
- **Decision:** Clarity and maintainability outweigh minor performance cost

**Alternative:** `[u8; 5]` indexed by color ordinal
- More compact, but less readable
- Stores zeros for absent colors
- Consider for future optimization if profiling shows need

---

### Wall: Bool Array vs Explicit Colors

**Choice:** `[[bool; 5]; 5]`

**Rationale:**
- **Wall colors are fixed by game rules:** Standard Azul pattern never changes
- **Compact:** 25 bytes vs 125 bytes (if using `Option<TileColor>`)
- **Simple checks:** `wall[row][col]` returns filled status directly
- **Helper function:** `get_wall_color(row, col)` provides color when needed
- **Trade-off:** Need to maintain helper function, can't vary wall pattern

**Alternative:** `[[Option<TileColor>; 5]; 5]`
- Stores explicit colors
- Allows non-standard wall patterns (not needed for MVP)
- Much larger (5× memory)

---

### State Mutations: Functional vs In-Place

**Choice:** Functional (clone-and-modify)

```rust
pub fn apply_action(state: &State, action: &DraftAction) -> Result<State, ValidationError>
```

**Rationale:**
- **Immutability:** Easier to reason about, no hidden mutations
- **Testability:** Can compare before/after states easily
- **Concurrency:** Safe for parallel evaluation (Sprint 05)
- **Debugging:** Can log full state transitions
- **Trade-off:** Cloning overhead (~few KB per state)

**Performance notes:**
- State clone cost: ~2-5 KB (mostly tile multisets and walls)
- For 250ms evaluation budget: ~1000 rollouts possible
- Clone cost negligible compared to evaluation logic
- Can optimize later with copy-on-write if profiling shows need

**Alternative:** In-place mutation
- Faster for single-threaded
- More error-prone (need careful undo logic)
- Harder to debug
- Not suitable for parallel evaluation

---

## Serialization Requirements

### Round-Trip Stability

Must hold: `state == deserialize(serialize(state))`

```rust
let state = create_test_state();
let json = serde_json::to_string(&state)?;
let restored: State = serde_json::from_str(&json)?;
assert_eq!(state, restored);
```

### Version Fields

- `state_version` and `ruleset_id` MUST appear in all serialized states
- Enable forward/backward compatibility checks
- Future versions can migrate old states

### Optional Fields

- `scenario_seed` is optional (None for untracked states)
- Use `#[serde(skip_serializing_if = "Option::is_none")]` to omit when None

---

## Invariants (Must Hold)

### Tile Conservation

Total tiles in system must equal `TOTAL_TILES` (100):

```
tiles_in_bag + 
tiles_in_lid + 
tiles_in_factories + 
tiles_in_center + 
tiles_in_pattern_lines + 
tiles_on_walls + 
tiles_on_floors 
= 100
```

**Enforcement:**
- Check after every `apply_action` in tests
- Optional runtime check in production
- Debug assertions in development

### Pattern Line Consistency

For each pattern line:
- `count_filled <= capacity`
- If `count_filled > 0`, then `color` must be `Some(_)`
- If `count_filled == 0`, then `color` must be `None`
- No mixed colors within a single pattern line

### Wall Validity

- Each wall position is either filled (true) or empty (false)
- No "impossible" configurations (e.g., same color twice in one row)
- Note: This is enforced by legality checks, not by data structure

---

## Related Documentation

- [Sprint 01A: Data Models](../sprints/Sprint_01A_Data_Models_Serialization.md) - Implementation details
- [Sprint 01B: Rules & Legality](../sprints/Sprint_01B_Rules_Legality_Checks.md) - Legality checking
- [Rules Specification](03_rules_and_edge_cases.md) - Game rules reference

---

## Future Considerations

### Binary Serialization

For performance-critical paths (e.g., evaluation in Sprint 05):
- Consider MessagePack or Protobuf
- ~50% size reduction vs JSON
- ~2-3× faster serialization
- Trade-off: Less human-readable

**Decision:** Defer until profiling shows need

### State Compression

For scenario packs or replay storage:
- gzip JSON: ~70-80% compression
- Sufficient for MVP

### Schema Evolution

When `state_version` increases:
- Write migration function: `migrate_v1_to_v2(old_state) -> new_state`
- Keep old version parsers for compatibility
- Document breaking changes
