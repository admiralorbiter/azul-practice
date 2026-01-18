# Sprint 5 — Best-Move Evaluator (Tier 2 Rollouts) + “Think Longer”

**Goal:** App can compute best move using rollout-based evaluation under a time budget, and grade the user’s move with a “Think longer” option.

## Outcomes
- Tier 2 evaluator:
  - shortlist candidate root actions
  - run seeded rollouts to end-of-round
  - return best action, EVs, and metadata
- UI supports “Evaluate” + “Think longer” budget control

## Scope
### 1) Evaluator design (Tier 2)
For each legal root action:
1. Apply action.
2. Run K rollouts of the remaining drafting using seeded policy bots.
3. Resolve end-of-round and compute utility.
4. EV = average utility across rollouts.

**Utility (MVP):**
- `myScoreEndOfRound - oppScoreEndOfRound`

### 2) Performance controls
- Budgeted evaluation:
  - default 250ms
  - “Think longer” presets: 250/750/1500ms (configurable)
- Return best-so-far if budget expires
- Optional: move evaluation to Web Worker if main-thread stalls

### 3) Pruning / acceleration
- Heuristic pre-score all actions quickly
- Keep top N (e.g., 10–25) for rollouts
- Cache legal actions and intermediate hashes within a single evaluation

### 4) Output contract
Return:
- `bestAction`
- `userActionEV`, `bestActionEV`, `deltaEV`
- `metadata`: elapsedMs, rolloutsRun, candidatesEvaluated, seed

### 5) UI integration
- “Submit/Evaluate” button
- Budget selector (“Think longer”)
- Display:
  - best move representation
  - delta EV
  - diagnostic stats (dev panel)

## Deliverables
- Engine export:
  - `evaluate_best_move(stateJson, playerId, paramsJson) -> resultJson`
- UI wiring for evaluation and time budget control
- Tests:
  - deterministic results under fixed seed/budget
  - regression tests for known scenarios

## Acceptance Criteria
- Evaluation completes within selected budget (or returns best-so-far)
- Results are reproducible given the same scenarioSeed and evaluatorSeed
- The chosen best move is stable and feels reasonable in practice scenarios

## Demo (end-of-sprint)
- Generate a scenario
- Make a move, evaluate at 250ms
- Evaluate again at 1500ms and compare (should be same or improved EV confidence)
- Show best action overlay (even if basic)

## Dependencies
- Sprint 4 scenario generation
- Sprint 3 end-of-round scoring

## Sprint Backlog (Suggested Tasks)
- [ ] Implement action shortlisting heuristic
- [ ] Implement rollout simulation policies
- [ ] Implement time-budget loop with best-so-far return
- [ ] Wire evaluator to UI + budget control
- [ ] Add deterministic seed plumbing for evaluation
