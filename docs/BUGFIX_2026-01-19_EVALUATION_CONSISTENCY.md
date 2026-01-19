# Evaluation Consistency Issue Analysis

**Date:** 2026-01-19  
**Issue:** Move evaluator feedback in PracticeScreen differs from best move in DevPanel

## Root Cause

The DevPanel and PracticeScreen use **different evaluation parameters**, leading to inconsistent results:

| Parameter | DevPanel | PracticeScreen | Difference |
|-----------|----------|----------------|------------|
| `rollouts_per_action` | 10 (hardcoded) | 60 (calculated from `timeBudget / 25`) | **6x more rollouts** |
| `time_budget_ms` | 250ms | 1500ms | **6x longer** |
| `evaluator_seed` | `Date.now()` | `Date.now()` | Different timestamps = different seeds |

## Why This Matters

1. **More rollouts = More accurate EV estimates**: 60 rollouts gives a better estimate than 10 rollouts, which can change which move appears "best"

2. **Different seeds = Different simulations**: Even with the same parameters, different seeds produce different rollout results, causing variance in EV estimates

3. **User confusion**: Users see "Best Move: EV 16.20" in DevPanel, but then get feedback comparing their move to a *different* best move (with different EV) calculated by PracticeScreen with different parameters

## Code Locations

### DevPanel Evaluation
**File:** `web/src/components/dev/DevPanel.tsx` (lines 63-72)
```typescript
const result = evaluateBestMove(gameState, gameState.active_player_id, {
  evaluator_seed: Date.now(),
  time_budget_ms: 250,
  rollouts_per_action: 10,  // ← Hardcoded low value
  shortlist_size: 20,
  rollout_config: {
    active_player_policy: 'all_greedy',
    opponent_policy: 'all_greedy'
  }
});
```

### PracticeScreen Evaluation
**File:** `web/src/components/PracticeScreen.tsx` (lines 156-168)
```typescript
const rolloutsPerAction = Math.floor(timeBudget / 25);  // ← 1500 / 25 = 60!

const result = gradeUserAction(stateBeforeMove, stateBeforeMove.active_player_id, userAction, {
  evaluator_seed: Date.now(),
  time_budget_ms: timeBudget,  // ← Default 1500ms
  rollouts_per_action: rolloutsPerAction,
  shortlist_size: 20,
  rollout_config: {
    active_player_policy: 'all_greedy',
    opponent_policy: 'all_greedy'
  }
});
```

## Solutions

### Option 1: Make DevPanel Match PracticeScreen (Recommended)
Change DevPanel to use the same evaluation parameters as PracticeScreen:
```typescript
const result = evaluateBestMove(gameState, gameState.active_player_id, {
  evaluator_seed: Date.now(),
  time_budget_ms: 1500,        // Match PracticeScreen
  rollouts_per_action: 60,     // Match PracticeScreen default
  shortlist_size: 20,
  rollout_config: {
    active_player_policy: 'all_greedy',
    opponent_policy: 'all_greedy'
  }
});
```

### Option 2: Use Shared Constants
Create a shared configuration file:
```typescript
// web/src/config/evaluator.ts
export const DEFAULT_EVALUATOR_PARAMS = {
  time_budget_ms: 1500,
  rollouts_per_action: 60,
  shortlist_size: 20,
  rollout_config: {
    active_player_policy: 'all_greedy' as const,
    opponent_policy: 'all_greedy' as const
  }
};
```

Then import and use in both components.

### Option 3: Use the Same Seed
For deterministic debugging, you could pass the same seed to both evaluations. However, this doesn't solve the different rollout counts issue.

## Recommendation

**Implement Option 2** (shared constants) because:
1. Ensures consistency across the app
2. Makes it easy to tune evaluation parameters in one place
3. Prevents future drift between components
4. Still allows PracticeScreen to adjust `time_budget_ms` via the "Think Longer" control

## Additional Notes

- The `gradeUserAction()` function in Rust already has logic to reuse EV from candidates if the user's action was evaluated (lines 480-484 in `evaluator.rs`), which helps reduce seed variance
- The seed offset of +1,000,000 for user action rollouts (line 498) ensures independent sampling
- Both evaluations use the same shortlist_size (20) and rollout policies (all_greedy), which is good
