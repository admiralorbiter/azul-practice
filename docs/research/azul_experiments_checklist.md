# Azul Practice Tool — Experiments & Ablations Checklist

Use this as a repeatable checklist for each iteration.

---

## 0) Before you start
- [ ] Benchmark suite exists and is stratified by (GameStage, RoundStage)
- [ ] Oracles available for at least late-round (exact) and mid-round (MCTS/high budget)
- [ ] Evaluator logs action EVs, n, variance, timing

---

## 1) Baseline report
Run Tier-2 (current) on full dataset and compute:
- [ ] top-1 and top-3 agreement vs oracle
- [ ] regret (mean, p95) per bucket
- [ ] seed-sensitivity and grade-flip rates
- [ ] compute cost per bucket

---

## 2) Opponent model sweep
Compare rollout opponent policies:
- [ ] greedy vs greedy
- [ ] greedy vs mixed (70/30 greedy/random)
- [ ] greedy vs stronger-lite

Success criteria:
- [ ] reduces mean regret in early buckets
- [ ] improves stability (fewer flips)
- [ ] keeps compute acceptable

---

## 3) Utility function tests
Try incremental modifications:
- [ ] add bonus proxy: round_diff + λ * bonus_potential
- [ ] risk-adjust: round_diff − ρ * variance
- [ ] hybrid: a*round_diff + b*expected_endgame_proxy

Success criteria:
- [ ] improves correlation with multi-round oracle
- [ ] does not degrade late-round agreement

---

## 4) Shortlist policy tests
If evaluating only top-N actions:
- [ ] measure oracle-best-in-shortlist rate by bucket
- [ ] expand N for problematic buckets
- [ ] test heuristic scoring tweaks that improve inclusion

---

## 5) Adaptive rollout allocation (“racing”)
Replace fixed K with racing:
- [ ] sequential halving
- [ ] early-stop when separated

Success criteria:
- [ ] equal or better top-1/regret at same compute
- [ ] produces meaningful confidence flag rate

---

## 6) Scenario generation audits
Check generator bias:
- [ ] feature distribution comparisons (generated vs oracle self-play vs real logs)
- [ ] identify buckets with distribution drift
- [ ] adjust filters / self-play policies

---

## 7) Human training-value validation
- [ ] “difficulty” labeling based on EV gap / entropy
- [ ] confirm hard positions correlate with user errors
- [ ] track improvement over sessions within each bucket

---

## 8) Ship criteria (for the practice tool)
- [ ] late-round: high top-1 agreement with exact oracle
- [ ] early/mid: regret below threshold (define per bucket)
- [ ] grade stability acceptable (rare flips)
- [ ] latency acceptable in browser budgets
