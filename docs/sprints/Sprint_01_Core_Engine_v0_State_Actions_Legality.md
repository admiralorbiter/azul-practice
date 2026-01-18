# Sprint 1 — Core Engine v0: State, Actions, Legality, Apply (Draft Phase)

**Goal:** Engine can represent a 2-player draft state and correctly enumerate/apply draft actions with clear legality errors.

## Outcomes
- A versioned `State` + `Action` schema implemented in Rust
- Deterministic, testable core functions:
  - `list_legal_actions(state, playerId)`
  - `apply_action(state, action) -> nextState | error`

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
- [ ] Define Rust `State` and `Action` structs + serde
- [ ] Implement legality checks
- [ ] Implement apply_action transitions
- [ ] Implement `list_legal_actions`
- [ ] Add invariants + tests
- [ ] Expose WASM exports + basic JS bindings
