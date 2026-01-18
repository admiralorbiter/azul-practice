# State / Action Model & Serialization (v0)

## Goals
- Stable and versioned schema for:
  - Rust engine internals
  - WASM boundary
  - scenario packs
  - replay/debug dumps

## Versioning
- `stateVersion`: integer
- `rulesetId`: string (e.g., `azul_v1_2p`)

## State (conceptual)
A minimal state must include:

### Global
- `stateVersion`
- `rulesetId`
- `scenarioSeed`
- `activePlayerId` (0 or 1)
- `roundNumber`
- `draftPhaseProgress` (`EARLY` | `MID` | `LATE`) — label/tag for scenario selection

### Supply
- `bag`: multiset of tiles by color
- `lid`: multiset of tiles by color

### Table
- `factories[]`: each is multiset of tiles by color
- `center`: multiset of tiles by color + first-player token present

### Players (for each player)
- `score`
- `patternLines[5]`: each has
  - `capacity`
  - `color` (optional)
  - `countFilled`
- `wall[5][5]`: filled/unfilled (may store color mapping explicitly)
- `floorLine`: ordered slots or counts + token state

---

## Actions
### DraftAction (MVP)
A single drafting move:
- `source`: `Factory(index)` | `Center`
- `color`: `TileColor`
- `dest`: `PatternLine(rowIndex)` | `Floor`

---

## Serialization
### Format
- MVP: JSON (readable + easy debugging)
- Optional future: binary encoding (MessagePack/Protobuf)

### Determinism notes
- Include `scenarioSeed` in each saved scenario.
- Evaluation rollouts should be seeded deterministically from scenario seed + action.

---

## Invariants (must hold)
- Tile conservation: bag+lid+factories+center+playerboards totals remain consistent.
- No invalid pattern line states (mixed colors, overfilled, illegal color for row).
- Wall placements follow rules.

---

## Example JSON (placeholder)
> Add a concrete example once the first engine types are drafted.
