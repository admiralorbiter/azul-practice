# Sprint 5 — Best-Move Evaluator (Tier 2 Rollouts) + "Think Longer"

**Status:** 📋 **PLANNED** (Broken into 3 Sub-Sprints)  
**Goal:** App can compute best move using rollout-based evaluation under a time budget, and grade the user's move with a "Think longer" option.

---

## Sub-Sprint Structure

Sprint 5 has been broken into **3 focused sub-sprints** for clearer implementation:

### [Sprint 05A — Rollout Simulation Infrastructure](Sprint_05A_Rollout_Simulation.md)
**Goal:** Build the core rollout engine that simulates games to end-of-round using policies.

**Key Deliverables:**
- Rollout simulation function (`simulate_rollout()`)
- Integration with GreedyPolicy and RandomPolicy from Sprint 4
- Deterministic rollouts with seeded RNG
- Statistics collection (scores, actions taken, etc.)
- Comprehensive test coverage

**Complexity:** Medium

---

### [Sprint 05B — Evaluator Core + Action Shortlisting](Sprint_05B_Evaluator_Core.md)
**Goal:** Implement the complete evaluation engine with time budgeting and best-move selection.

**Key Deliverables:**
- Action shortlisting heuristic (reduce ~50 actions to top ~15-20)
- Time-budgeted evaluation loop with best-so-far return
- EV calculation via rollout sampling
- Best action selection and user action grading
- WASM API for JavaScript integration
- Performance optimizations (caching, early cutoff)

**Complexity:** High

---

### [Sprint 05C — Feedback System + UI Integration](Sprint_05C_Feedback_UI.md)
**Goal:** Add rich feedback, grading system, and complete UI integration.

**Key Deliverables:**
- Feature delta tracking (floor penalty, adjacency, completion, waste)
- Template-based feedback bullet generation (1-3 explanations)
- Grading system (Excellent/Good/Okay/Miss)
- Complete evaluation UI in PracticeScreen
- "Think Longer" time budget controls
- Results panel with grade, EV comparison, and feedback
- Dev panel diagnostics for evaluation

**Complexity:** Medium

---

## Overall Sprint Outcomes

After completing all three sub-sprints:

- ✓ Tier 2 evaluator with rollout-based Monte Carlo evaluation
- ✓ Action shortlisting for performance
- ✓ Time-budgeted evaluation (250/750/1500ms presets)
- ✓ Best move computation with EV calculation
- ✓ User move grading with delta EV
- ✓ Rich feedback with feature-based explanations
- ✓ Complete UI with "Think Longer" controls
- ✓ End-to-end practice loop: generate → play → evaluate → repeat

---

## Core Algorithm (High-Level)

### Tier 2 Evaluation

For each legal root action:
1. Apply action
2. Run K rollouts of the remaining drafting using seeded policy bots (Sprint 5A)
3. Resolve end-of-round and compute utility
4. EV = average utility across rollouts

**Utility (MVP):**
- `myScoreEndOfRound - oppScoreEndOfRound`

### Performance Controls

- **Budgeted evaluation:**
  - Default: 250ms
  - "Think longer" presets: 250/750/1500ms (configurable)
- Return best-so-far if budget expires
- Optional: Web Worker for long evaluations (if UI lags)

### Optimization Strategies

- **Action shortlisting:** Heuristic pre-score all actions, keep top N (Sprint 5B)
- **Policy reuse:** Leverage existing GreedyPolicy from Sprint 4 (Sprint 5A)
- **Deterministic seeds:** Reproducible evaluation (all sub-sprints)
- **Caching:** Cache legal actions within single evaluation (Sprint 5B)

---

## Output Contract

The evaluator returns:

```typescript
{
  bestAction: DraftAction,
  bestActionEv: number,
  userActionEv?: number,
  deltaEv?: number,
  grade?: "EXCELLENT" | "GOOD" | "OKAY" | "MISS",
  feedback?: FeedbackBullet[],
  bestFeatures: ActionFeatures,
  userFeatures?: ActionFeatures,
  metadata: {
    elapsedMs: number,
    rolloutsRun: number,
    candidatesEvaluated: number,
    totalLegalActions: number,
    seed: number,
    completedWithinBudget: boolean
  },
  candidates?: CandidateAction[]  // For dev panel
}
```

---

## Integration Points

### Cross-Sub-Sprint Dependencies

- **5A → 5B:** Rollout simulation used by evaluator core
- **5B → 5C:** Evaluation API used by feedback system and UI

### Cross-Sprint Dependencies

- **Sprint 3:** End-of-round resolution used in rollouts
- **Sprint 4:** Policy infrastructure (GreedyPolicy, RandomPolicy) and RNG
- **Sprint 4:** Scenario generation provides states to evaluate

---

## Acceptance Criteria

### Overall Sprint Success

- [ ] All three sub-sprints completed (5A, 5B, 5C)
- [ ] Evaluation completes within selected budget (or returns best-so-far)
- [ ] Results are reproducible given the same scenarioSeed and evaluatorSeed
- [ ] The chosen best move is stable and feels reasonable in practice scenarios
- [ ] Feedback explanations are accurate and helpful
- [ ] UI is responsive and intuitive
- [ ] Performance: 250ms default budget achievable

### Per-Sub-Sprint Criteria

See individual sub-sprint documents for detailed acceptance criteria.

---

## Testing Strategy

### Sprint 5A Tests
- Rollout completes from various game states
- Deterministic rollouts with fixed seed
- Tile conservation throughout simulation
- Integration with end-of-round resolution

### Sprint 5B Tests
- Action shortlisting reduces candidate set
- Time budget respected
- EV calculation correctness
- Deterministic evaluation results
- WASM API round-trip

### Sprint 5C Tests
- Feature delta calculation
- Feedback template selection
- Grading thresholds
- UI component rendering
- End-to-end practice loop

---

## Demo (End-of-Sprint)

After completing Sprint 5:

1. **Generate a scenario** (using Sprint 4 generator)
2. **Make a move** (using existing UI from Sprint 2)
3. **Evaluate at 250ms** → See grade, best move, feedback
4. **Try "Think Longer" (1500ms)** → Compare confidence/stability
5. **Click "Next Scenario"** → Repeat practice loop

**Success Metrics:**
- Evaluation feels fast (<300ms perceived)
- Best moves seem reasonable to experienced players
- Feedback is actionable and specific
- Practice loop flows smoothly

---

## Dependencies

**Completed:**
- ✅ Sprint 3: End-of-round scoring
- ✅ Sprint 4: Scenario generation + policy infrastructure

**Required For:**
- Sprint 6: Feedback and polish (uses evaluation results)
- Sprint 7: Content and calibration (uses evaluation for tuning)

---

## Implementation Approach

### Recommended Order

1. **Sprint 05A** (Foundation)
   - Build rollout engine
   - Test with various game states
   - Ensure determinism and correctness

2. **Sprint 05B** (Core Algorithm)
   - Implement evaluator with shortlisting
   - Add time budgeting
   - Export WASM API
   - Profile performance

3. **Sprint 05C** (User Experience)
   - Add feature tracking
   - Generate feedback
   - Build UI components
   - Integrate end-to-end

### Estimated Timeline

- **5A:** ~2-3 focused sessions (rollout engine + tests)
- **5B:** ~3-4 sessions (evaluator core + performance + WASM)
- **5C:** ~2-3 sessions (feedback + UI integration)

**Total:** Similar complexity to Sprint 3 (which also had 3 sub-sprints: 03A, 03B, 03C)

---

## Research Alignment

This implementation follows the **hybrid approach** recommended in the research synthesis:

✅ **Quality rollouts** using GreedyPolicy (not just random) → 5A  
✅ **Action shortlisting** with heuristic pre-scoring → 5B  
✅ **Feature delta tracking** for explanations → 5C  
✅ **Performance optimizations** with time budgeting → 5B  
✅ **Template-based feedback** with measurable features → 5C

See [Research Synthesis](../engineering/azul_best_move_algorithm_research_synthesis.md) for detailed rationale.

---

## Related Documentation

- **Sub-Sprint Details:**
  - [Sprint 05A: Rollout Simulation](Sprint_05A_Rollout_Simulation.md)
  - [Sprint 05B: Evaluator Core](Sprint_05B_Evaluator_Core.md)
  - [Sprint 05C: Feedback + UI](Sprint_05C_Feedback_UI.md)

- **Specifications:**
  - [Best-Move Evaluation & Feedback Spec](../specs/08_best_move_evaluation_and_feedback.md)
  - [Research Synthesis](../engineering/azul_best_move_algorithm_research_synthesis.md)

- **Dependencies:**
  - [Sprint 03: End-of-Round Resolution](Sprint_03C_COMPLETED.md)
  - [Sprint 04: Scenario Generation](Sprint_04_COMPLETED.md)

---

## Next Steps

**To Begin Sprint 5:**

1. Start with [Sprint 05A: Rollout Simulation](Sprint_05A_Rollout_Simulation.md)
2. Build and test rollout infrastructure
3. Proceed sequentially through 5B and 5C
4. Test end-to-end practice loop after 5C

**After Sprint 5:**

- Consider Sprint 06 for polish (drag-drop, animations, accessibility)
- Or Sprint 07 for calibration and advanced features
- The MVP core loop will be fully functional!
