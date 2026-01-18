# Sprint 1 — Core Engine v0: State, Actions, Legality, Apply (Draft Phase)

**Status:** ✅ **COMPLETED**  
**Completion Date:** Prior to Sprint 02  
**All Sub-Sprints:** ✅ 01A, 01B, 01C, 01D Complete

> **Note:** This sprint has been subdivided into 4 focused sub-sprints for better organization and incremental implementation. See the [Subdivision Overview](#subdivision-overview) section below.

**Goal:** Engine can represent a 2-player draft state and correctly enumerate/apply draft actions with clear legality errors.

## Outcomes
- A versioned `State` + `Action` schema implemented in Rust
- Deterministic, testable core functions:
  - `list_legal_actions(state, playerId)`
  - `apply_action(state, action) -> nextState | error`

---

## Subdivision Overview

This sprint has been subdivided into **four focused sub-sprints** for better organization, clearer dependencies, and incremental validation:

1. **[Sprint 01A: Data Models & Serialization](Sprint_01A_Data_Models_Serialization.md)**
   - Define Rust structs for State, Action, PlayerBoard, etc.
   - Implement JSON serialization with serde
   - Ensure round-trip stability and version tracking

2. **[Sprint 01B: Rules & Legality Checks](Sprint_01B_Rules_Legality_Checks.md)**
   - Implement `list_legal_actions` function
   - Enforce pattern line and wall constraints
   - Handle all edge cases for draft move validation

3. **[Sprint 01C: Action Application & State Transitions](Sprint_01C_Apply_Action_Transitions.md)**
   - Implement `apply_action` with state mutations
   - Handle tile overflow and first-player token transfers
   - Implement tile conservation invariants

4. **[Sprint 01D: WASM Integration & Tests](Sprint_01D_WASM_Integration_Tests.md)**
   - Expose WASM exports with error handling
   - Create TypeScript wrapper and types
   - Build demo UI and integration tests

### Subdivision Rationale

The original Sprint 01 scope is substantial and involves several interconnected components. Subdividing provides:

- **Clearer dependencies:** Each sub-sprint builds on the previous one
- **Incremental validation:** Test thoroughly at each stage before moving forward
- **Better planning:** More detailed specifications for each focused task
- **Easier debugging:** Isolate issues to specific layers (models vs rules vs WASM)
- **Parallelizable documentation:** Can write detailed implementation plans per sub-sprint

### Implementation Order

The sub-sprints must be completed sequentially due to dependencies:

```
Sprint 01A (Data Models)
    ↓
Sprint 01B (Legality) ← depends on State/Action types
    ↓
Sprint 01C (Apply Action) ← depends on legality checks
    ↓
Sprint 01D (WASM) ← depends on full engine implementation
```

### Integration Points

- **1A → 1B:** State and Action types are used by legality functions
- **1B → 1C:** Legality checks are reused for validation in `apply_action`
- **1C → 1D:** Complete engine functions are wrapped for WASM boundary
- **All → Tests:** Each sub-sprint includes essential tests that build on each other

---

## Scope
### 1) Data model (Rust)
- `State` (2-player):
  - factories, center (incl. first-player token)
  - bag/lid (tile multisets)
  - player boards: pattern lines, wall, floor, score
  - active player id
  - `stateVersion`, `rulesetId`, `scenarioSeed`
- Serialization:
  - JSON encode/decode round-trippable
  - stable field naming and versioning

### 2) Action model
- DraftAction:
  - source (factory index or center)
  - color
  - destination (pattern line row or floor)

### 3) Rules enforcement (draft move)
- Correct legal move checks (pattern line constraints, wall color constraints, etc.)
- Correct application of rules:
  - remove tiles from chosen source
  - move remaining factory tiles to center (if source was factory)
  - place tiles into row/floor with overflow behavior
  - handle first-player token when taking from center

### 4) Error model
- Validation errors are structured:
  - error code
  - human-readable message
  - optional context (row index, color, etc.)

## Deliverables
- WASM exports:
  - `list_legal_actions(stateJson, playerId) -> actionsJson`
  - `apply_action(stateJson, actionJson) -> nextStateJson | errorJson`
- Rust unit tests:
  - legality edge cases
  - apply_action state diff checks
  - tile conservation invariants

## Acceptance Criteria
- For a fixed test state:
  - Legal action enumeration is stable and matches expectations
  - Illegal actions are rejected with correct error messages
- JSON round-trip does not lose information (`state -> json -> state` stable)
- Tile conservation checks pass (no tiles created/destroyed)

## Demo (end-of-sprint)
- In dev UI: load a hard-coded scenario
- Click “Show legal actions count”
- Apply an action and see new state (raw JSON view is fine)

## Dependencies
- Sprint 0 WASM pipeline

## Sprint Backlog (Suggested Tasks)

The detailed tasks for this sprint are organized within the sub-sprint documents:

### Sprint 01A Tasks ✅ COMPLETED
- [x] Define Rust `State` and `Action` structs + serde
- [x] Implement JSON serialization with round-trip tests
- [x] Document all data structures with examples
- See [Sprint_01A_Data_Models_Serialization.md](Sprint_01A_Data_Models_Serialization.md) for details

### Sprint 01B Tasks ✅ COMPLETED
- [x] Implement `list_legal_actions` function
- [x] Implement legality checks for pattern lines and walls
- [x] Add edge case tests for all constraint types
- See [Sprint_01B_Rules_Legality_Checks.md](Sprint_01B_Rules_Legality_Checks.md) for details

### Sprint 01C Tasks ✅ COMPLETED
- [x] Implement `apply_action` state transitions
- [x] Handle tile overflow and first-player token logic
- [x] Add invariants (tile conservation) and state transition tests
- See [Sprint_01C_Apply_Action_Transitions.md](Sprint_01C_Apply_Action_Transitions.md) for details

### Sprint 01D Tasks ✅ COMPLETED
- [x] Expose WASM exports with error handling
- [x] Create TypeScript wrappers and type definitions
- [x] Build demo UI component
- [x] Add integration tests
- See [Sprint_01D_WASM_Integration_Tests.md](Sprint_01D_WASM_Integration_Tests.md) for details
