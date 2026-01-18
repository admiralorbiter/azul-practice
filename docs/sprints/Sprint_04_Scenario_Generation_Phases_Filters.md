# Sprint 4 — Scenario Generation (Early/Mid/Late) + Quality Filters

**Goal:** “New Scenario” produces plausible, reproducible 2-player states for early/mid/late draft with basic quality guarantees.

## Outcomes
- Scenario generator that produces legal states by playing forward
- Phase targeting: EARLY / MID / LATE
- Quality filters that reject low-signal scenarios

## Scope
### 1) Generator strategy (play-forward)
- Start from legal round start (2p)
- Use fast policy bots to draft forward for N picks
- Stop at active player turn
- Tag scenario phase based on progress:
  - EARLY: 0–2 picks (or equivalent)
  - MID: mid drafting
  - LATE: near end of drafting

### 2) Filters (keep scenarios “good”)
Reject scenarios that are:
- forced (too few legal moves)
- degenerate (most actions dump to floor similarly)
- low-signal (best-vs-median gap below threshold — initially heuristic-based)

### 3) Reproducibility
- `scenarioSeed` stored in state
- Generator and policies are seeded so scenarios can be replayed exactly

### 4) UI integration
- “New Scenario” button
- Phase selection:
  - dropdown (EARLY/MID/LATE) or weighted random
- Show seed + phase in dev panel

## Deliverables
- Engine export:
  - `generate_scenario(paramsJson) -> stateJson`
- UI wiring for scenario generation + phase selection
- Tests:
  - generate 100 scenarios without invalid state
  - distribution sanity check (phase tags correct)

## Acceptance Criteria
- 100 generated scenarios are valid and playable
- Phase selection works (early feels early, late feels late)
- A scenario can be shared/replayed by seed

## Demo (end-of-sprint)
- Generate 5 early, 5 mid, 5 late scenarios
- For each, verify it feels appropriate and has meaningful choices

## Dependencies
- Sprint 3 end-of-round/refill rules (for realism and later rollouts)

## Sprint Backlog (Suggested Tasks)
- [ ] Implement seeded RNG utilities in Rust core
- [ ] Implement simple policy bots for play-forward generation
- [ ] Implement phase tagging
- [ ] Implement filters + thresholds (initial)
- [ ] Wire generator to UI
