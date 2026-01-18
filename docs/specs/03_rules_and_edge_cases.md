# Rules & Edge Cases Specification (Authoritative)

> This document defines the authoritative interpretation of Azul rules as implemented by the engine.
> It should be expanded with explicit examples and test vectors.

## 1. Components & setup (2-player)
- Tile bag and lid rules (refill behavior)
- Factory count and initial fill per round
- Center area and first-player token

## 2. Round structure
1. Fill factories
2. Drafting phase (players alternate drafting moves)
3. End-of-round wall tiling + scoring
4. Prepare next round

## 3. Drafting move rules
### 3.1 Choose a source
- A factory OR the center.

### 3.2 Choose a color
- Take all tiles of that color from the chosen source.
- If from a factory: remaining tiles move to center.
- If from center: the first-player token rules apply.

### 3.3 Choose a destination
- Place tiles into one pattern line OR floor line.
- Pattern line constraints:
  - capacity per row
  - cannot mix colors
  - cannot place a color if that color already exists in the corresponding wall row
- Excess tiles go to the floor line.

## 4. End-of-drafting conditions
- Drafting ends when factories and center are empty (except any persistent token rules).

## 5. End-of-round wall tiling + scoring (deterministic)
- Completed pattern lines place exactly one tile on the wall.
- Remaining tiles from completed line go to lid.
- Score adjacency per Azul rules.
- Apply floor penalties.
- Determine first player for next round.

## 6. End-of-game and bonus scoring
- Trigger conditions
- Bonus scoring rules

---

## Edge cases to specify explicitly (with examples)
1. Attempting to place a different color into a partially-filled pattern line.
2. Attempting to place a color into a row where that color is already on the wall.
3. Taking from center with first-player token present.
4. Tile bag depletion and lid refill (partial fills).
5. Floor line overflow.
6. Scenario validity constraints (no impossible tile counts; consistent totals).

---

## Required test vectors (to add)
- Legal action enumeration for a known scenario.
- Exact scoring outcomes for known wall placements.
- Bag/lid refills for boundary conditions.
