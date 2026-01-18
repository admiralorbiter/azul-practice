# Sprint Status Tracker

**Last Updated:** January 18, 2026

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

## 🚧 In Progress Sprints

*None currently*

---

## 📋 Planned Sprints

### Sprint 03: End-of-Round Scoring & Refill
**Status:** 📋 **PLANNED** (Subdivided into 3 focused sub-sprints)  
**Documentation:** [Sprint_03_End_of_Round_Scoring_Refill.md](sprints/Sprint_03_End_of_Round_Scoring_Refill.md)

**Sub-Sprints:**
- 📋 **Sprint 03A:** Wall Tiling & Pattern Line Resolution
- 📋 **Sprint 03B:** Scoring System with Golden Tests
- 📋 **Sprint 03C:** Round Transition & Refill

**Planned Work:**
- Detect end-of-draft-phase
- Wall tiling logic
- Scoring (horizontal, vertical adjacency)
- Floor line penalty application
- Factory refill from bag/lid
- Round completion and game end detection
- WASM integration and UI button

**Subdivision Rationale:** Following Sprint 1's successful pattern, Sprint 3 is broken into focused sub-sprints for incremental testing, clear dependencies, and easier debugging. Each sub-sprint has detailed documentation with algorithms, examples, and test requirements.

---

### Sprint 04: Scenario Generation
**Status:** 📋 **PLANNED**  
**Documentation:** [Sprint_04_Scenario_Generation_Phases_Filters.md](sprints/Sprint_04_Scenario_Generation_Phases_Filters.md)

**Planned Work:**
- Random scenario generation
- Phase filters (EARLY, MID, LATE)
- Difficulty estimation
- Scenario validation
- Seed-based reproducibility

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
- ✅ Completed: 3 sprints (00, 01, 02)
  - Including 4 Sprint 01 sub-sprints (01A, 01B, 01C, 01D)
- 🚧 In Progress: 0 sprints
- 📋 Planned: 5 sprints (03, 04, 05, 06, 07)
  - Including 3 Sprint 03 sub-sprints (03A, 03B, 03C)

**Progress:**
- **37.5%** complete (3 of 8 major sprints)
- **Core engine:** 100% complete (Sprint 01: 4 sub-sprints)
- **Basic UI:** 100% complete (Sprint 02)
- **Game logic:** 0% (waiting on Sprint 03: 3 sub-sprints)
- **AI/Evaluation:** 0% (waiting on Sprint 05)

**Subdivision Approach:**
- Sprint 1 was subdivided into 4 focused sub-sprints ✅ Completed
- Sprint 3 is subdivided into 3 focused sub-sprints 📋 Planned
- This approach enables incremental validation and clearer dependencies

---

## Next Steps

**Immediate Priority:** Sprint 03 (End-of-Round Logic)

This is the natural next step as it completes the core game loop, enabling:
- Full game playthrough (not just draft moves)
- Scoring validation
- Multi-round gameplay
- Foundation for scenario generation (Sprint 04)

**Dependencies:**
- Sprint 03 has no blockers (all prerequisites complete)
  - Sub-sprint dependencies: 03A → 03B → 03C (sequential)
- Sprint 04 depends on Sprint 03 (needs full game states)
- Sprint 05 depends on Sprint 03 (needs complete game simulation)
- Sprint 06 depends on Sprint 05 (needs move evaluation for feedback)

**Sub-Sprint Details:**

**Sprint 03 Sub-Sprints (Planned):**
- 📋 **Sprint 03A:** Wall Tiling & Pattern Line Resolution
  - [Sprint_03A_Wall_Tiling_Pattern_Lines.md](sprints/Sprint_03A_Wall_Tiling_Pattern_Lines.md)
  - Focus: Mechanical tile movement from pattern lines to wall/lid
  - Estimated: 3-4 days
  
- 📋 **Sprint 03B:** Scoring System
  - [Sprint_03B_Scoring_System.md](sprints/Sprint_03B_Scoring_System.md)
  - Focus: Adjacency scoring and floor penalties with golden tests
  - Estimated: 4-5 days
  
- 📋 **Sprint 03C:** Round Transition & Refill
  - [Sprint_03C_Round_Transition_Refill.md](sprints/Sprint_03C_Round_Transition_Refill.md)
  - Focus: Bag/lid mechanics, game end detection, WASM integration
  - Estimated: 4-5 days

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
4. Open Dev Panel to inspect state

**Current capabilities:**
- ✅ Load game states
- ✅ Visualize board (factories, center, player boards)
- ✅ Select and apply draft moves
- ✅ Highlight legal moves
- ✅ Error handling
- ✅ Multi-move sequences
- ❌ End-of-round logic (Sprint 03)
- ❌ Full game completion (Sprint 03)
- ❌ Scenario generation (Sprint 04)
- ❌ Move evaluation (Sprint 05)
- ❌ Drag-and-drop (Sprint 06)
