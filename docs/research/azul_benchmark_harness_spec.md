# Azul Practice Tool — Benchmark Harness & Logging Spec

This file describes a practical, implementation-friendly way to run experiments and produce reproducible reports.

---

## 1) Core objects (suggested)

### 1.1 State representation
- `GameState` (full deterministic state)
- must be serializable to bytes/string for:
  - dataset storage
  - reproducible runs
  - state hashing

**Recommendation:**  
- Use a canonical serialization (e.g., bincode) + a stable version tag.
- Include:
  - bag/lid contents (if modeling randomness later)
  - factories + center
  - player boards (wall, pattern lines, floor)
  - current player to act
  - round number / stage metadata

### 1.2 Action representation
Define a compact, comparable action encoding:
- `(source, color, factory_id/center, destination_line, overflow_policy?)`

Ensure:
- stable ordering for enumeration
- deterministic string form for logs and diffing

---

## 2) Dataset format

### 2.1 Folder layout
```
bench/
  positions/
    v1/
      early_start.jsonl
      early_mid.jsonl
      ...
  metadata/
    v1_manifest.json
```

### 2.2 Position record schema (JSONL)
Each line:
```json
{
  "id": "uuid-or-hash",
  "bucket": "early_mid",
  "seed": 12345,
  "state_version": "v1",
  "state_blob_b64": "...",
  "tags": ["generated", "rich", "near_filter_boundary"],
  "features": {
    "legal_actions": 12,
    "wall_tiles_p1": 9,
    "wall_tiles_p2": 7,
    "tiles_remaining": 42
  }
}
```

---

## 3) Evaluator output schema

For each (position, evaluator):
```json
{
  "position_id": "...",
  "evaluator": "tier2_rollout",
  "budget": "medium",
  "k_rollouts": 10,
  "policy": "greedy_v1",
  "seed": 999,
  "actions": [
    {"a": "A1", "ev": 3.4, "n": 10, "var": 1.2},
    {"a": "A2", "ev": 3.1, "n": 10, "var": 0.8}
  ],
  "recommended": "A1",
  "timing_ms": 7.8
}
```

### Notes
- Always log `n` and `var` per action for calibration and adaptive allocation.
- Log the full ranked list when possible; otherwise log shortlist + “excluded count”.

---

## 4) Oracle labeling workflow

### 4.1 Exact oracle (late-round)
- Run exact to end-of-round on:
  - all RoundStage=End positions
  - and a sampled subset of RoundStage=Mid
- Store:
  - oracle-best action
  - oracle EVs for all actions (if feasible)

### 4.2 MCTS oracle (mid/early round)
- Run offline with high budgets:
  - 5×, 10×, 20× your deep setting
- Store:
  - visit counts
  - mean value per action

### 4.3 Multi-round oracle (proxy to game optimal)
- Run N-round or full-game simulation with stochastic bag
- Store:
  - mean final score diff
  - stdev (variance across stochasticity)

---

## 5) Metric computations

### 5.1 Top-1 / top-k
- top-1: recommended == oracle_best
- top-k: oracle_best in {top k by evaluator}

### 5.2 Regret
- regret = oracle_EV(best_oracle) − oracle_EV(recommended)

Track mean and p95 by bucket.

### 5.3 Stability
Run the same evaluator multiple times:
- different seeds
- different budgets (fast/med/deep)
Compute:
- recommendation flip rate
- grade flip rate (MISS/OKAY/GOOD)

---

## 6) Adaptive rollout allocation (“racing”) integration

### Sequential halving
Inputs:
- total rollout budget B
- candidate actions A

Procedure:
1. allocate small n0 to each action
2. compute confidence bounds from mean/variance
3. drop worst half by LCB
4. repeat until B spent

### Early stop rule
Stop when:
- LCB(best) > UCB(second_best)

Log:
- final n per action
- stop reason (“budget” vs “separated”)

---

## 7) Reporting

### 7.1 Per-bucket table
For each bucket:
- N
- top-1
- top-3
- mean regret
- p95 regret
- recommendation flip rate
- avg timing

### 7.2 Aggregate
- weighted overall metrics (weights by expected frequency of bucket)
- worst 3 buckets by regret and instability (targets for improvements)

---

## 8) Practical defaults (starter)

- Dataset size: 200 positions per bucket (1800 total)
- Tier-2 budgets:
  - fast: K=4
  - medium: K=10
  - deep: K=25
- Oracles:
  - exact: all late-round + 10% mid
  - MCTS: 10× deep on early/mid buckets (offline only)
  - multi-round: N=2 rounds for early buckets as proxy

---

If you want, I can also provide:
- a suggested state hash (Zobrist-style vs canonical serialize+hash)
- a concrete action encoding that matches Azul rules cleanly
- a “feature set” checklist for realism / difficulty tagging
