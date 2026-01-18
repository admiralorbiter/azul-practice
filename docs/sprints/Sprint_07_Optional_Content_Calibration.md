# Sprint 7 (Optional) — Scenario Content + Calibration

**Goal:** Make the MVP feel “great” by tuning scenario quality and evaluator calibration, and optionally shipping a small curated scenario pack.

## Outcomes
- Better scenarios (more signal, less noise)
- Calibrated evaluator budgets and rollout policies
- A small curated pack for consistency and testing

## Scope
### 1) Generator tuning
- Improve filters:
  - avoid forced moves
  - ensure meaningful EV gaps
  - avoid repetitive patterns
- Add difficulty knobs:
  - stricter filters for “hard”
  - looser for “easy”

### 2) Evaluator calibration
- Tune:
  - shortlist size
  - rollout count vs time
  - policy bot behavior
- Confirm “Think longer” improves confidence without huge latency

### 3) Curated scenario pack (optional)
- Export 25–100 high-quality scenarios
- Tag with notes or difficulty rating
- Use as golden/regression suite

## Deliverables
- Tuned thresholds + configuration
- Optional `scenario_pack_v1.json`
- Regression suite expansion

## Acceptance Criteria
- Scenario quality is consistently good
- Evaluator feels stable and “correct enough” for practice
- Curated pack helps validate changes without regressions

## Demo
- Compare old vs tuned generator output
- Show evaluation confidence improvements with longer budget

## Dependencies
- MVP loop fully functional (through Sprint 6)
