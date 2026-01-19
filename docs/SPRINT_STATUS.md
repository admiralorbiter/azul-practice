# Sprint Status Tracker

**Last Updated:** January 19, 2026 (Sprint 04 Complete - Deterministic Fix)

## Overview

This document tracks the completion status of all sprints for the Azul Practice Tool project.

---

## ✅ Completed Sprints

### Sprint 00: Foundation & WASM Pipeline
**Status:** ✅ **COMPLETED**  
**Completion:** Prior to Sprint 01  
**Documentation:** [Sprint_00_Foundation_WASM_Pipeline.md](sprints/Sprint_00_Foundation_WASM_Pipeline.md)

**Key Deliverables:**
- Repository structure established
- Rust → WASM build pipeline working
- WASM integration in Vite/React app
- `get_version()` and `ping()` exports working
- Dev-mode logging to browser console

---

### Sprint 01: Core Engine (Draft Phase)
**Status:** ✅ **COMPLETED**  
**Completion:** Prior to Sprint 02  
**Documentation:** [Sprint_01_Core_Engine_v0_State_Actions_Legality.md](sprints/Sprint_01_Core_Engine_v0_State_Actions_Legality.md)

**Sub-Sprints:**
- ✅ **Sprint 01A:** Data Models & Serialization
- ✅ **Sprint 01B:** Rules & Legality Checks
- ✅ **Sprint 01C:** Action Application & State Transitions
- ✅ **Sprint 01D:** WASM Integration & Tests

**Key Deliverables:**
- Complete `State` and `DraftAction` types with JSON serialization
- `list_legal_actions(state, player_id)` function with full validation
- `apply_action(state, action)` function with tile conservation
- WASM exports with TypeScript wrappers
- Comprehensive test coverage

---

### Sprint 02: UI v0 (Board Rendering & Click Interaction)
**Status:** ✅ **COMPLETED**  
**Completion Date:** January 18, 2026  
**Documentation:** 
- [Sprint_02_UI_v0_Board_Render_Click_Interactions.md](sprints/Sprint_02_UI_v0_Board_Render_Click_Interactions.md)
- [Sprint_02_COMPLETED.md](sprints/Sprint_02_COMPLETED.md) (detailed report)

**Key Deliverables:**
- **26 files created:**
  - 11 board components (Factory, Center, PatternLine, WallGrid, FloorLine, PlayerBoard, GameBoard)
  - 3 UI components (ColorPicker, ErrorToast, DevPanel)
  - 1 main screen (PracticeScreen)
  - Custom hooks and utilities
- Full click-to-select interaction flow
- Legal move highlighting system
- WASM integration for actions
- Error handling with toast notifications
- Dev panel with state inspection
- 3 test scenarios (early, mid, late game)

**Known Issues Fixed:**
- Tile conservation violations in test scenarios (now all have exactly 100 tiles)

---

### Sprint 03: End-of-Round Scoring & Refill
**Status:** ✅ **COMPLETED**  
**Completion Date:** January 18, 2026  
**Documentation:** 
- [Sprint_03_End_of_Round_Scoring_Refill.md](sprints/Sprint_03_End_of_Round_Scoring_Refill.md)
- [Sprint_03C_COMPLETED.md](sprints/Sprint_03C_COMPLETED.md) (detailed report)

**Sub-Sprints:**
- ✅ **Sprint 03A:** Wall Tiling & Pattern Line Resolution
- ✅ **Sprint 03B:** Scoring System with Golden Tests
- ✅ **Sprint 03C:** Round Transition & Refill

**Key Deliverables:**
- **Pattern Line Resolution (03A):**
  - `resolve_pattern_lines()` function with wall tile placement
  - Excess tile discard to lid (capacity - 1)
  - Pattern line cleanup after resolution
  - 8 unit tests with tile conservation checks
- **Scoring System (03B):**
  - `calculate_wall_tile_score()` for adjacency scoring (horizontal + vertical)
  - `calculate_floor_penalty()` with 7-slot penalty system
  - `apply_floor_penalties()` with score clamping to 0
  - 21 comprehensive tests (10 wall scoring, 6 floor penalty, 3 clamping, 3 integration)
- **Round Transition (03C):**
  - Complete `resolve_end_of_round()` orchestration function
  - `refill_factories()` with bag/lid mechanics
  - Game end detection (complete horizontal row)
  - First player token handling
  - WASM export and TypeScript wrapper
  - Dev UI "Resolve End of Round" button
  - 8 integration tests for full end-of-round flow
- **Total Test Suite:** 112 tests passing (90 unit + 9 integration + 13 doc tests)

**Known Issues Fixed:**
- Graceful handling of invalid wall placements (skip instead of panic)

---

### Sprint 04: Scenario Generation
**Status:** ✅ **COMPLETED** (with deterministic guarantee)  
**Completion Date:** January 19, 2026  
**Documentation:** 
- [Sprint_04_Scenario_Generation_Phases_Filters.md](sprints/Sprint_04_Scenario_Generation_Phases_Filters.md)
- [Sprint_04_COMPLETED.md](sprints/Sprint_04_COMPLETED.md) (detailed report)
- [DOCUMENTATION_UPDATE_2026-01-19_DETERMINISTIC_FIX.md](DOCUMENTATION_UPDATE_2026-01-19_DETERMINISTIC_FIX.md)

**Key Deliverables:**
- **Deterministic Stage-Driven Generation:**
  - Generator completes rounds **until** target stage is reached (not fixed round count)
  - **Guaranteed** wall tile counts: Early (≤8), Mid (9-17), Late (≥18)
  - Stage-driven loop with safety checks (max 10 rounds, overshoot detection)
  - No more "Max attempts exceeded" errors
- **Two-Axis Staging System:**
  - **GameStage** (across-game): Early/Mid/Late based on wall tiles
  - **RoundStage** (within-round): Start/Mid/End based on tiles on table
  - Independent targeting on both axes
- **Snapshot Sampling:**
  - Records decision points during gameplay (every 2 actions)
  - Selects best snapshot matching target stage and quality filters
  - Produces varied scenarios within stage constraints
- **Policy-Based Simulation:**
  - `RandomPolicy` and `GreedyPolicy` implementations
  - Policy mix configuration (AllRandom/AllGreedy/Mixed)
  - Deterministic bot behavior with seeded RNG
- **Enhanced Quality Filters:**
  - Minimum legal actions: 6 (raised from 3)
  - Minimum unique destinations: 2
  - Require non-floor option: true
  - Max floor ratio: 0.5
  - Hard fallback ensures UI never fails
- **Seed-Based Reproducibility:**
  - Deterministic RNG with seed storage in state
  - Same seed generates identical scenario
  - Seed display and copy in DevPanel
- **UI Integration:**
  - Two dropdowns: "Game Stage" and "Round Stage"
  - "New Scenario" button with 500 retry attempts
  - Enhanced DevPanel with both stage axes displayed
- **Total Test Suite:** 126 tests passing (4 probabilistic tests ignored)

**Critical Fixes:**
1. **Wall Filling (Initial):** Redesigned to complete full rounds with `resolve_end_of_round()`, enabling realistic Mid/Late game scenarios with filled walls.
2. **Deterministic Stage Guarantee (Final):** Replaced probabilistic "complete N rounds and hope" with deterministic "complete rounds until stage matches." This **guarantees** correct wall tile counts for all stages.

**Known Limitations:**
- 2-player only (no 3/4-player support yet)
- Greedy policy is heuristic-based (not optimal play)

---

## 🚧 In Progress Sprints

*None currently*

---

## 📋 Planned Sprints

---

### Sprint 05: Best Move Evaluation
**Status:** 📋 **PLANNED**  
**Documentation:** [Sprint_05_Best_Move_Evaluator_Tier2_Think_Longer.md](sprints/Sprint_05_Best_Move_Evaluator_Tier2_Think_Longer.md)

**Planned Work:**
- Monte Carlo rollout evaluator
- Multi-ply lookahead
- Time budget controls
- Move ranking
- Evaluation quality tiers

---

### Sprint 06: Feedback & Polish
**Status:** 📋 **PLANNED**  
**Documentation:** [Sprint_06_Feedback_Explanations_DragDrop_Polish.md](sprints/Sprint_06_Feedback_Explanations_DragDrop_Polish.md)

**Planned Work:**
- Move feedback and grading
- Explanation generation
- Drag-and-drop interaction
- UI polish and animations
- Accessibility improvements

---

### Sprint 07: Content & Calibration (Optional)
**Status:** 📋 **OPTIONAL**  
**Documentation:** [Sprint_07_Optional_Content_Calibration.md](sprints/Sprint_07_Optional_Content_Calibration.md)

**Planned Work:**
- Evaluation calibration
- Content testing
- Performance optimization
- Advanced features

---

## Statistics

**Completion Summary:**
- ✅ Completed: 4 sprints (00, 01, 02, 03, 04)
  - Including 4 Sprint 01 sub-sprints (01A, 01B, 01C, 01D)
  - Including 3 Sprint 03 sub-sprints (03A, 03B, 03C)
- 🚧 In Progress: 0 sprints
- 📋 Planned: 3 sprints (05, 06, 07)

**Progress:**
- **50%** complete (4 of 8 major sprints)
- **Core engine:** 100% complete (Sprint 01: 4 sub-sprints)
- **Basic UI:** 100% complete (Sprint 02)
- **Game logic:** 100% complete (Sprint 03: 3 sub-sprints)
- **Scenario generation:** 100% complete (Sprint 04)
- **AI/Evaluation:** 0% (Sprint 05 next)
- **Polish:** 0% (Sprint 06 planned)

**Subdivision Approach:**
- Sprint 1: Subdivided into 4 focused sub-sprints ✅ Completed
- Sprint 3: Subdivided into 3 focused sub-sprints ✅ Completed
- Sprint 4: Unified sprint (with iterative fixes) ✅ Completed
- This approach enables incremental validation and clearer dependencies

---

## Next Steps

**Immediate Priority:** Sprint 05 (Best Move Evaluation)

With scenarios now generating realistic game states, we can implement move evaluation:
- Monte Carlo rollout evaluator
- Multi-ply lookahead
- Move ranking and quality scoring
- Foundation for practice feedback (Sprint 06)

**Dependencies:**
- Sprint 05 depends on Sprint 03 ✅ (complete - full game simulation available)
- Sprint 05 depends on Sprint 04 ✅ (complete - policy infrastructure and scenarios available)
- Sprint 06 depends on Sprint 05 (needs move evaluation for feedback)

---

## Files Created/Modified (Sprint 03)

### New Files (2 Rust modules)
**Rust Engine:**
- `rust/engine/src/rules/resolution.rs` (~89 lines) - Pattern line resolution
- `rust/engine/src/rules/scoring.rs` (~185 lines) - Wall scoring and floor penalties
- `rust/engine/src/rules/refill.rs` (~80 lines) - Factory refill mechanics
- `rust/engine/src/rules/end_of_round.rs` (~95 lines) - End-of-round orchestration

### Modified Files
**Rust:**
- `rust/engine/Cargo.toml` - Added rand dependencies
- `rust/engine/src/rules/mod.rs` - Exported new modules
- `rust/engine/src/wasm_api.rs` - Added resolve_end_of_round export
- `rust/engine/src/rules/tests.rs` - Added 37 new tests (~600 lines)

**Web:**
- `web/src/wasm/engine.ts` - Added resolveEndOfRound wrapper
- `web/src/components/dev/DevPanel.tsx` - Added resolve button
- `web/src/components/dev/DevPanel.css` - Button styles
- `web/src/components/PracticeScreen.tsx` - Connected callback

### Documentation
- `docs/sprints/Sprint_03C_COMPLETED.md` - Detailed completion report
- `docs/SPRINT_STATUS.md` - Updated completion status
- Updated Sprint 03, 03A, 03B, 03C docs with completion markers

**Total:** ~1,080 lines added (production + tests), 112 tests passing

---

## Files Created/Modified (Sprint 02)

### New Files (26 total)
**Components (13):**
- `web/src/components/board/Factory.tsx` + `.css`
- `web/src/components/board/CenterArea.tsx` + `.css`
- `web/src/components/board/PatternLine.tsx` + `.css`
- `web/src/components/board/WallGrid.tsx` + `.css`
- `web/src/components/board/FloorLine.tsx` + `.css`
- `web/src/components/board/PlayerBoard.tsx` + `.css`
- `web/src/components/board/GameBoard.tsx` + `.css`
- `web/src/components/ui/ColorPicker.tsx` + `.css`
- `web/src/components/ui/ErrorToast.tsx` + `.css`
- `web/src/components/dev/DevPanel.tsx` + `.css`
- `web/src/components/PracticeScreen.tsx` + `.css`

**Utilities & Hooks (3):**
- `web/src/hooks/useActionSelection.ts`
- `web/src/styles/colors.ts`
- `web/src/test-scenarios.ts`

### Modified Files
- `web/src/App.tsx` - Updated to use PracticeScreen
- `web/src/wasm/engine.ts` - Fixed type safety (unknown instead of any)
- `web/.eslintrc.cjs` - Added WASM pkg ignore pattern

### Documentation
- `docs/sprints/Sprint_02_COMPLETED.md` - Detailed completion report
- `docs/SPRINT_STATUS.md` - This file
- Updated Sprint 00, 01, 02 docs with completion status

---

## Quick Reference

**To run the application:**
```bash
# Build WASM (from project root)
npm run wasm:build

# Start dev server
cd web
npm run dev
```

**To test:**
1. Load a scenario (Early/Mid/Late Game)
2. Click factory → Select color → Click destination → Apply Move
3. Verify state updates and player switches
4. Make moves until pattern lines are complete
5. Open Dev Panel → Click "Resolve End of Round"
6. Verify: scores update, wall tiles placed, factories refilled
7. Continue playing into next round

**Current capabilities:**
- ✅ Load game states
- ✅ Visualize board (factories, center, player boards)
- ✅ Select and apply draft moves
- ✅ Highlight legal moves
- ✅ Error handling
- ✅ Multi-move sequences
- ✅ End-of-round resolution (Sprint 03)
- ✅ Wall tile scoring with adjacency (Sprint 03)
- ✅ Floor penalties and score clamping (Sprint 03)
- ✅ Factory refill with bag/lid mechanics (Sprint 03)
- ✅ Game end detection (Sprint 03)
- ✅ Full game loop (draft → resolve → next round) (Sprint 03)
- ✅ Scenario generation with phase targeting (Sprint 04)
- ✅ Multi-round simulation with realistic board progression (Sprint 04)
- ✅ Seed-based reproducibility (Sprint 04)
- ✅ Quality filters and robust generation (Sprint 04)
- ❌ Move evaluation (Sprint 05)
- ❌ Drag-and-drop (Sprint 06)
- ❌ Move feedback and grading (Sprint 06)
