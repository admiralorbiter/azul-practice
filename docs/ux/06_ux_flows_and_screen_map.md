# UX Flows & Screen Map (MVP)

## Screens
1. **Practice Screen** (primary)
2. **Settings** (drawer or modal)
3. **Dev Panel** (dev-only, can be a collapsible section)

---

## Primary user flow: Practice loop
1. Click **New Scenario**
2. The board renders the scenario.
3. User performs a move (drag/drop).
4. Click **Evaluate**
5. See:
   - grade
   - best move
   - delta
   - explanation bullets
6. Click **Next Scenario**

---

## Secondary flow: “Think longer”
1. User selects a longer evaluation budget.
2. Click **Evaluate**
3. UI shows a short “evaluating…” state.
4. Results appear.

---

## Error/guardrail states
- Illegal drop → snap back + inline error
- Evaluation timeout → return best-so-far with a note (“fast budget reached”)
- Engine invariant violation (dev) → show diagnostic panel and allow copying state
