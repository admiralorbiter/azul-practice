# Sprint 6 — Feedback Explanations + Drag/Drop + UX Polish

**Goal:** Make the practice loop feel excellent: drag/drop interactions, meaningful short feedback, and a polished UI that you enjoy using repeatedly.

## Outcomes
- Drag/drop board UX with highlighting and snapback
- Explainable feedback bullets based on feature deltas
- Polish pass: visuals, spacing, responsiveness, and error handling

## Scope
### 1) Drag-and-drop interactions
- Drag tiles from source → destination row/floor
- Highlight legal drop targets during drag
- On illegal drop: snap back and show reason
- Keep click-to-move as fallback for accessibility

### 2) Feedback explanations (template-based)
Compute feature deltas between user action and best action based on rollout summaries:
- expected floor penalty difference
- pattern line completion likelihood difference
- wasted tiles to floor difference
- adjacency scoring likelihood difference
- first player token outcomes (optional reporting)

Render 1–3 bullets:
- “Best move reduces expected floor penalty by ~X.”
- “Best move increases chance to complete a pattern line.”
- etc.

### 3) Grading system
- Bucket by delta EV:
  - Excellent / Good / Okay / Miss (thresholds configurable)
- Show delta EV numerically in dev mode

### 4) UI polish
- Board Game Arena-inspired palette (approx)
- Clean typography + spacing
- Right-side feedback panel usability
- Shortcuts:
  - Next scenario hotkey
  - Evaluate hotkey (optional)

### 5) Quality-of-life debugging
- Copy seed + state JSON
- Toggle: show legal moves
- Optional: “Show best move” overlay animation

## Deliverables
- Drag/drop UX complete
- Feedback bullet system integrated with evaluator output
- Grading buckets + settings for thresholds
- A polished practice loop

## Acceptance Criteria
- Interactions feel smooth and intuitive
- Feedback is understandable and consistently helpful
- You can do 20 scenarios in a row without frustration
- No frequent UI jank during evaluation (worker if needed)

## Demo (end-of-sprint)
- Generate scenario → drag/drop your move → evaluate
- See grade + best move + 2 feedback bullets
- Click “Next scenario” and repeat

## Dependencies
- Sprint 5 evaluator output (EV + stats)

## Sprint Backlog (Suggested Tasks)
- [ ] Implement drag/drop components with highlights
- [ ] Add feedback feature extraction + templates
- [ ] Add grade thresholds + display
- [ ] Apply UI polish pass and usability tweaks
- [ ] Add best-move overlay visualization
