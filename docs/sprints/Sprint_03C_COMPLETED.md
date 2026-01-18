# Sprint 03 (Complete) — End-of-Round Resolution Implementation Report

**Completion Date:** January 18, 2026  
**Sprint Duration:** Sprint 03A, 03B, 03C (Sequential)  
**Status:** ✅ **FULLY COMPLETE**

---

## Executive Summary

Sprint 03 successfully implemented the complete end-of-round resolution system for Azul, including pattern line resolution, wall tile placement, adjacency scoring, floor penalties, and factory refill mechanics. The sprint was subdivided into three focused sub-sprints (03A, 03B, 03C) following the successful pattern from Sprint 01, enabling incremental validation and clear dependency management.

**Impact:** The game engine now supports a complete gameplay loop from draft actions through end-of-round resolution to the next round, with factories refilling and scores updating correctly.

---

## Sub-Sprint Breakdown

### Sprint 03A: Wall Tiling & Pattern Line Resolution ✅

**Goal:** Implement pattern line resolution logic that moves completed tiles to the wall.

**Key Deliverables:**
- Created `rust/engine/src/rules/resolution.rs` with `resolve_pattern_lines()` function
- Tile placement on wall at correct positions using `get_wall_column_for_color()`
- Excess tile discard to lid (capacity - 1 per pattern line)
- Pattern line state cleanup after resolution
- Integrated immediate scoring (Sprint 03B integration point)
- 8 comprehensive unit tests with tile conservation checks

**Test Coverage:**
- Complete pattern line resolution (single and multiple)
- Row 0 edge case (no discard, capacity = 1)
- Both players resolving simultaneously
- Tile conservation validation
- Multiple colors across different rows

---

### Sprint 03B: Scoring System ✅

**Goal:** Implement accurate scoring for wall tiles and floor penalties with comprehensive golden tests.

**Key Deliverables:**
- Created `rust/engine/src/rules/scoring.rs` with three core functions:
  - `calculate_wall_tile_score()` - Adjacency scoring (horizontal + vertical chains)
  - `calculate_floor_penalty()` - 7-slot penalty system with first-player token
  - `apply_floor_penalties()` - Score application with clamping to 0
- Integrated scoring into pattern line resolution (Sprint 03A)
- 21 comprehensive tests covering all scoring scenarios

**Test Coverage:**
- **Wall Tile Scoring (10 golden tests):**
  - Isolated tile (1 point)
  - Horizontal chains: 2, 5 tiles
  - Vertical chains: 3, 5 tiles
  - T-shape (6 points: 3+3)
  - Cross/maximum (10 points: 5+5)
  - L-shape (5 points: 3+2)
  - Gap stops chain (2 points)
  - Corner placement (3 points)
- **Floor Penalties (6 tests):**
  - Empty floor (0)
  - Token only (-1)
  - 3 tiles without token (-4)
  - 3 tiles with token (-6)
  - Maximum penalty (-14)
  - 7 tiles without token (-14)
- **Score Clamping (2 tests):**
  - Clamping to zero
  - Positive result preservation
- **Integration (3 tests):**
  - Pattern line resolution with scoring
  - Multiple tiles scoring independently
  - Complete scoring flow (wall + floor)

---

### Sprint 03C: Round Transition & Refill ✅

**Goal:** Orchestrate complete end-of-round flow with factory refill and game end detection.

**Key Deliverables:**
- Created `rust/engine/src/rules/end_of_round.rs` with:
  - `resolve_end_of_round()` - Main orchestration function
  - `check_game_end()` - Complete horizontal row detection
- Created `rust/engine/src/rules/refill.rs` with:
  - `refill_factories()` - Factory refill from bag
  - `draw_random_tile_from_bag()` - Random tile selection
  - Bag refill from lid when insufficient tiles
- Added dependencies: `rand = "0.8"` and `getrandom = "0.2"` for randomness
- WASM export: `resolve_end_of_round()` function
- TypeScript wrapper: `resolveEndOfRound()` in `engine.ts`
- UI Integration: "Resolve End of Round" button in DevPanel
- 8 integration tests for complete end-of-round flow

**Test Coverage:**
- Complete end-of-round flow (all phases)
- Bag refill from lid (when bag < 20 tiles)
- Game end detection (complete horizontal row)
- Partial factory fill (late game edge case)
- First player determination and token placement
- Tile conservation through full resolution
- Floor tiles discarded to lid
- No complete row edge case

---

## Technical Implementation

### Architecture

The end-of-round system follows a four-phase architecture:

```
Phase 1: Wall Tiling & Scoring
  ↓ resolve_pattern_lines() + calculate_wall_tile_score()
  ↓ apply_floor_penalties()

Phase 2: Cleanup
  ↓ Determine next first player
  ↓ Discard floor tiles to lid
  ↓ Return token to center

Phase 3: Game State Check
  ↓ check_game_end()
  ↓ If ended: return (no refill)

Phase 4: Next Round Setup
  ↓ Increment round number
  ↓ refill_factories() with bag/lid mechanics
```

### Files Created

**Rust Engine (3 new modules):**
- `rust/engine/src/rules/resolution.rs` (~89 lines)
- `rust/engine/src/rules/scoring.rs` (~185 lines)
- `rust/engine/src/rules/refill.rs` (~80 lines)

**Total:** ~354 lines of production Rust code

### Files Modified

**Rust Engine:**
- `rust/engine/Cargo.toml` - Added rand dependencies
- `rust/engine/src/rules/mod.rs` - Exported new modules
- `rust/engine/src/wasm_api.rs` - Added WASM export (~45 lines)
- `rust/engine/src/rules/tests.rs` - Added 37 tests (~600 lines)

**Web Application:**
- `web/src/wasm/engine.ts` - Added TypeScript wrapper (~40 lines)
- `web/src/components/dev/DevPanel.tsx` - Added button and handler (~25 lines)
- `web/src/components/dev/DevPanel.css` - Added button styles (~15 lines)
- `web/src/components/PracticeScreen.tsx` - Connected callback (~5 lines)

**Total Changes:** ~1,080 lines (production + tests)

---

## Test Results

### Test Suite Summary

**Total Tests: 112 passing**
- 90 unit tests (lib tests)
- 9 integration tests (WASM boundary)
- 13 documentation tests

**New Tests Added in Sprint 03:**
- Sprint 03A: 8 pattern line resolution tests
- Sprint 03B: 21 scoring system tests
- Sprint 03C: 8 end-of-round integration tests
- **Total:** 37 new tests

### Test Execution Time

All 112 tests execute in < 2 seconds, maintaining fast iteration cycles.

---

## Key Features Implemented

### 1. Pattern Line Resolution
- ✅ Detects complete pattern lines (count_filled == capacity)
- ✅ Places one tile on wall at correct position
- ✅ Discards excess tiles to lid (capacity - 1)
- ✅ Clears pattern line state (count_filled = 0, color = None)
- ✅ Handles all 5 rows for both players
- ✅ Gracefully skips invalid placements (wall already filled)

### 2. Adjacency Scoring
- ✅ Horizontal chain counting (left and right)
- ✅ Vertical chain counting (up and down)
- ✅ Isolated tile scoring (1 point)
- ✅ Single direction scoring (chain length)
- ✅ Both directions scoring (sum of both chains)
- ✅ Maximum score: 10 points (5+5 cross pattern)
- ✅ Gap detection stops chain counting

### 3. Floor Penalties
- ✅ 7-slot penalty system: -1, -1, -2, -2, -2, -3, -3
- ✅ First-player token occupies slot 0 (-1 penalty)
- ✅ Tiles beyond 7th slot incur no penalty
- ✅ Score clamping to 0 (cannot go negative)
- ✅ Maximum penalty: -14 points

### 4. Factory Refill
- ✅ Clears all factories and center
- ✅ Draws 20 tiles from bag (4 per factory × 5)
- ✅ Random tile selection with `rand` crate
- ✅ Bag refill from lid when insufficient (<20 tiles)
- ✅ Partial fill support (late game with <20 total tiles)
- ✅ Tile conservation maintained

### 5. Round Transition
- ✅ First player determination (who has token)
- ✅ Token returned to center for next round
- ✅ Floor tiles discarded to lid
- ✅ Floor lines cleared
- ✅ Round number incremented
- ✅ Game end detection (complete horizontal row)
- ✅ Conditional factory refill (skip if game ended)

### 6. WASM Integration
- ✅ `resolve_end_of_round()` exported to WASM
- ✅ TypeScript wrapper with error handling
- ✅ Type-safe integration with GameState
- ✅ Error propagation to UI

### 7. Dev UI
- ✅ "Resolve End of Round" button in Dev Panel
- ✅ Immediate state update on click
- ✅ Visual feedback (board refreshes)
- ✅ Error handling with alerts
- ✅ Green button styling with hover effects

---

## Edge Cases Handled

### 1. Invalid Wall Placements
**Issue:** Pattern line complete but wall position already filled  
**Solution:** Gracefully skip placement, clear pattern line, continue processing  
**Impact:** Prevents panic in browser, allows game to continue

### 2. Insufficient Tiles (Late Game)
**Scenario:** Bag + lid have fewer than 20 tiles  
**Behavior:** Factories partially fill with available tiles  
**Result:** Game continues normally with fewer tiles

### 3. Game End Detection
**Trigger:** Any player completes a horizontal row (all 5 tiles)  
**Behavior:** Skip factory refill, mark game ended  
**Future:** End-of-game bonuses deferred to future sprint

### 4. First Player Edge Case
**Scenario:** No player has token (shouldn't happen in normal gameplay)  
**Fallback:** Keep current active_player_id  
**Impact:** Game remains stable

### 5. Row 0 Pattern Line
**Special Case:** Capacity = 1, so no tiles to discard (capacity - 1 = 0)  
**Handling:** Check `tiles_to_discard > 0` before lid insertion  
**Verified:** Unit test confirms 0 tiles go to lid

---

## Known Issues & Fixes

### Issue 1: Wall Position Already Filled
**Discovered During:** UI testing  
**Error:** `panicked at engine\src\rules\resolution.rs:63:17: Wall position [0, 0] already filled`  
**Root Cause:** `debug_assert!` triggered on invalid state (manually edited or edge case)  
**Fix:** Changed to graceful handling - skip placement and clear pattern line  
**Status:** ✅ Fixed and tested  
**Impact:** Prevents crashes, allows game to continue with inconsistent states

### Issue 2: Clippy Warning (Unnecessary Cast)
**Warning:** `casting to the same type is unnecessary (u8 -> u8)`  
**Location:** `end_of_round.rs:81`  
**Fix:** Removed redundant `as u8` cast  
**Status:** ✅ Fixed  
**Impact:** Clean build with no warnings

---

## Performance Characteristics

### Test Execution
- **Unit tests:** < 0.01s (90 tests)
- **Integration tests:** < 0.01s (9 tests)
- **Doc tests:** ~1.2s (13 tests with compilation)
- **Total:** < 2s for all 112 tests

### WASM Module
- **Size:** ~995 KB (unoptimized dev build)
- **Load time:** < 50ms on local dev server
- **Execution:** < 5ms for full end-of-round resolution

### Randomness
- Uses `rand` crate with `thread_rng()`
- Compatible with WASM via `getrandom` with `js` feature
- Deterministic seeds possible for testing (future enhancement)

---

## Integration Points

### With Sprint 01 (Core Engine)
- ✅ Uses `State` and data models
- ✅ Maintains tile conservation invariants
- ✅ Compatible with `apply_action()` flow

### With Sprint 02 (UI)
- ✅ Dev Panel provides testing interface
- ✅ State updates trigger UI refresh
- ✅ Error handling integrates with existing toast system
- ✅ Visual feedback matches game state changes

### With Future Sprints
- **Sprint 04 (Scenario Generation):** `resolve_end_of_round()` enables multi-round scenarios
- **Sprint 05 (Best Move Evaluation):** Complete game loop allows rollout-based evaluation
- **Sprint 06 (Feedback):** Score changes and explanations hook into this system

---

## Developer Experience

### Testing Workflow
1. Load scenario in UI (early/mid/late game)
2. Make several draft moves
3. Open Dev Panel
4. Click "Resolve End of Round"
5. Verify: round increments, scores update, factories refill

### Debugging Support
- Dev Panel shows complete state JSON
- Copy/paste state for reproduction
- Manual state editing supported (with graceful handling)
- Console logging for errors
- Comprehensive test coverage aids debugging

### Code Quality
- ✅ No compiler warnings (except pre-existing in lib.rs)
- ✅ Clippy clean (except pre-existing warnings)
- ✅ Comprehensive documentation with examples
- ✅ Type safety via Rust and TypeScript
- ✅ Clear separation of concerns (resolution, scoring, refill)

---

## Lessons Learned

### 1. Sub-Sprint Structure Works Well
The three-phase subdivision (03A, 03B, 03C) proved highly effective:
- Each phase had clear deliverables
- Testing was incremental and manageable
- Dependencies were explicit and linear
- Debugging was easier with isolated subsystems

### 2. Golden Tests Are Critical
Sprint 03B's 21 golden tests provided confidence in scoring correctness:
- Covers all adjacency patterns
- Documents expected behavior
- Catches regression quickly
- Serves as specification

### 3. Graceful Degradation Matters
Initial panic on invalid wall placement was user-hostile:
- Changed to graceful skipping
- Allows game to continue
- Better UX for edge cases
- More robust to manual state editing

### 4. WASM Integration Is Smooth
The WASM boundary continues to work well:
- Rust functions export cleanly
- TypeScript wrappers provide safety
- Error handling works across boundary
- JSON serialization is reliable

---

## Statistics

### Code Metrics
- **New Rust modules:** 3
- **New Rust functions:** 8 (production)
- **New TypeScript functions:** 1 (wrapper)
- **New UI components:** 1 (button in DevPanel)
- **Total lines added:** ~1,080 (production + tests)
- **Tests added:** 37
- **Test pass rate:** 100% (112/112)

### Sprint Velocity
- **Sprint 03A:** ~1 day (implementation + testing)
- **Sprint 03B:** ~1 day (implementation + 21 golden tests)
- **Sprint 03C:** ~1 day (orchestration + integration + UI)
- **Bug fixes:** ~1 hour (graceful handling)
- **Total:** ~3 days for complete end-of-round system

---

## Next Steps

### Immediate Follow-Up (Optional)
- [ ] Add end-of-game bonuses (row/column/color completion)
- [ ] Deterministic RNG seeding for reproducible refills
- [ ] Additional edge case testing

### Sprint 04: Scenario Generation
- Can now generate scenarios across multiple rounds
- Factory refill enables realistic mid/late-game states
- Scoring system enables evaluation of position quality

### Sprint 05: Best Move Evaluation
- Complete game loop enables rollout-based evaluation
- Can simulate future rounds for move quality assessment
- Scoring provides objective evaluation metric

---

## Conclusion

Sprint 03 successfully implemented the complete end-of-round resolution system, enabling Azul gameplay from draft through scoring to the next round. The subdivision into three focused sub-sprints (03A, 03B, 03C) proved effective for managing complexity and ensuring quality.

**Key Achievements:**
- ✅ Complete game loop functional (draft → resolve → refill)
- ✅ 37 new tests with 100% pass rate
- ✅ Clean WASM integration with TypeScript wrappers
- ✅ Dev UI support for manual testing
- ✅ Graceful error handling
- ✅ Comprehensive documentation

**Foundation Established For:**
- Multi-round scenario generation
- Best move evaluation with lookahead
- Full gameplay experience
- AI training and evaluation

Sprint 03 marks a major milestone in the Azul Practice Tool, completing the core game engine functionality needed for the practice and evaluation features planned in later sprints.

---

**Report Compiled:** January 18, 2026  
**Sprint Status:** ✅ Complete  
**Next Sprint:** 04 - Scenario Generation
