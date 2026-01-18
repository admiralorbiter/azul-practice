# Best-Move Evaluation & Feedback (Tier 2 Rollouts)

## Goal
Compute a “best move” for a scenario, grade the user’s move, and generate short, explainable feedback.

## Definitions
- `EV(action)`: expected utility of an action estimated via rollouts.
- `utility`: MVP scalar objective.

## Utility (MVP)
Default:
- `utility = myScoreEndOfRound - oppScoreEndOfRound`

(We can later experiment with alternative utility formulations.)

---

## Tier 2 evaluation (rollout-based)
### High-level algorithm
1. Enumerate legal root actions.
2. Optionally score all actions with a cheap heuristic and keep the top N.
3. For each candidate action:
   - apply it
   - run rollouts to end of drafting
   - apply deterministic end-of-round scoring
   - record utility
4. Return the action with highest EV.

### Rollout policy (MVP)
- Both players choose actions using a fast, greedy-ish policy with tie-break randomness.
- Rollouts are seeded deterministically for reproducibility.

### Time budgets
- Default “fast”: ~250ms (tunable)
- “Think longer”: e.g., 750ms / 1500ms (tunable)
- If time expires: return best-so-far with diagnostics.

### Optimizations
- Root shortlist (top N by heuristic)
- Early cutoff for dominated actions
- Cache legal moves per state hash within evaluation

---

## Result schema (conceptual)
- `bestAction`
- `userAction`
- `EV_best`, `EV_user`, `delta`
- `grade`
- `diagnostics`:
  - elapsed time
  - rollouts
  - candidate list (optional dev)

---

## Grading
- Map `delta` to buckets. Example (placeholder):
  - `delta <= 0.25` → Excellent
  - `0.25 < delta <= 1.0` → Good
  - `1.0 < delta <= 2.5` → Okay
  - `delta > 2.5` → Miss

(Exact thresholds to be tuned.)

---

## Feedback (template-based, MVP-feasible)
Feedback bullets are selected from a fixed set using measurable feature deltas between best vs user.

### Candidate bullet templates
- Floor penalty: “Best move reduces expected floor penalty by ~X points.”
- Line completion: “Best move increases chance to complete a pattern line this round.”
- Wasted tiles: “Your move sends ~X more tiles to the floor than best.”
- Wall adjacency: “Best move more often creates adjacency scoring on the wall.”
- First player token: “Best move more often secures/avoids first-player token.”

### Selection rule
- Compute feature stats during rollouts.
- Choose 1–3 bullets with largest differences.
