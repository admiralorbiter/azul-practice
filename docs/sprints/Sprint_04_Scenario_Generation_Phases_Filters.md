# Sprint 4 — Scenario Generation (Early/Mid/Late) + Quality Filters

**Status:** ✅ **COMPLETED** (January 19, 2026)  
**Goal:** "New Scenario" produces plausible, reproducible 2-player states for early/mid/late draft with basic quality guarantees.

> **📋 See [Sprint_04_COMPLETED.md](Sprint_04_COMPLETED.md) for detailed completion report including the critical multi-round generation fix.**

## Outcomes
- ✅ Scenario generator that produces legal states by playing forward
- ✅ Phase targeting: EARLY / MID / LATE with realistic progression
- ✅ Quality filters that reject low-signal scenarios
- ✅ Multi-round simulation (Mid: Round 2, Late: Round 3)
- ✅ Walls appropriately filled for Mid/Late phases

## Scope
### 1) Generator strategy (multi-round play-forward)
- Start from legal round start (2p)
- Use fast policy bots to draft forward
- **Complete full rounds** to fill walls (critical fix!)
- Early: 0 rounds complete + 3-8 picks in round 1
- Mid: 1 round complete + 3-10 picks in round 2
- Late: 2 rounds complete + 2-8 picks in round 3
- Tag scenario phase based on tile depletion

### 2) Filters (keep scenarios "good")
Reject scenarios that are:
- forced (too few legal moves)
- degenerate (most actions dump to floor similarly)
- **Hard fallback:** Returns best available state if filters can't be satisfied

### 3) Reproducibility
- `scenarioSeed` stored in state
- Generator and policies are seeded so scenarios can be replayed exactly
- Deterministic tile drawing and action enumeration

### 4) UI integration
- "New Scenario" button
- Phase selection dropdown (EARLY/MID/LATE)
- Show seed + phase + round in dev panel with copy button

## Deliverables
- ✅ Engine export: `generate_scenario(paramsJson) -> stateJson`
- ✅ UI wiring for scenario generation + phase selection
- ✅ Tests: 23 new tests (123 total passing)
- ✅ Distribution validation (walls filled for Mid/Late)

## Acceptance Criteria
- ✅ 100 generated scenarios are valid and playable
- ✅ Phase selection works (early feels early, late feels late)
  - **Early:** Round 1, empty walls, 14-20 tiles in play
  - **Mid:** Round 2, walls filled, 7-13 tiles in play, non-zero scores
  - **Late:** Round 3, walls more filled, 0-6 tiles in play, higher scores
- ✅ A scenario can be shared/replayed by seed

## Demo (end-of-sprint)
- ✅ Generate 5 early, 5 mid, 5 late scenarios
- ✅ For each, verify it feels appropriate and has meaningful choices
- ✅ Verify walls are filled appropriately for phase

## Dependencies
- ✅ Sprint 3 end-of-round/refill rules (for realism and multi-round simulation)

## Sprint Backlog (Completed Tasks)
- ✅ Implement seeded RNG utilities in Rust core
- ✅ Implement simple policy bots for play-forward generation
- ✅ Implement phase tagging based on tile depletion
- ✅ Implement filters + thresholds with fallback mechanism
- ✅ Wire generator to UI
- ✅ Fix determinism issues (HashMap iteration order)
- ✅ Implement multi-round generation for wall filling
- ✅ Add comprehensive test coverage (23 new tests)
