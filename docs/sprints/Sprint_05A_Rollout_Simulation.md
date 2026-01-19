# Sprint 05A — Rollout Simulation Infrastructure

**Status:** 📋 **PLANNED**  
**Prerequisites:** Sprint 03 (End-of-Round Resolution) ✅ and Sprint 04 (Policy Infrastructure) ✅ complete  
**Dependencies:** End-of-round resolution, policy trait, RNG infrastructure  
**Complexity:** Medium

---

## Goal

Build the core rollout engine that simulates complete drafting rounds using policy-based bots, enabling Monte Carlo evaluation of draft actions.

## Outcomes

- ✓ Rollout simulation function that plays from any state to end-of-round
- ✓ Integration with existing GreedyPolicy and RandomPolicy from Sprint 4
- ✓ Deterministic rollouts with seeded RNG
- ✓ Statistics collection for evaluation (scores, actions taken, etc.)
- ✓ Tile conservation maintained throughout simulation
- ✓ Comprehensive test coverage

---

## Rollout Simulation Overview

A **rollout** is a simulated playthrough from a given game state to the end of the current drafting round:

1. Start with a state in the middle of a drafting round
2. For each turn, use a policy to select a legal action
3. Apply the action and advance to next player
4. Continue until all factories and center are empty
5. Trigger end-of-round resolution (Sprint 03)
6. Return final state and statistics

**Purpose:** Rollouts estimate the expected outcome of a draft action by sampling possible continuations.

---

## Core Types

### RolloutConfig

Configuration for a single rollout simulation.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RolloutConfig {
    /// Policy for the active player (player whose action we're evaluating)
    pub active_player_policy: PolicyMix,
    
    /// Policy for the opponent
    pub opponent_policy: PolicyMix,
    
    /// Seed for deterministic RNG
    pub seed: u64,
    
    /// Maximum actions per rollout (safety cutoff)
    #[serde(default = "default_max_actions")]
    pub max_actions: usize,
}

fn default_max_actions() -> usize {
    100 // Safety limit to prevent infinite loops
}
```

**Policy Recommendations:**
- **For evaluation:** Both players use `PolicyMix::AllGreedy` (realistic play)
- **For testing:** Mix of Random and Greedy to test various scenarios
- **For validation:** `PolicyMix::AllRandom` for maximum coverage

### RolloutResult

Output from a rollout simulation.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RolloutResult {
    /// Final state after end-of-round resolution
    pub final_state: State,
    
    /// Player 0's final score
    pub player_0_score: i32,
    
    /// Player 1's final score
    pub player_1_score: i32,
    
    /// Number of drafting actions simulated (before resolution)
    pub actions_simulated: usize,
    
    /// Whether the round ended normally (true) or hit max_actions (false)
    pub completed_normally: bool,
}
```

### RolloutError

Error conditions during rollout.

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RolloutError {
    /// No legal actions available but round not complete
    Deadlock(String),
    
    /// Policy failed to select an action
    PolicyFailure(String),
    
    /// Action application failed
    IllegalAction(String),
    
    /// Hit max_actions safety limit
    MaxActionsExceeded,
}

impl std::fmt::Display for RolloutError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RolloutError::Deadlock(msg) => write!(f, "Rollout deadlock: {}", msg),
            RolloutError::PolicyFailure(msg) => write!(f, "Policy failure: {}", msg),
            RolloutError::IllegalAction(msg) => write!(f, "Illegal action: {}", msg),
            RolloutError::MaxActionsExceeded => write!(f, "Max actions exceeded"),
        }
    }
}

impl std::error::Error for RolloutError {}
```

---

## Algorithm: simulate_rollout

### Function Signature

```rust
pub fn simulate_rollout(
    initial_state: &State,
    config: &RolloutConfig,
) -> Result<RolloutResult, RolloutError>
```

### Pseudocode

```
function simulate_rollout(initial_state, config) -> Result<RolloutResult>:
    // 1. Initialize
    state = initial_state.clone()
    rng = create_rng_from_seed(config.seed)
    actions_simulated = 0
    
    // 2. Simulate drafting phase
    loop:
        // Check termination conditions
        if is_round_complete(&state):
            break  // Normal completion
        
        if actions_simulated >= config.max_actions:
            return Err(RolloutError::MaxActionsExceeded)
        
        // Get legal actions for current player
        legal_actions = list_legal_actions(&state, state.active_player_id)
        
        if legal_actions.is_empty():
            return Err(RolloutError::Deadlock(
                "No legal actions but round not complete"
            ))
        
        // Select action using policy
        current_player = state.active_player_id
        policy = if current_player == 0:
                     config.active_player_policy
                 else:
                     config.opponent_policy
        
        action = select_action_with_policy(
            &state, 
            &legal_actions, 
            policy, 
            &mut rng
        )
        
        if action is None:
            return Err(RolloutError::PolicyFailure(
                "Policy returned no action"
            ))
        
        // Apply action
        state = apply_action(&state, &action)?
        actions_simulated += 1
    
    // 3. Resolve end of round (Sprint 03)
    resolve_end_of_round(&mut state)
    
    // 4. Return result
    Ok(RolloutResult {
        final_state: state.clone(),
        player_0_score: state.players[0].score,
        player_1_score: state.players[1].score,
        actions_simulated,
        completed_normally: true,
    })
```

### Round Completion Check

```rust
/// Check if the drafting round is complete (all factories and center empty)
fn is_round_complete(state: &State) -> bool {
    // Check all factories are empty
    for factory in &state.factories {
        if !factory.is_empty() {
            return false;
        }
    }
    
    // Check center is empty (only first player token may remain)
    if !state.center.tiles.is_empty() {
        return false;
    }
    
    true
}
```

---

## Policy Selection Helper

Wrapper to select actions using PolicyMix from Sprint 4.

```rust
/// Select an action using the specified policy mix
fn select_action_with_policy<R: Rng>(
    state: &State,
    legal_actions: &[DraftAction],
    policy_mix: PolicyMix,
    rng: &mut R,
) -> Option<DraftAction> {
    use crate::rules::policy::{RandomPolicy, GreedyPolicy, DraftPolicy};
    
    match policy_mix {
        PolicyMix::AllRandom => {
            let policy = RandomPolicy;
            policy.select_action(state, legal_actions, rng)
        }
        PolicyMix::AllGreedy => {
            let policy = GreedyPolicy;
            policy.select_action(state, legal_actions, rng)
        }
        PolicyMix::Mixed { greedy_probability } => {
            let use_greedy = rng.gen::<f64>() < greedy_probability;
            if use_greedy {
                let policy = GreedyPolicy;
                policy.select_action(state, legal_actions, rng)
            } else {
                let policy = RandomPolicy;
                policy.select_action(state, legal_actions, rng)
            }
        }
    }
}
```

---

## Integration with Sprint 4 Infrastructure

Sprint 4 already provides:

✅ **Policy Trait** (`rust/engine/src/rules/policy.rs`):
```rust
pub trait DraftPolicy {
    fn select_action<R: Rng>(
        &self,
        state: &State,
        legal_actions: &[DraftAction],
        rng: &mut R,
    ) -> Option<DraftAction>;
}
```

✅ **Concrete Policies:**
- `RandomPolicy` - Uniform random selection
- `GreedyPolicy` - Heuristic-based selection (prefers pattern lines, higher tile counts)

✅ **PolicyMix Enum:**
```rust
pub enum PolicyMix {
    AllRandom,
    AllGreedy,
    Mixed { greedy_probability: f64 },
}
```

✅ **RNG Infrastructure** (`rust/engine/src/rules/rng.rs`):
```rust
pub fn create_rng_from_seed(seed: u64) -> StdRng
pub fn generate_seed_string() -> String
pub fn parse_seed_string(s: &str) -> Result<u64, String>
```

**Sprint 5A Integration:** Simply reuse these components!

---

## Test Requirements

### Unit Tests

**Test 1: Rollout completes from start of round**
```rust
#[test]
fn test_rollout_from_round_start() {
    let state = create_start_of_round_state();
    let config = RolloutConfig {
        active_player_policy: PolicyMix::AllGreedy,
        opponent_policy: PolicyMix::AllGreedy,
        seed: 12345,
        max_actions: 100,
    };
    
    let result = simulate_rollout(&state, &config).unwrap();
    
    // Round should complete
    assert!(result.completed_normally);
    
    // Should have taken some actions
    assert!(result.actions_simulated > 0);
    assert!(result.actions_simulated < 30); // Typical round is 10-20 actions
    
    // Factories and center should be empty
    assert!(is_round_complete(&result.final_state));
    
    // Scores should be non-negative
    assert!(result.player_0_score >= 0);
    assert!(result.player_1_score >= 0);
}
```

**Test 2: Rollout completes from mid-round**
```rust
#[test]
fn test_rollout_from_mid_round() {
    let state = create_mid_round_state();
    let config = RolloutConfig {
        active_player_policy: PolicyMix::AllRandom,
        opponent_policy: PolicyMix::AllRandom,
        seed: 67890,
        max_actions: 100,
    };
    
    let result = simulate_rollout(&state, &config).unwrap();
    
    // Should complete with fewer actions than from start
    assert!(result.completed_normally);
    assert!(result.actions_simulated > 0);
    assert!(result.actions_simulated < 15);
}
```

**Test 3: Deterministic rollouts with same seed**
```rust
#[test]
fn test_deterministic_rollouts() {
    let state = create_test_state();
    let config = RolloutConfig {
        active_player_policy: PolicyMix::AllRandom,
        opponent_policy: PolicyMix::AllRandom,
        seed: 42,
        max_actions: 100,
    };
    
    // Run rollout twice with same seed
    let result1 = simulate_rollout(&state, &config).unwrap();
    let result2 = simulate_rollout(&state, &config).unwrap();
    
    // Results should be identical
    assert_eq!(result1.actions_simulated, result2.actions_simulated);
    assert_eq!(result1.player_0_score, result2.player_0_score);
    assert_eq!(result1.player_1_score, result2.player_1_score);
    
    // States should be identical
    assert_eq!(
        serde_json::to_string(&result1.final_state).unwrap(),
        serde_json::to_string(&result2.final_state).unwrap()
    );
}
```

**Test 4: Different seeds produce different results**
```rust
#[test]
fn test_different_seeds_different_results() {
    let state = create_test_state();
    let config1 = RolloutConfig {
        active_player_policy: PolicyMix::AllRandom,
        opponent_policy: PolicyMix::AllRandom,
        seed: 111,
        max_actions: 100,
    };
    let config2 = RolloutConfig {
        seed: 222,
        ..config1.clone()
    };
    
    let result1 = simulate_rollout(&state, &config1).unwrap();
    let result2 = simulate_rollout(&state, &config2).unwrap();
    
    // Results should differ (with very high probability)
    assert_ne!(result1.actions_simulated, result2.actions_simulated);
}
```

**Test 5: Tile conservation throughout rollout**
```rust
#[test]
fn test_tile_conservation_in_rollout() {
    let state = create_fully_populated_state();
    let config = RolloutConfig {
        active_player_policy: PolicyMix::AllGreedy,
        opponent_policy: PolicyMix::AllGreedy,
        seed: 999,
        max_actions: 100,
    };
    
    // Verify initial state has 100 tiles
    assert_eq!(count_total_tiles(&state), 100);
    
    let result = simulate_rollout(&state, &config).unwrap();
    
    // Verify final state still has 100 tiles
    assert_eq!(count_total_tiles(&result.final_state), 100);
}
```

**Test 6: Max actions safety limit**
```rust
#[test]
fn test_max_actions_exceeded() {
    let state = create_test_state();
    let config = RolloutConfig {
        active_player_policy: PolicyMix::AllRandom,
        opponent_policy: PolicyMix::AllRandom,
        seed: 123,
        max_actions: 3, // Artificially low limit
    };
    
    let result = simulate_rollout(&state, &config);
    
    // Should hit max actions before completing
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), RolloutError::MaxActionsExceeded);
}
```

**Test 7: Both GreedyPolicy and RandomPolicy work**
```rust
#[test]
fn test_greedy_vs_random_policies() {
    let state = create_test_state();
    
    // Test with all greedy
    let config_greedy = RolloutConfig {
        active_player_policy: PolicyMix::AllGreedy,
        opponent_policy: PolicyMix::AllGreedy,
        seed: 555,
        max_actions: 100,
    };
    let result_greedy = simulate_rollout(&state, &config_greedy).unwrap();
    assert!(result_greedy.completed_normally);
    
    // Test with all random
    let config_random = RolloutConfig {
        active_player_policy: PolicyMix::AllRandom,
        opponent_policy: PolicyMix::AllRandom,
        seed: 555,
        max_actions: 100,
    };
    let result_random = simulate_rollout(&state, &config_random).unwrap();
    assert!(result_random.completed_normally);
    
    // Test with mixed policy
    let config_mixed = RolloutConfig {
        active_player_policy: PolicyMix::Mixed { greedy_probability: 0.7 },
        opponent_policy: PolicyMix::Mixed { greedy_probability: 0.7 },
        seed: 555,
        max_actions: 100,
    };
    let result_mixed = simulate_rollout(&state, &config_mixed).unwrap();
    assert!(result_mixed.completed_normally);
}
```

**Test 8: Integration with end-of-round resolution**
```rust
#[test]
fn test_end_of_round_resolution_applied() {
    // Create state with complete pattern lines
    let mut state = create_test_state();
    state.players[0].pattern_lines[0] = PatternLine {
        capacity: 1,
        color: Some(TileColor::Blue),
        count_filled: 1,
    };
    
    let config = RolloutConfig {
        active_player_policy: PolicyMix::AllGreedy,
        opponent_policy: PolicyMix::AllGreedy,
        seed: 777,
        max_actions: 100,
    };
    
    let result = simulate_rollout(&state, &config).unwrap();
    
    // After resolution:
    // 1. Pattern line should be cleared
    assert_eq!(result.final_state.players[0].pattern_lines[0].count_filled, 0);
    assert_eq!(result.final_state.players[0].pattern_lines[0].color, None);
    
    // 2. Wall should have tile placed
    assert!(result.final_state.players[0].wall[0][0]); // Blue at row 0, col 0
    
    // 3. Score should be updated (at least 1 point)
    assert!(result.final_state.players[0].score > 0);
}
```

---

## Edge Cases

### Edge Case 1: Rollout from nearly complete round

**Scenario:** Only 1-2 actions remain before round ends.

**Expected Behavior:**
- Rollout completes quickly (1-2 actions)
- End-of-round resolution still applies
- Statistics reflect minimal simulation

**Test:**
```rust
#[test]
fn test_rollout_from_nearly_complete_round() {
    // Create state with only center tiles remaining
    let mut state = create_test_state();
    state.factories = vec![HashMap::new(); 5]; // All factories empty
    state.center.tiles.insert(TileColor::Blue, 2);
    
    let config = RolloutConfig {
        active_player_policy: PolicyMix::AllGreedy,
        opponent_policy: PolicyMix::AllGreedy,
        seed: 888,
        max_actions: 100,
    };
    
    let result = simulate_rollout(&state, &config).unwrap();
    
    // Should complete with very few actions
    assert!(result.actions_simulated <= 2);
    assert!(result.completed_normally);
}
```

### Edge Case 2: State with complete pattern lines

**Scenario:** Rollout starts with some pattern lines already complete.

**Expected Behavior:**
- Rollout proceeds normally
- End-of-round resolves pattern lines and updates scores
- Final scores reflect tile placements

### Edge Case 3: Heavy floor penalties

**Scenario:** Both players accumulate many floor tiles during rollout.

**Expected Behavior:**
- Scores apply penalties correctly
- Scores clamp to 0 (cannot go negative)
- Tile conservation maintained

---

## Performance Considerations

### Optimization Strategies

1. **State Cloning:**
   - Rollouts clone the initial state
   - Acceptable for MVP (typical state ~5KB)
   - Future: consider arena allocation or copy-on-write

2. **Legal Action Caching:**
   - Currently recomputed each turn
   - Future: cache legal actions for repeated evaluations

3. **RNG Efficiency:**
   - Reuse single RNG instance across rollout
   - Avoid creating new RNG per action

4. **Early Termination:**
   - Not applicable for full rollouts (need complete round)
   - Relevant for Sprint 5B (time budgeting)

### Expected Performance

**Target:** 100-500 rollouts per second

**Factors:**
- State size: 2-player, 5 factories
- Actions per round: 10-20 typical
- Policy complexity: GreedyPolicy does simple heuristic scoring
- End-of-round: Scoring is O(walls) = O(25) per player

**Profiling Plan:** Measure in Sprint 5B during evaluation loop.

---

## Acceptance Criteria

- [ ] `simulate_rollout()` function implemented and exported
- [ ] Rollouts complete successfully from any valid game state
- [ ] Deterministic results with same seed
- [ ] Different seeds produce different results
- [ ] Integration with GreedyPolicy and RandomPolicy works
- [ ] Mixed policy mode functions correctly
- [ ] End-of-round resolution applied correctly
- [ ] Tile conservation maintained throughout
- [ ] Max actions safety limit prevents infinite loops
- [ ] All 8+ unit tests pass
- [ ] Documentation includes usage examples

---

## Files to Create/Modify

### New Files

```
rust/engine/src/
└── rules/
    ├── rollout.rs              (~250-300 lines)
    │   ├── RolloutConfig
    │   ├── RolloutResult  
    │   ├── RolloutError
    │   ├── simulate_rollout()
    │   ├── is_round_complete()
    │   └── select_action_with_policy()
    │
    └── tests/
        └── rollout_tests.rs    (~200 lines)
            └── 8 comprehensive tests
```

### Modified Files

```
rust/engine/src/
└── rules/
    └── mod.rs                  (~5 lines added)
        └── pub mod rollout;
        └── pub use rollout::*;
```

---

## Module Organization

**`rules/mod.rs`:**
```rust
mod constants;
mod legality;
mod wall_utils;
mod apply;
mod error;
mod invariants;
mod resolution;
mod scoring;
mod refill;
mod end_of_round;
mod rng;
mod policy;
mod filters;
mod generator;
mod rollout;  // NEW

#[cfg(test)]
mod tests;

pub use constants::*;
pub use legality::*;
pub use wall_utils::*;
pub use apply::*;
pub use error::*;
pub use invariants::*;
pub use resolution::*;
pub use scoring::*;
pub use refill::*;
pub use end_of_round::*;
pub use rng::*;
pub use policy::*;
pub use filters::*;
pub use generator::*;
pub use rollout::*;  // NEW
```

---

## Design Decisions

### Why Clone State?

**Decision:** Clone initial state rather than mutate in-place

**Rationale:**
- Rollouts are speculative simulations
- Original state must remain unchanged
- Allows parallel rollouts in future (Sprint 5B)
- Consistent with apply_action pattern from Sprint 01

**Trade-off:** Memory overhead (~5KB per rollout) vs safety and clarity

---

### Why Not Export to WASM Yet?

**Decision:** Keep rollout as internal Rust function (Sprint 5A)

**Rationale:**
- Rollouts are building blocks for evaluation (Sprint 5B)
- Direct WASM export not needed by UI
- Sprint 5B will export `evaluate_best_move()` which uses rollouts internally

**Future:** Could expose for debugging/testing in dev panel

---

### Why Reuse Sprint 4 Policies?

**Decision:** Use existing GreedyPolicy rather than implement new rollout-specific policy

**Rationale:**
- GreedyPolicy already exists and works well
- Heuristics (prefer pattern lines, avoid floor) are appropriate for rollouts
- Research recommends "fast biased policy" - GreedyPolicy fits this
- Reduces Sprint 5A scope
- Can enhance policies later without changing rollout infrastructure

---

### Why Track `completed_normally`?

**Decision:** Include flag indicating normal vs max_actions termination

**Rationale:**
- Diagnostics: detect when rollouts are cut short
- Quality control: evaluator can discard incomplete rollouts
- Debugging: helps identify policy or state issues

---

## Integration with Sprint 5B

Sprint 5B will use rollouts as follows:

```rust
// Pseudocode from Sprint 5B evaluator
for each candidate_action in shortlisted_actions:
    // Apply candidate action
    state_after_action = apply_action(state, candidate_action)?
    
    // Run N rollouts
    let mut utilities = Vec::new();
    for i in 0..rollouts_per_action:
        let config = RolloutConfig {
            active_player_policy: PolicyMix::AllGreedy,
            opponent_policy: PolicyMix::AllGreedy,
            seed: base_seed + i,
            max_actions: 100,
        };
        
        let result = simulate_rollout(&state_after_action, &config)?;
        
        // Compute utility (score differential from active player's perspective)
        let active_player_id = state.active_player_id;
        let utility = if active_player_id == 0 {
            result.player_0_score - result.player_1_score
        } else {
            result.player_1_score - result.player_0_score
        };
        
        utilities.push(utility);
    }
    
    // Compute EV for this action
    let ev = utilities.iter().sum::<i32>() as f64 / utilities.len() as f64;
    
    // Track best action
    if ev > best_ev {
        best_ev = ev;
        best_action = candidate_action.clone();
    }
```

---

## Related Documentation

- [Sprint 05: Master Document](Sprint_05_Best_Move_Evaluator_Tier2_Think_Longer.md)
- [Sprint 05B: Evaluator Core](Sprint_05B_Evaluator_Core.md) - Uses rollout infrastructure
- [Sprint 03C: End-of-Round Resolution](Sprint_03C_COMPLETED.md) - Called by rollouts
- [Sprint 04: Policy Infrastructure](Sprint_04_COMPLETED.md) - GreedyPolicy and RandomPolicy
- [Research Synthesis](../engineering/azul_best_move_algorithm_research_synthesis.md) - Section 5: Making rollouts work

---

## Next Steps

After completing Sprint 05A:
- Proceed to [Sprint 05B: Evaluator Core](Sprint_05B_Evaluator_Core.md)
- Rollout infrastructure will be used to evaluate candidate actions
- Sprint 5B adds action shortlisting, time budgeting, and WASM API
