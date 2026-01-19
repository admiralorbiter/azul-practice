# Azul Practice Tool — Research Synthesis for a “Best Move” Algorithm (2‑Player, MVP → Scalable)

**Purpose:** This document synthesizes *rules constraints*, *strategy heuristics*, and *AI/search approaches* into a concrete plan for building a “best move” evaluator for an Azul practice tool. It’s written to support an MVP that is simple in features but **robust in core mechanics**, and it’s intentionally aligned with a future path toward stronger bots (MCTS / RL) without re-architecting.

---

## 1) What “best move” means in Azul (for your tool)

In Azul, the only decision during a round is the **draft choice**: choose a source (a factory or center) and a color, take all tiles of that color, then place them into **exactly one** pattern line (or the floor line if overflow/illegal placement forces it). The remainder from a factory goes to the center. The round ends when factories and center are empty, then everyone resolves wall-tiling, scoring adjacency, and floor penalties.

These mechanics define the evaluator’s core: given a *draft decision*, estimate downstream impact on:

- **Immediate points** (end-of-round adjacency scoring for tiles moved to the wall)  
- **Penalties** (floor line)  
- **Positioning / potential** (setting up future adjacency and end-game bonuses like completed columns/sets)  
- **Denial / tempo** (what you leave your opponent, including forcing overflow/floor penalties)

Rules constraints you must model precisely (non-negotiable for a trustworthy practice tool):

- Drafting from factory vs center; moving leftover factory tiles to center; first player marker taken when first drawing from center.  
- Pattern lines: sizes 1–5; fill right-to-left; **one color per line**; **cannot place a color into a pattern line whose corresponding wall row already has that color**.  
- Wall-tiling: for each **completed** pattern line, move the **rightmost tile** to the wall position of that color, score immediately via adjacency, and discard the rest to the box lid; incomplete lines persist.  
- Scoring: adjacency points are computed by counting contiguous horizontal group and contiguous vertical group connected to the newly placed tile (scoring both directions if both exist); otherwise score 1 point.  
- Floor line: tiles there impose negative points based on printed track; extra overflow beyond a full floor line is discarded to the lid.  
- Next round: factories refill from the bag; if the bag is empty, refill it from the lid and continue (rare edge case exists if both are empty).  
- Game end trigger: after a round in which a player completes a full horizontal row of 5 on the wall, the game ends after that round and then end-game bonuses are applied.  

**Primary rules references**: the Azul rulebook (scoring, pattern line restrictions, refill rules) and BGA help are both consistent summaries of these mechanics.  
- Rulebook PDF: https://cdn.1j1ju.com/medias/03/14/fd-azul-rulebook.pdf  
- BGA help: https://en.boardgamearena.com/doc/Gamehelpazul

---

## 2) Why early / mid / late game states matter (and how to define them)

You want scenarios from **start**, **mid-round**, and **end-round**, and you also want “early/mid/late game” across the whole match.

### 2.1 Two separate “stage” axes you should track

1) **Within-round stage** (start/mid/end round)
- **Start-of-round:** factories just refilled; center has only the first player marker (or whatever your implementation represents).
- **Mid-round:** some factories are partially emptied; center likely has multiple colors; floor lines may already have tiles.
- **End-round:** few sources remain; actions tend to be forced; blocking and overflow become more common.

2) **Across-game stage** (early/mid/late *game*)
A robust operational definition should be based on board state rather than “round number”, because games can end earlier/later.

Recommended definitions (easy to compute and correlates well with decision texture):
- **Early game:** total wall tiles placed by a player ≤ 8 (roughly rounds 1–2, depending on play)
- **Mid game:** 9–17 wall tiles placed
- **Late game:** ≥ 18 wall tiles placed **or** any player is within 1 tile of completing a horizontal wall row

Thresholds are tunable; what matters is consistency.

---

## 3) Valid state generation: best-practice methods

The biggest trap is generating states that look plausible but are **not reachable** under the game’s rules and tile supply constraints. The safest approach is: **generate states by simulating actual play** rather than constructing boards by hand.

### 3.1 Gold-standard: forward simulation (self-play) + snapshot sampling

**Approach**
1) Start from the real initial setup.
2) Use a pluggable “policy” to play legal moves forward:
   - Random legal move (baseline; fast but noisy)
   - Heuristic-biased random (better scenario diversity)
   - “Weak bot vs weak bot” self-play (good realism)
3) Record snapshots at:
   - Start/mid/end within-round
   - Early/mid/late across-game
4) Score each snapshot for “training usefulness” and keep the best.

**Why it’s best**
- Every produced state is reachable by construction.
- You naturally get realistic distributions of center/factory tiles, floor penalties, partially filled lines, etc.

### 3.2 “Useful scenario” filters (so you don’t train on boring states)

A scenario is **useful** if:
- The player has **≥ 6 legal moves** (enough choice).
- The evaluator predicts a meaningful spread, e.g.:
  - `EV(best_move) - EV(2nd_best) >= Δ`, where Δ might be 2–4 points for MVP (tunable).
- It is not “forced garbage” (e.g., only legal moves dump everything to floor).
- It contains at least one strategic consideration:
  - blocking/denial opportunity,
  - tradeoff between short-term score vs long-term bonus potential,
  - choice involving taking the first-player marker (tempo cost) vs tile quality.

This filter is what turns “random states” into **practice puzzles**.

### 3.3 Alternative: constrained construction + reachability validation (use sparingly)

You *can* construct states by sampling wall patterns / partial lines / factories / center / bag counts… but ensuring reachability is hard. If you go this route:

1) generate a candidate state,  
2) attempt to find **some** legal history from a valid initial state that reaches it (search / backtracking),  
3) accept only if reachable.

This is more complex than forward simulation; usually not worth it until you need very targeted puzzle families.

---

## 4) Evaluator architecture: options from MVP to “serious bot”

### 4.1 A practical ladder of strength (and engineering effort)

**Level 0 — Rule-correct scoring + “one-step” greedy**
- Evaluate only the immediate impact (complete pattern lines this round; floor penalties this round).
- Fast and easy, but often wrong because Azul is about positioning and denial.

**Level 1 — Heuristic evaluation (“static eval”)**
A weighted function estimating long-term value:
- adjacency potential, empty-neighbor count, center-column progress,
- probability of completing columns/sets,
- opponent blocking value, etc.
Good for speed, but brittle and needs tuning.

**Level 2 — Budgeted rollouts (recommended MVP “Tier 2”)**
For each candidate move:
- apply move,
- simulate the rest of the round (and optionally 1–2 future rounds) using fast, biased policies,
- return an average outcome (score difference).

This gives strong results with predictable runtime and pairs naturally with a “Think Longer” button.

**Level 3 — MCTS / UCT**
A structured version of rollouts:
- builds a search tree guided by exploration vs exploitation (UCT),
- uses rollouts at leaf nodes.

Key reference: Kocsis & Szepesvári’s UCT algorithm (“Bandit based Monte-Carlo Planning”).  
Source: https://ggp.stanford.edu/readings/uct.pdf

**Level 4 — RL policy/value networks**
Train a model to estimate move quality directly, then optionally guide MCTS (AlphaZero style). Future work.

### 4.2 Recommended MVP algorithm (Tier 2 rollouts + action filtering)

You previously said “Tier 2 is fine if it’s still fast” and you want a “Think Longer” option. So:

**Recommendation**
- Implement **Tier 2 rollouts** as the default evaluator.
- Add a “Think Longer” mode that increases:
  - number of rollouts per candidate move,
  - depth (finish round vs +1 extra round),
  - opponent policy strength.

**Why**
- Much easier than full MCTS while producing robust, believable results.
- Smooth upgrade path: your rollout engine becomes the simulation step inside UCT.

---

## 5) Making rollouts work well for Azul (the details that matter)

### 5.1 State representation (must include hidden zones to be “valid”)

Even if the player doesn’t see the bag contents in a real game, your simulator needs them to advance:

- Factory displays (5/7/9 depending on player count; for 2P it’s 5)  
- Center pool + first player marker presence  
- Each player:
  - wall grid (5×5)  
  - pattern lines (capacities 1..5)  
  - floor line occupancy  
  - score  
- Bag contents and lid/discard contents (for refill)

### 5.2 Enumerating legal actions (and reducing the branching factor)

Represent a draft action as:
- `source_id` (factory index or “center”),
- `color`,
- `target_pattern_line` (1–5) or `floor`.

Generate actions as:

1) Enumerate `(source, color)` pairs available.  
2) For each, enumerate valid pattern lines:
   - line empty OR already same color,
   - corresponding wall row does NOT already contain that color,
   - capacity remaining > 0  
3) Include `floor` as a target if:
   - no valid pattern line exists, OR
   - player chooses to sacrifice tiles intentionally (sometimes strategically valid).  
4) Optional: collapse dominated targets (if identical overflow and constraints).

### 5.3 Rollout policy: “semi-smart random” beats pure random

Pure random rollouts often fail in Azul. Use a fast biased policy:

- prioritize completing a pattern line (especially top 3 rows) if it doesn’t create large floor penalties,
- prefer moves that increase adjacency potential,
- avoid creating “dead” pattern lines that can’t be completed due to wall constraints,
- include cheap denial when it doesn’t cost much.

Human strategy guides often emphasize center-column adjacency and efficient completion of top rows as practical heuristics (useful as rollout bias, not “rules facts”).  
Heuristic inspiration: https://boostyourplay.com/azul-ultimate-strategy-guide-16-pro-tips/

### 5.4 How deep should rollouts go?

For MVP “best move now”:
- Minimum: roll to **end-of-round** and apply wall-tiling + scoring + penalties.
- Better: go **one extra round** with fewer samples (higher cost per rollout).
- “Think Longer”: end-of-round + 1 round with increased samples.

### 5.5 What should the rollout return (reward function)?

Use **score differential**:
- `reward = (my_score - opp_score)` at the horizon.

Optionally add a *terminal bonus estimate* if you stop early:
- estimate end-game bonuses remaining (rows/cols/sets) with a heuristic.

End-game bonus values (rows +2, columns +7, color sets +10) are widely summarized.  
Example player aid: https://playeraid.net/modules/azul/en

---

## 6) Toward explainable feedback (“your move was good because…”)

Fully natural-language “deep reasoning” is hard, but **structured, truthful explanations** are very feasible if you track feature deltas.

### 6.1 Explanation via “feature deltas” (recommended)

For each candidate move, compute:
- expected immediate wall placements this round,
- expected floor penalty this round,
- expected adjacency points this round,
- change in “completion progress” for columns / color sets,
- denial impact (what tiles you remove from availability this round).

Then pick the top drivers vs the second-best move and map them to templates:

- “This move completes a pattern line with low waste, leading to an adjacency-scoring placement.”
- “This move is strong because it denies your opponent the only remaining blue tiles.”
- “This move is weaker because it creates large floor penalties without improving long-term bonus progress.”

Adjacency scoring is defined as counting contiguous horizontal tiles, then contiguous vertical tiles, scoring both directions if applicable.  
Source: rulebook PDF (Scoring section): https://cdn.1j1ju.com/medias/03/14/fd-azul-rulebook.pdf

---

## 7) Scenario generation pipeline (recommended implementation)

1) **Self-play generator**
   - Play N games with a weak policy vs weak policy.
   - Capture snapshots every K turns and at round boundaries.

2) **Labeling**
   - For each snapshot, run Tier‑2 evaluator to compute best move, EV gap, variance.

3) **Filtering**
   - Keep snapshots by category:
     - within-round: start / mid / end
     - across-game: early / mid / late
   - Reject states with:
     - < 6 legal moves,
     - EV gap < Δ,
     - huge forced floor dumps (unless you want “damage control” puzzles).

4) **Balancing**
   - Keep target mix to avoid over-representing late forced states.

5) **Freeze packs**
   - Save puzzle packs (JSON) with full state + seed + best move + explanation deltas.

---

## 8) Performance and “Think Longer” mode (Rust/WASM friendly)

- Run evaluation in a Web Worker to keep UI responsive.
- Add a transposition table keyed by a fast hash of state (big win).
- Provide 2–3 modes:
  - Fast (few rollouts),
  - Deep,
  - Max (more samples / confidence).

---

## 9) Where MCTS fits later (without rework)

When you want a stronger bot:
- selection via UCT,
- expansion generating legal actions,
- simulation using your rollout policy,
- backprop of rewards.

UCT reference: https://ggp.stanford.edu/readings/uct.pdf

Azul-specific implementation inspiration:
- https://github.com/mgsweet/Azul-MCTS-AI  
- https://github.com/kaiyoo/AI-agent-Azul-Game-Competition

---

## References (primary)

- Azul rulebook PDF: https://cdn.1j1ju.com/medias/03/14/fd-azul-rulebook.pdf  
- Board Game Arena Azul help: https://en.boardgamearena.com/doc/Gamehelpazul  
- Kocsis & Szepesvári (2006) UCT paper: https://ggp.stanford.edu/readings/uct.pdf
