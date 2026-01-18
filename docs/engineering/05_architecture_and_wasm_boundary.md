# Architecture & WASM Boundary (Rust → Web)

## Architecture overview
- **Rust core engine** implements the authoritative Azul rules and evaluation.
- The web UI (TypeScript/React) renders state and collects user intent.
- Rust is compiled to **WebAssembly**, exposing a small API surface.

## Modules
### Rust
- `model` — state and action types
- `rules` — legality + state transitions
- `scoring` — scoring logic
- `scenario` — scenario generation and filtering
- `eval` — best-move evaluation (Tier 2 rollouts)
- `util` — RNG, hashing, logging

### Web UI
- `ui` — screens + components
- `state` — app UI state (selected tiles, drag state)
- `wasm` — boundary wrappers and type mapping

---

## WASM API (v0 contract)
All inputs/outputs are serialized JSON (v0). Later we can introduce binary without changing semantics.

### Introspection
- `get_version() -> { engineVersion, stateVersion, rulesetId }`

### Core engine
- `list_legal_actions(stateJson, playerId) -> actionsJson`
- `apply_action(stateJson, actionJson) -> nextStateJson | errorJson`

### Scenario generation
- `generate_scenario(paramsJson) -> stateJson`

### Evaluation
- `evaluate_best_move(stateJson, playerId, paramsJson) -> resultJson`

---

## Error model
- `ValidationError`: returned when a user attempts an illegal move.
  - includes `code` + human-readable `message`
- `InvariantViolation`: indicates engine bug or corrupted state.
  - in dev mode, this should be loud (panic/assert)

---

## Logging & debugging
- Dev mode enables:
  - structured Rust logs forwarded to JS console
  - deterministic RNG seeding
  - ability to dump/copy state JSON
  - evaluation diagnostics (rollouts, elapsed time, top-N candidates)

---

## Extensibility notes
- Stable `State/Action` schemas enable:
  - future bot services (remote)
  - scenario packs
  - replay tools
- Keeping the boundary minimal avoids UI-engine entanglement.
