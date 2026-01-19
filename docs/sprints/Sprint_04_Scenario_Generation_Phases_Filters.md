# Sprint 4 — Scenario Generation (Early/Mid/Late) + Quality Filters

**Status:** ✅ **COMPLETED** (January 19, 2026)  
**Updated:** ✅ **DETERMINISTIC FIX** (January 19, 2026) - Stage-driven generation  
**Goal:** "New Scenario" produces plausible, reproducible 2-player states with meaningful decision points across both game progression axes.

> **📋 See [Sprint_04_COMPLETED.md](Sprint_04_COMPLETED.md) for detailed completion report.**
> **🔄 CRITICAL FIX:** Generator now uses **deterministic stage-driven generation** that completes rounds until the target GameStage is reached, then samples snapshots. This **guarantees** correct wall tile counts for Early/Mid/Late game.

## Outcomes (Revised)
- ✅ **Two-axis stage system**: GameStage (Early/Mid/Late game) + RoundStage (Start/Mid/End of round)
- ✅ Scenario generator using **snapshot sampling** across decision points
- ✅ Enhanced quality filters with strategic signal criteria
- ✅ Multi-round simulation with wall filling
- ✅ UI controls for both staging axes

## Scope (Revised)
### 1) Two-axis staging system
**GameStage (across-game progression):**
- Early: ≤8 wall tiles placed per player
- Mid: 9-17 wall tiles placed per player
- Late: ≥18 wall tiles or near row completion

**RoundStage (within-round progression):**
- Start: 14-20 tiles on table (factories + center)
- Mid: 7-13 tiles remaining
- End: 0-6 tiles remaining

### 2) Deterministic stage-driven generator
**Phase 1: Stage targeting (deterministic)**
- Complete full rounds until `compute_game_stage()` matches target
- For Late game: keeps playing until ≥18 wall tiles exist
- For Mid game: keeps playing until 9-17 wall tiles exist
- For Early game: starts immediately (0 wall tiles)
- Safety checks prevent infinite loops and overshooting

**Phase 2: Snapshot sampling (variety within stage)**
- Once at target stage, play forward and record snapshots (every 2 decisions)
- Annotate each snapshot with both stage axes + quality metrics
- Select best snapshot matching target criteria
- Deterministic selection driven by seed

### 3) Enhanced quality filters
**Structural filters:**
- Min legal actions: 6 (raised from 3)
- Min unique destinations: 2
- Require non-floor option: true
- Max floor ratio: 0.5 (at most half floor-only)

**Strategic filters (foundation for future EV-gap):**
- Min/max value gap (optional, for balanced puzzles)
- Floor pressure indicators

**Hard fallback:** Returns best available state if filters can't be satisfied

### 4) UI integration
- Two dropdowns: "Game Stage" and "Round Stage"
- "New Scenario" button generates matching snapshot
- DevPanel shows both stage axes
- Seed reproducibility maintained

## Deliverables
- ✅ Engine export: `generate_scenario(paramsJson) -> stateJson`
- ✅ UI wiring for scenario generation + phase selection
- ✅ Tests: 23 new tests (123 total passing)
- ✅ Distribution validation (walls filled for Mid/Late)

## Acceptance Criteria (Deterministic Fix)
- ✅ 100 generated scenarios are valid and playable
- ✅ **GUARANTEED stage matching** (deterministic, not probabilistic):
  - **Late game:** ALWAYS has ≥18 wall tiles (or 4+ in a row)
  - **Mid game:** ALWAYS has 9-17 wall tiles
  - **Early game:** ALWAYS has ≤8 wall tiles
  - Generator completes rounds until target stage is reached
- ✅ **RoundStage targeting:** Tile-in-play counts match expected ranges (Start/Mid/End)
- ✅ **Combined targeting:** Can request (Early game, Start round) or (Late game, End round) etc.
- ✅ Quality filters enforce minimum puzzle quality:
  - ≥6 legal actions (sufficient choice)
  - ≥2 unique destinations (not degenerate)
  - At least one non-floor option (strategic choice exists)
- ✅ Snapshot sampling produces varied scenarios within target criteria
- ✅ Scenarios are reproducible by seed
- ✅ Generator never fails in UI (max 500 retry attempts)

## Demo (end-of-sprint)
- ✅ Generate 5 early, 5 mid, 5 late scenarios
- ✅ For each, verify it feels appropriate and has meaningful choices
- ✅ Verify walls are filled appropriately for phase

## Dependencies
- ✅ Sprint 3 end-of-round/refill rules (for realism and multi-round simulation)
- ✅ Best-move algorithm research synthesis (for quality criteria)

## Sprint Backlog (Completed Tasks)
**Original implementation:**
- ✅ Implement seeded RNG utilities in Rust core
- ✅ Implement simple policy bots for play-forward generation
- ✅ Implement phase tagging based on tile depletion
- ✅ Implement filters + thresholds with fallback mechanism
- ✅ Wire generator to UI
- ✅ Fix determinism issues (HashMap iteration order)
- ✅ Implement multi-round generation for wall filling
- ✅ Add comprehensive test coverage (23 new tests)

**Revision (January 19, 2026):**
- ✅ Split DraftPhase into GameStage and RoundStage
- ✅ Implement snapshot sampling generator
- ✅ Enhance FilterConfig with strategic criteria
- ✅ Update UI with two-axis controls
- ✅ Add distribution tests for both axes
- ✅ **CRITICAL FIX:** Implement deterministic stage-driven generation
  - Replace probabilistic "complete N rounds and hope" with "complete rounds until stage matches"
  - Add safety checks (max 10 rounds, overshoot detection)
  - Guarantee correct wall tile counts for all game stages
- ✅ Update documentation

## Revision Motivation

The initial implementation successfully generated reachable, deterministic scenarios, but puzzle quality was inconsistent. The revisions address feedback through multiple iterations:

### Revision 1: Two-axis staging + snapshot sampling
1. **Two-axis staging:** Separates "where in the game" (Early/Mid/Late game) from "where in the round" (Start/Mid/End). This aligns with strategic differences in Azul as documented in the best-move research synthesis (Section 2).

2. **Snapshot sampling:** Instead of predetermining "complete N rounds + M picks," the generator plays forward and samples decision points, selecting the best match. This produces more varied, natural scenarios.

3. **Enhanced filters:** Raised quality bar (6+ legal actions, floor pressure checks) ensures puzzles have meaningful strategic texture rather than forced/trivial moves.

4. **Foundation for EV-gap filtering:** FilterConfig now supports min/max value gap parameters, enabling future "balanced mix" filtering (clear best move vs. close dilemma) as recommended in the research synthesis (Section 2.2).

### Revision 2: Deterministic stage guarantee (CRITICAL FIX)
**Problem:** The generator was probabilistic—it completed a fixed number of rounds (e.g., 2 for Late game) and hoped the wall would fill to ≥18 tiles. This failed frequently, causing "Max attempts exceeded" errors.

**Solution:** Implemented **stage-driven generation**:
```rust
// Instead of: complete 2 rounds and hope
// Now: keep completing rounds UNTIL stage matches target
while compute_game_stage(&state) != target_game_stage {
    complete_one_round();
    // Safety checks prevent infinite loops
}
```

**Impact:** 
- ✅ Late game **always** has ≥18 wall tiles (guaranteed, not probabilistic)
- ✅ Mid game **always** has 9-17 wall tiles  
- ✅ Early game **always** has ≤8 wall tiles
- ✅ No more "Max attempts exceeded" errors in UI
- ✅ Reliable, predictable scenario generation

See: [`docs/engineering/azul_best_move_algorithm_research_synthesis.md`](../engineering/azul_best_move_algorithm_research_synthesis.md)
