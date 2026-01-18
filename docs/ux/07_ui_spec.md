# UI Specification (MVP)

## UI goals
- Sleek, low-friction interaction.
- Drag/drop as primary interaction; click-to-select as fallback.
- Visual inspiration: Board Game Arena layout/colors (not a strict clone).

## Practice Screen layout
### Top bar
- New Scenario
- Evaluate
- Next
- Think longer (budget selector)
- Settings

### Main area
- Factories (grid)
- Center (prominent)
- Player board (large)
- Opponent board (compact but readable)

### Right panel: Feedback
- Grade (e.g., Excellent / Good / Miss)
- Best move summary
- Delta (expected points)
- Explanation bullets (1–3)
- Optional: “Show best move” overlay/highlight

---

## Interaction spec
### Drag/drop
- Drag from factory/center.
- Hover highlights legal destinations.
- Drop into a pattern line or floor.
- Illegal drop snaps back and explains why.

### Click fallback
- Click source+color → highlight legal destinations → click destination.

### Visual feedback
- Selected color highlight
- Legal destination glow
- Clear display of floor penalties
- Clear display of pattern line capacity

---

## Accessibility baseline
- Keyboard operability for core action via click fallback.
- ARIA labels for interactive regions.
- Sufficient contrast.
