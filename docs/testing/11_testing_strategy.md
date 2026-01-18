# Testing Strategy

## Goals
- Prevent rule mistakes and regressions.
- Ensure deterministic outcomes.
- Validate scoring and legality across edge cases.

## Rust tests
### Unit tests
- Legal move checks (table-driven)
- apply_action state transitions
- scoring functions

### Golden tests
For fixed scenario JSON:
- legal action count
- expected best move under fixed budget/seed
- expected EV delta ranges

### Invariant / property tests (later)
- tile conservation
- no illegal pattern line states
- state hash stability (if used)

## JS/WASM integration tests
- JSON boundary roundtrip
- apply_action from UI wiring
- evaluate_best_move call + result parsing

## Manual QA checklist (MVP)
- Drag/drop feel
- Illegal moves blocked with clear reasons
- Think longer budgets work and remain responsive
