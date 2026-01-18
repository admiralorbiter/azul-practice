# MVP Requirements (PRD)

## Summary
MVP = **best-move practice loop**.

1) generate scenario → 2) user makes a drafting move → 3) engine grades vs best move → 4) show feedback → 5) repeat.

## Definitions
- **Scenario:** A valid 2-player Azul state at an active player’s turn.
- **Move/Action:** A single legal drafting action (take tiles from source and place to a pattern line or floor).
- **Best move:** The action with highest estimated utility via Tier 2 rollouts under a time budget.

---

## Functional requirements

### Core loop
**FR-01** The system SHALL generate a valid 2-player scenario on demand.

**FR-02** The UI SHALL render the full scenario state needed to take a turn:
- factories
- center
- both players’ boards (at least compressed opponent)
- score, floor line, pattern lines, wall

**FR-03** The user SHALL be able to perform a drafting action via drag/drop (with click fallback).

**FR-04** The engine SHALL validate actions and:
- apply legal actions deterministically
- reject illegal actions with an explicit error reason

**FR-05** The system SHALL compute and display a best move and grade the user’s move.

**FR-06** The system SHALL provide feedback:
- always: best move + user move + expected value delta
- optionally: 1–3 explanation bullets from a predefined template set

**FR-07** The system SHALL allow the user to immediately request the next scenario.

### Scenario timing
**FR-08** The generator SHALL support scenario timing targets:
- `EARLY` (near start of drafting)
- `MID` (mid drafting)
- `LATE` (late drafting)

### Performance UX
**FR-09** The UI SHALL include a “Think longer” control that increases evaluation time budget.

**FR-10** The default evaluation SHALL feel fast; longer evaluation budgets are opt-in.

### Debug/observability
**FR-11** Dev mode SHALL expose:
- scenario seed
- evaluation elapsed time
- rollout counts
- top candidate actions with EVs
- ability to copy scenario JSON

---

## Non-functional requirements

### Compatibility
**NFR-01** MVP SHALL target modern desktop browsers.

### Determinism & reproducibility
**NFR-02** Given the same scenario seed and evaluation settings, the engine SHALL produce reproducible results (for debugging/testing).

### Performance
**NFR-03** Default evaluation budget: ~250ms (tunable).

**NFR-04** “Think longer” budgets: configurable options (e.g., 250ms / 750ms / 1500ms).

**NFR-05** The UI SHALL remain responsive during evaluation (plan for worker/off-main-thread execution if needed).

### Quality
**NFR-06** The engine SHALL have test coverage for legality, state transitions, and scoring.

---

## MVP out of scope
- multiplayer
- accounts/cloud
- full match play UX
- deep explanation generation beyond template-based feedback

---

## Open questions (acceptable to defer)
- Exact default budgets and rollout counts after profiling.
- Whether to optimize utility as score difference or raw score.
