# Rules & Edge Cases Specification (Authoritative)

> This document defines the authoritative interpretation of Azul rules as implemented by the engine.
> All rule interpretations, edge cases, and examples are binding for implementation.

---

## 1. Components & Setup (2-Player)

### Game Components

**Tiles:**
- 5 colors: Blue, Yellow, Red, Black, White
- 20 tiles per color
- **Total: 100 tiles**

**Factories:**
- 2-player game: **5 factories**
- Each factory holds up to 4 tiles during setup

**Center Area:**
- Accumulates tiles from factories
- Contains first-player token at start of round

**Player Boards (each player):**
- 5 pattern lines (capacities: 1, 2, 3, 4, 5)
- 5×5 wall (fixed color pattern)
- Floor line (7 penalty slots)
- Score track

**Supply:**
- Bag: Holds tiles for drawing
- Lid: Holds discarded tiles for refilling bag

---

### Round Setup

**Start of Round:**
1. If bag has fewer than 20 tiles (5 factories × 4), refill bag from lid
2. Draw 4 tiles from bag for each factory
3. Place first-player token in center (if not already there from previous turn)

**First Round Setup:**
- All 100 tiles start in bag
- Factories filled as normal
- First-player token placed in center
- Both players start with score 0

---

## 2. Complete Round Structure

### Phase 1: Factory Filling

```
For each factory (0-4):
  Draw 4 tiles from bag
  Place on factory
  
If bag depletes mid-fill:
  Refill bag from lid
  Continue drawing
  
If bag + lid combined < tiles needed:
  Fill factories with available tiles only
  Some factories may have < 4 tiles
```

---

### Phase 2: Drafting Phase (Sprint 01 Focus)

Players alternate taking actions until all factories and center are empty.

**Action Anatomy:**
1. Choose source (factory or center)
2. Choose color from that source
3. Take ALL tiles of that color
4. If source was factory: remaining factory tiles → center
5. If source was center with token: token → player's floor line
6. Choose destination (pattern line or floor)
7. Place tiles in destination (overflow → floor)
8. Next player's turn

**Drafting Ends When:**
- All factories are empty AND
- Center contains no tiles (first-player token may remain)

---

### Phase 3: Wall Tiling & Scoring (Sprint 03)

For each player, for each pattern line (row 0-4):

```
If pattern_line.count_filled == capacity:
  1. Place one tile on wall at corresponding position
  2. Discard remaining tiles to lid
  3. Score adjacency (horizontal + vertical)
  4. Clear pattern line
```

**Floor Line Scoring:**
```
Penalties: [-1, -1, -2, -2, -2, -3, -3]
For i in 0..min(floor_line.length, 7):
  score += PENALTIES[i]
score = max(score, 0)  // Score cannot go below 0

Discard all floor line tiles to lid
Remove first-player token if present
```

**First Player for Next Round:**
- Player who has first-player token becomes first player
- Token returned to center for next round

---

### Phase 4: Prepare Next Round (Sprint 03)

1. Check end-of-game condition (any player has complete horizontal wall row)
2. If not ended: Refill factories (Phase 1)
3. If ended: Calculate bonus scoring

---

## 3. Drafting Move Rules (Detailed)

### Source Selection

**Option A: Choose a Factory**
- Factory must contain at least one tile
- Player takes ALL tiles of chosen color from factory
- Remaining tiles in factory move to center
- Factory becomes empty

**Option B: Choose Center**
- Center must contain at least one tile of chosen color
- Player takes ALL tiles of chosen color from center
- If first-player token is in center, player receives it

---

### Destination Selection

**Option A: Pattern Line (Row 0-4)**

**Constraints (must ALL be satisfied):**
1. **Capacity:** Row has space (count_filled < capacity)
2. **Color consistency:** If row has tiles, color must match
3. **Wall conflict:** Color must not already exist in wall row
4. **Not complete:** count_filled ≠ capacity

**Behavior:**
```
space_available = capacity - count_filled
tiles_to_place = min(tile_count, space_available)
overflow = tile_count - tiles_to_place

pattern_line.count_filled += tiles_to_place
pattern_line.color = chosen_color

For overflow tiles:
  floor_line.tiles.push(color)
```

**Option B: Floor Line**

**Constraints:** None (always legal)

**Behavior:**
```
For each tile:
  floor_line.tiles.push(color)
```

---

## 4. Standard Azul Wall Pattern

Each wall row has a fixed color order (rotated by 1 from previous row):

```
     Col 0    Col 1    Col 2    Col 3    Col 4
Row 0: Blue   Yellow   Red      Black    White
Row 1: White  Blue     Yellow   Red      Black
Row 2: Black  White    Blue     Yellow   Red
Row 3: Red    Black    White    Blue     Yellow
Row 4: Yellow Red      Black    White    Blue
```

**Implications:**
- Each color appears exactly once per row
- Each color appears exactly once per column
- Pattern line row N can place its tile at wall[N][col] where col depends on color

---

## 5. Edge Cases (Worked Examples)

### Edge Case 1: Color Mismatch in Pattern Line

**Initial State:**
```
Pattern line row 2 (capacity 3):
  color: Red
  count_filled: 2
```

**Action Attempted:**
```
Take 3 Blue tiles from Factory 0 → Pattern Line 2
```

**Result:** ILLEGAL - COLOR_MISMATCH

**Reason:** Pattern line already contains Red tiles, cannot place Blue

**Legal Alternatives:**
- Place Blue in a different row (if no wall conflict)
- Place Blue to Floor

---

### Edge Case 2: Wall Conflict

**Initial State:**
```
Wall row 1, position [1][2]: Filled
(This is Yellow's position in row 1)

Pattern line row 1:
  color: None
  count_filled: 0
```

**Action Attempted:**
```
Take 2 Yellow tiles from Center → Pattern Line 1
```

**Result:** ILLEGAL - WALL_CONFLICT

**Reason:** Yellow already exists in wall row 1 at position [1][2]

**Legal Alternatives:**
- Place Yellow in row 0 (position [0][1])
- Place Yellow in row 2 (position [2][3])
- Place Yellow in row 3 (position [3][4])
- Place Yellow in row 4 (position [4][0])
- Place Yellow to Floor

---

### Edge Case 3: Complete Pattern Line

**Initial State:**
```
Pattern line row 3 (capacity 4):
  color: Blue
  count_filled: 4  // FULL
```

**Action Attempted:**
```
Take 2 Blue tiles from Factory 1 → Pattern Line 3
```

**Result:** ILLEGAL - PATTERN_LINE_COMPLETE

**Reason:** Pattern line is already at capacity, cannot accept more tiles this round

**Legal Alternatives:**
- Place Blue in a different row (if no wall conflict and not full)
- Place Blue to Floor

---

### Edge Case 4: Taking from Center with First-Player Token

**Initial State:**
```
Center:
  tiles: {White: 3, Black: 2}
  has_first_player_token: true

Player 1:
  floor_line:
    tiles: [Red]
    has_first_player_token: false
```

**Action:**
```
Take 3 White tiles from Center → Pattern Line 4
```

**Result:** LEGAL - Token transfers to player

**After Action:**
```
Center:
  tiles: {Black: 2}
  has_first_player_token: false

Player 1:
  pattern_line[4]:
    color: White
    count_filled: 3
  floor_line:
    tiles: [Red]
    has_first_player_token: true  ← Token moved here
```

**Notes:**
- Taking from center with token is always legal (not rejected by legality check)
- Token goes to player's floor line (penalty at end of round)
- Token determines first player for next round

---

### Edge Case 5: Overflow to Floor

**Initial State:**
```
Pattern line row 1 (capacity 2):
  color: Blue
  count_filled: 1

Floor line:
  tiles: []
```

**Action:**
```
Take 4 Blue tiles from Factory 2 → Pattern Line 1
```

**Result:** LEGAL - Overflow tiles go to floor

**After Action:**
```
Pattern line row 1:
  color: Blue
  count_filled: 2  // Full

Floor line:
  tiles: [Blue, Blue]  // 2 overflow tiles
```

**Calculation:**
```
space_available = 2 - 1 = 1
tiles_to_place = min(4, 1) = 1
overflow = 4 - 1 = 3

Wait, error in example above. Let me recalculate:
Actually, we have 1 tile already, space for 1 more.
Taking 4 tiles: 1 fits, 3 overflow.
```

**Corrected After Action:**
```
Pattern line row 1:
  color: Blue
  count_filled: 2  // Full (1 + 1)

Floor line:
  tiles: [Blue, Blue, Blue]  // 3 overflow tiles
```

---

### Edge Case 6: All to Floor

**Initial State:**
```
All pattern lines either:
  - Complete (count == capacity), OR
  - Have wall conflict with Red, OR
  - Have different color

Floor line:
  tiles: [Black]
```

**Action:**
```
Take 3 Red tiles from Center → Floor
```

**Result:** LEGAL - Floor is always available

**After Action:**
```
Floor line:
  tiles: [Black, Red, Red, Red]
```

**Notes:**
- Even if all pattern lines are blocked, floor is always legal
- Guarantees at least one legal action for any color in any source

---

### Edge Case 7: Empty Source

**Initial State:**
```
Factory 2: {}  (empty)
```

**Action Attempted:**
```
Take Blue from Factory 2 → anywhere
```

**Result:** ILLEGAL - SOURCE_EMPTY

**Reason:** Factory contains no Blue tiles (in fact, no tiles at all)

**Prevention:** Legal action enumeration should not generate actions from empty sources

---

### Edge Case 8: Floor Line Exceeds 7 Tiles

**Initial State:**
```
Floor line:
  tiles: [Blue, Red, Yellow, Black, White, Blue]  // 6 tiles
```

**Action:**
```
Take 3 Red from Center → Floor
```

**After Action:**
```
Floor line:
  tiles: [Blue, Red, Yellow, Black, White, Blue, Red, Red, Red]  // 9 tiles
```

**Scoring (End of Round):**
```
Penalties: [-1, -1, -2, -2, -2, -3, -3]
Applied to first 7 tiles only:
  -1 + -1 + -2 + -2 + -2 + -3 + -3 = -14 points

Tiles 8 and 9 do not incur additional penalty
All tiles are discarded to lid
```

---

## 6. Legality Matrix

Table showing: State Condition → Action → Legal/Illegal + Reason

| # | State Condition | Action | Legal? | Reason |
|---|-----------------|--------|--------|--------|
| 1 | Pattern line row 2 empty | Take Blue → PL 2 | ✓ Legal | Empty row accepts any color (if no wall conflict) |
| 2 | Pattern line row 2 has 2 Red | Take Red → PL 2 | ✓ Legal | Color matches, has space |
| 3 | Pattern line row 2 has 2 Red | Take Blue → PL 2 | ✗ Illegal | COLOR_MISMATCH |
| 4 | Wall[1][2] filled (Yellow) | Take Yellow → PL 1 | ✗ Illegal | WALL_CONFLICT |
| 5 | Wall[1][2] filled (Yellow) | Take Yellow → Floor | ✓ Legal | Floor always accepts |
| 6 | Pattern line row 3 full (4/4) | Take any → PL 3 | ✗ Illegal | PATTERN_LINE_COMPLETE |
| 7 | Factory 0: {Blue: 2, Red: 1} | Take Blue → anywhere | ✓ Legal | Source has color |
| 8 | Factory 0: {Blue: 2, Red: 1} | Take Yellow → anywhere | ✗ Illegal | SOURCE_EMPTY |
| 9 | Factory 2 empty | Take any → anywhere | ✗ Illegal | SOURCE_EMPTY |
| 10 | Center has first-player token | Take any from Center | ✓ Legal | Token transfers (side effect) |
| 11 | Pattern line has 3 tiles, taking 5 | Take 5 → PL (capacity 5) | ✓ Legal | Fits with no overflow |
| 12 | Pattern line has 1 tile, taking 4 | Take 4 → PL (capacity 2) | ✓ Legal | 1 fits, 3 overflow to floor |
| 13 | All pattern lines blocked | Take Red → Floor | ✓ Legal | Floor always available |

---

## 7. Floor Line Penalties (Reference for Sprint 03)

**Penalty Values:**
```
Slot:    0   1   2   3   4   5   6
Penalty: -1  -1  -2  -2  -2  -3  -3
```

**Application:**
- First-player token occupies slot 0 (if player has it)
- Tiles fill remaining slots left-to-right
- Penalties apply to first 7 slots only
- Score cannot go below 0

**Examples:**

**Example 1:** 3 tiles, no token
```
Floor: [Blue, Red, Yellow]
Penalty: -1 + -1 + -2 = -4
```

**Example 2:** 5 tiles, has token
```
Floor: [Token, Blue, Red, Yellow, Black, White]
Penalty: -1 + -1 + -2 + -2 + -2 + -3 = -11
```

**Example 3:** 9 tiles, no token
```
Floor: [Blue, Red, Yellow, Black, White, Blue, Red, Blue, Yellow]
Penalty: -1 + -1 + -2 + -2 + -2 + -3 + -3 = -14
(Tiles 8 and 9 incur no additional penalty)
```

---

## 8. Tile Bag & Lid Behavior

### Bag Refill

**When:** Bag has insufficient tiles to fill factories

**Process:**
```
If bag.total_tiles() < (factories.len() * 4):
  Move all tiles from lid to bag
  Continue filling factories from bag
```

**Edge Case: Partial Factory Fill**

If bag + lid combined have fewer than needed tiles:
```
Factory 0: [Blue, Red, Yellow, Black]  // 4 tiles
Factory 1: [White, Blue, Red]          // 3 tiles (bag depleted)
Factory 2: [Yellow]                     // 1 tile
Factory 3: []                           // empty
Factory 4: []                           // empty
```

This is LEGAL but rare (late in game after many wall placements).

---

## 9. End-of-Round Scoring (Reference for Sprint 03)

### Wall Tiling

For each complete pattern line:

**Step 1:** Place one tile on wall
```
row = pattern_line_index
col = get_wall_column_for_color(row, pattern_line.color)
wall[row][col] = true
```

**Step 2:** Score adjacency

**Horizontal scoring:**
```
Count contiguous tiles in row (including newly placed)
If only 1 tile (isolated): score += 1
If > 1 tile: score += count
```

**Vertical scoring:**
```
Count contiguous tiles in column (including newly placed)
If > 1 tile: score += count
(Do not score 1 point for single tile again)
```

**Example:**
```
Before:
  [X][X][ ][ ][ ]
  [ ][ ][ ][ ][ ]

Place tile at [0][2]:
  [X][X][N][ ][ ]
  [ ][ ][ ][ ][ ]

Horizontal: 3 contiguous → +3 points
Vertical: 1 tile (isolated) → +0 points (already counted horizontal)
Total: +3 points
```

**Step 3:** Discard tiles
```
Discard (capacity - 1) tiles from pattern line to lid
Clear pattern line: count_filled = 0, color = None
```

---

## 10. Test Vectors

### Test Vector 1: Legal Action Enumeration

**State:**
- Factory 0: {Blue: 2, Red: 1, Yellow: 1}
- Factory 1: {Black: 3, White: 1}
- Factory 2: {}
- Factory 3: {Blue: 1, Yellow: 3}
- Factory 4: {Red: 2, Black: 2}
- Center: {White: 2, Red: 1}, token: true
- Player 0 pattern lines: all empty
- Player 0 wall: empty

**Expected Legal Action Count:**
```
Factory 0: 3 colors × 6 destinations = 18 actions
Factory 1: 2 colors × 6 destinations = 12 actions
Factory 2: 0 actions (empty)
Factory 3: 2 colors × 6 destinations = 12 actions
Factory 4: 2 colors × 6 destinations = 12 actions
Center: 2 colors × 6 destinations = 12 actions
Total: 66 actions
```

---

### Test Vector 2: Action Application with Overflow

**Before State:**
- Factory 0: {Blue: 4}
- Player 0 pattern line 1: {color: Blue, count: 1}
- Player 0 floor: []

**Action:**
```
Take Blue from Factory 0 → Pattern Line 1
```

**After State:**
- Factory 0: {}
- Center: {} (no remnants)
- Player 0 pattern line 1: {color: Blue, count: 2}  // capacity reached
- Player 0 floor: [Blue, Blue]  // 2 overflow tiles
- Active player: toggled

---

### Test Vector 3: First-Player Token Transfer

**Before State:**
- Center: {White: 2}, token: true
- Player 1 floor: []

**Action:**
```
Take White from Center → Pattern Line 4
```

**After State:**
- Center: {}, token: false
- Player 1 pattern line 4: {color: White, count: 2}
- Player 1 floor: [], token: true

---

## 11. Invariants to Test

### Tile Conservation

After any action:
```
total_tiles(bag) + 
total_tiles(lid) + 
total_tiles(factories) + 
total_tiles(center) + 
total_tiles(all_pattern_lines) + 
total_tiles(all_walls) + 
total_tiles(all_floors) 
= 100
```

### Pattern Line Validity

For each pattern line after any action:
```
count_filled <= capacity
count_filled > 0 ⟹ color == Some(_)
count_filled == 0 ⟹ color == None
```

### First-Player Token Uniqueness

At any time:
```
count_tokens(center) + 
sum(count_tokens(player.floor) for player in players) 
= 1
```

---

## Related Documentation

- [Sprint 01B: Rules & Legality](../sprints/Sprint_01B_Rules_Legality_Checks.md)
- [Sprint 01C: Apply Action](../sprints/Sprint_01C_Apply_Action_Transitions.md)
- [State Model Specification](04_state_action_model_and_serialization.md)
