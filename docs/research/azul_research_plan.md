# Azul Practice Tool — Research Plan (Move Evaluation + Scenario Generation)

This plan is written to help you **verify, quantify, and improve** your Tier-2 rollout evaluator and your self-play snapshot generator.

It is structured as:
1) **Targets (what “best move” means)**
2) **Oracles (stronger judges to benchmark against)**
3) **Benchmark suite (positions + stratification)**
4) **Metrics (accuracy, regret, calibration, stability)**
5) **Statistical methods (variance, confidence, racing)**
6) **Ablations & improvement loop**
7) **Training-value validation (does this actually help players?)**
8) **Minimal “first build” roadmap**

---

## 1) Define evaluation targets (you need 3, not 1)

Your current Tier-2 evaluator:
- Enumerates legal actions
- Runs **K rollouts per action** using policy bots
- Uses **end-of-round score differential** as utility

That implies different reasonable “ground truths” depending on what you want to optimize.

### Target A — Round-Optimal (perfect-info remainder of the round)
**Definition:** Best action maximizes end-of-round score differential under optimal play for the rest of the round (deterministic, perfect information).

**Why:** Matches your current utility (end-of-round score diff) and avoids endgame bonus complexity.

### Target B — Policy-Optimal (best vs a specified opponent model)
**Definition:** Best action vs a specified opponent model: greedy, random, mixed, or stronger.

**Why:** Your EV estimates are conditional on the rollout policy (greedy heuristics). This target tells you how sensitive feedback is to opponent modeling.

### Target C — Game-Optimal Proxy (multi-round or endgame-aware)
**Definition:** Best action under deeper simulation: e.g., 2–3 rounds or full game with bag/lid randomness.

**Why:** Round-only utility can miss bonus tactics and long-horizon strategy; you need a proxy stronger than Tier-2 to audit your bias.

---

## 2) Build oracle evaluators (benchmarks need a stronger judge)

You need at least one “stronger-than-you” reference. Oracles can be offline-only.

### Oracle 1 — Exact solver to end-of-round (late round / small branching)
When RoundStage is late (few tiles remaining), the game tree shrinks. Implement:
- Alpha-beta (or negamax) to end-of-round
- Transposition table (hash state)
- Terminal utility = end-of-round score diff

Use this oracle to produce high-confidence labels for late-round positions.

### Oracle 2 — High-budget MCTS/UCT to end-of-round (mid/early round)
- Use your rollout engine as the simulation policy
- Spend 5–20× the rollouts of your “Deep” setting
- Optionally use a stronger opponent mixture (e.g., greedy+noise)

MCTS becomes a scalable reference for mid-round.

### Oracle 3 — Multi-round rollout oracle (endgame-aware proxy)
- Resolve current round
- Refill factories (bag/lid)
- Simulate N additional rounds (or full game)
- Utility = final score diff (or score diff + bonus proxy)

This oracle is your best audit for long-horizon tactics.

---

## 3) Benchmark suite: positions that are representative AND difficult

### 3.1 Stratify by your two-axis stage model
Use your existing axes:
- **GameStage** (progress on wall)
- **RoundStage** (tiles remaining)

Buckets (minimum):
- Early/Start, Early/Mid, Early/End
- Mid/Start, Mid/Mid, Mid/End
- Late/Start, Late/Mid, Late/End

### 3.2 Populate positions from multiple sources
1) **Generated** (your current pipeline): self-play → snapshot at decision points → quality filters  
2) **Adversarial**: positions near filter boundaries (e.g., just meets “strategic richness” threshold)  
3) **Real logs (optional but very valuable)**: extract decision points from recorded games (e.g., BGA) to avoid generator overfitting.

### 3.3 Realism checks (to detect generator bias)
Compute feature distributions for each bucket:
- floor occupancy rate
- pattern-line fill histogram
- wall progress vector
- frequency of forced floor-only moves
- number of legal actions distribution
Compare:
- generated vs real logs (if available)
- generated vs stronger-self-play (oracle-driven self-play)

Flag buckets where distributions diverge.

---

## 4) Metrics: how you’ll know you’re correct and improving

Assume for each position:
- Tier-2 action ranking + EV estimates
- Oracle action ranking + EV estimates

### 4.1 Ranking correctness
- **Top-1 agreement**
- **Top-k inclusion** (oracle-best is in shortlist)
- **Rank correlation** (Spearman/Kendall) when oracle gives full action scores

### 4.2 Regret (more informative than accuracy)
For each position:
- **regret = oracle_EV(best_oracle) − oracle_EV(recommended_by_tier2)**

Track:
- mean regret per bucket
- p95 regret per bucket (tail risk)

### 4.3 Calibration and stability (MC noise + user-facing grades)
- seed sensitivity: stdev of EV per action across random seeds
- grade stability: probability a move’s grade changes across seeds/budgets
- “confidence flag” rate: fraction of positions where best action is statistically separated

---

## 5) Math & stats: confidence, variance, and adaptive rollout allocation

Your EV estimate is a sample mean:
- EV_hat(a) = (1/K) Σ utility_i(a)

Compute online per action:
- sample variance s²(a)
- standard error SE(a) = s(a)/√K
- confidence interval: EV_hat(a) ± z * SE(a) (z≈1.96)

### 5.1 Replace fixed K with “racing” (same budget, higher reliability)
**Sequential halving / racing**
1. Give every action a small number of rollouts (2–4)
2. Drop the bottom half using lower confidence bounds
3. Repeat until budget spent

**Stopping rule**
- Stop when LCB(best) > UCB(second_best)

This gives a principled “high confidence” indicator you can show the user.

---

## 6) Improvement loop (ablation studies)

Run controlled experiments where you change one knob and re-score the benchmark suite.

### A) Opponent-model sweep
Compare:
- greedy vs greedy
- greedy vs mixed (e.g., 70/30 greedy/random)
- greedy vs stronger (oracle-lite policy)

Measure changes in:
- Top-1 agreement
- regret
- grade stability

### B) Utility experiments
Start simple:
- utility = round_diff + λ * bonus_proxy  
- utility = round_diff − ρ * variance (risk adjusted)

Measure correlation with multi-round oracle and regret changes.

### C) Shortlist robustness
If you only evaluate top-N actions:
- measure oracle-best-in-shortlist rate
- measure regret conditional on oracle-best excluded
Adapt N by bucket if needed.

### D) Scenario difficulty labeling
Define difficulty:
- **Easy**: big EV gap between best and second
- **Hard**: several actions within tight band (high entropy)
Validate difficulty against user performance (more misses on hard).

---

## 7) Validate that this helps humans (not just “engine strength”)

### Offline proxy: student improvement
Train a lightweight “student” policy (even linear model over feature deltas) on your feedback labels and test whether it improves win-rate vs baseline greedy.

### Online metrics
Track per user:
- time-to-decision by bucket
- grade distribution shift over sessions (MISS → OKAY → GOOD)
- retention / repeat sessions
- improvement on “hard” subset specifically

---

## 8) Minimal “first build” roadmap (high signal, low scope)

1) **Exact end-of-round oracle** for RoundStage=End  
2) **Benchmark suite** stratified by (GameStage, RoundStage)  
3) **Metrics harness**: top-1/top-k, regret, seed sensitivity, grade stability  
4) **Adaptive rollout allocation** (racing) under existing Fast/Medium/Deep budgets  
5) Add MCTS oracle next, then multi-round oracle.

---

## Appendices

### A. Recommended default benchmarks (starter sizes)
- 200 positions per bucket × 9 buckets = 1800 positions total (starter)
- For late-round, include a higher fraction of exact-oracle-labeled positions.

### B. Report format (per experiment)
For each bucket:
- N positions
- top-1 agreement
- mean regret, p95 regret
- grade flip rate
- average compute cost (ms)

---

If you want, share 5–10 representative serialized states per bucket and we can pin down:
- exact oracle interface
- state hashing scheme
- action encoding for consistent comparisons across evaluators
