# Scenario Generation (2-player; Early/Mid/Late)

## Goal
Generate valid, useful scenarios for the best-move practice loop.

## Why not pure random
Pure random board states tend to be invalid, implausible, or low-signal. The MVP generator should produce states by **playing forward legally**.

---

## Generator approach (MVP)
1. Start from a legal round start.
2. Use fast agents to play forward a number of drafting turns.
3. Stop at the active player’s turn.
4. Label scenario as `EARLY` / `MID` / `LATE` based on progress.

## Phase targeting
Suggested defaults:
- EARLY: 0–2 picks in
- MID: roughly 1/3–2/3 through drafting
- LATE: near end of drafting (few picks remain)

---

## Quality filters (important)
Reject scenarios that are:
- Forced (only 1 legal move)
- Degenerate (almost all moves dump to floor regardless)
- Low-signal (best vs median EV gap below threshold)

---

## Parameters (MVP)
- `phaseTarget`: EARLY/MID/LATE/ANY
- `seed`: integer
- `minLegalMoves`
- `minEVGap`

---

## Optional: scenario packs
- Generated scenarios saved as JSON for reproducibility.
- Curated pack can be added later for consistent training/testing.
