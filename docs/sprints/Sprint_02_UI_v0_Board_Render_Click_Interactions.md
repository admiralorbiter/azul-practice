# Sprint 2 — UI v0: Board Renderer + Interaction Loop (Click-to-Move)

**Goal:** You can load a state, see the board, select a draft action, and apply it through the engine from the UI.

## Outcomes
- A functional Practice screen (basic layout)
- Click-to-select interaction that is correct and usable
- UI uses legality enumeration to highlight valid actions

## Scope
### 1) Board rendering
- Factories grid (2-player count)
- Center pool (incl. first-player token display)
- Player board (yours):
  - pattern lines, wall grid, floor line, score
- Minimal opponent display (optional, small) for realism/debug

### 2) Interaction (click-first)
- Click a source (factory or center)
- Choose a color present
- Show legal destinations (highlight pattern lines and floor)
- Click destination to form an action
- “Apply” button calls `apply_action`

### 3) Feedback (v0)
- Show legality errors in a toast or inline panel
- Show raw state JSON in a collapsible debug panel (dev mode)

## Deliverables
- Practice screen UI (v0)
- WASM integration:
  - load state JSON
  - call `list_legal_actions`
  - call `apply_action`
- Basic UI tests (optional), manual demo steps documented

## Acceptance Criteria
- A user can complete multiple draft turns without breaking state
- Illegal actions are prevented or rejected with clear message
- UI stays responsive and consistently reflects state returned by engine

## Demo (end-of-sprint)
- Load a sample scenario
- Make 3–5 draft moves using click interaction
- Confirm score/boards update as expected (even if end-of-round not yet implemented)

## Dependencies
- Sprint 1 engine exports stable enough for UI calls

## Sprint Backlog (Suggested Tasks)
- [ ] Build core board components (Factories, Center, PlayerBoard)
- [ ] Add selection model (source→color→destination)
- [ ] Highlight legal actions
- [ ] Integrate apply_action and refresh state
- [ ] Add dev panel with seed/version/state JSON
