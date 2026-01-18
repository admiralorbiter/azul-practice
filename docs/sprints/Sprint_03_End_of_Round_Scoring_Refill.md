# Sprint 3 — End-of-Round + Scoring + Refill Rules

**Goal:** Engine can resolve end-of-round deterministically and score correctly, enabling realistic scenario generation and rollout evaluation later.

## Outcomes
- Deterministic end-of-round resolution
- Trusted scoring via golden tests
- Correct bag/lid refill behavior for 2-player setup

## Scope
### 1) End-of-round resolution
- Move completed pattern lines → place 1 tile on wall + rest to lid
- Apply adjacency scoring for placed tiles
- Apply floor penalties
- Determine next-round first player based on token

### 2) Refill / setup mechanics
- Factory refill for new round from bag
- If bag insufficient, refill from lid (rules-correct)
- Define behavior when both insufficient (should still be rules-consistent)

### 3) Testing (high leverage)
- Golden tests:
  - Given fixed wall placements, expected scoring totals
  - Known edge cases for adjacency scoring
- Invariants:
  - Tile conservation including lid/bag transfers

## Deliverables
- Engine function:
  - `resolve_end_of_round(state) -> nextState`
- Golden test suite for scoring
- Optional UI button for dev: “Resolve end-of-round” (for testing)

## Acceptance Criteria
- Scoring matches expected values in golden tests
- End-of-round transformations are deterministic and correct
- Refill logic produces valid next-round states

## Demo (end-of-sprint)
- Use UI to play until drafting ends (or load a near-end state)
- Trigger end-of-round resolution
- Verify score changes + wall placements

## Dependencies
- Sprint 1 engine core

## Sprint Backlog (Suggested Tasks)
- [ ] Implement wall placement logic
- [ ] Implement adjacency scoring + bonuses (as per rules spec)
- [ ] Implement floor penalties
- [ ] Implement bag/lid refill routines
- [ ] Build golden test cases for scoring
