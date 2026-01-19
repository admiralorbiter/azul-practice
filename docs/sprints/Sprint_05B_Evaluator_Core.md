# Sprint 05B — Evaluator Core + Action Shortlisting

**Status:** 📋 **PLANNED**  
**Prerequisites:** Sprint 05A (Rollout Simulation) complete  
**Dependencies:** Rollout simulation, action shortlisting, time budgeting  
**Complexity:** High

---

## Goal

Implement the complete evaluation engine with time budgeting, action shortlisting, and best-move selection using rollout-based Monte Carlo evaluation.

## Outcomes

- ✓ Action shortlisting heuristic to reduce candidate set
- ✓ Time-budgeted evaluation loop with best-so-far return
- ✓ EV calculation via rollout sampling
- ✓ Best action selection and user action grading
- ✓ WASM API for JavaScript integration
- ✓ Performance optimizations (caching, early cutoff)
- ✓ Deterministic evaluation with seed control

---

## Evaluation Algorithm Overview

The evaluator computes the **best move** for a given game state using Monte Carlo rollout sampling:

1. **Enumerate** all legal actions for the active player
2. **Shortlist** top N actions using cheap heuristic pre-scoring
3. **For each candidate action:**
   - Apply the action
   - Run K rollouts to end-of-round
   - Compute utility (score differential)
   - Calculate EV (expected value) = mean utility
4. **Select** action with highest EV
5. **Grade** user's action (if provided) by comparing EVs

**Key Features:**
- Time budgeting: return best-so-far if time expires
- Deterministic: same seed → same evaluation
- Efficient: shortlist reduces work from ~50 to ~15 actions

---

## Core Types

### EvaluatorParams

Configuration for evaluation.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EvaluatorParams {
    /// Time budget in milliseconds
    #[serde(default = "default_time_budget")]
    pub time_budget_ms: u64,
    
    /// Number of rollouts per candidate action
    #[serde(default = "default_rollouts_per_action")]
    pub rollouts_per_action: usize,
    
    /// Seed for deterministic evaluation
    pub evaluator_seed: u64,
    
    /// Number of actions to shortlist (0 = no shortlisting)
    #[serde(default = "default_shortlist_size")]
    pub shortlist_size: usize,
    
    /// Policies for rollout simulation
    #[serde(default)]
    pub rollout_config: RolloutPolicyConfig,
}

fn default_time_budget() -> u64 { 250 }
fn default_rollouts_per_action() -> usize { 10 }
fn default_shortlist_size() -> usize { 20 }

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RolloutPolicyConfig {
    pub active_player_policy: PolicyMix,
    pub opponent_policy: PolicyMix,
}

impl Default for RolloutPolicyConfig {
    fn default() -> Self {
        Self {
            active_player_policy: PolicyMix::AllGreedy,
            opponent_policy: PolicyMix::AllGreedy,
        }
    }
}
```

### EvaluationResult

Output from evaluation.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EvaluationResult {
    /// Best action found
    pub best_action: DraftAction,
    
    /// Expected value of best action
    pub best_action_ev: f64,
    
    /// Expected value of user's action (if provided)
    pub user_action_ev: Option<f64>,
    
    /// Delta between user and best (user_ev - best_ev)
    pub delta_ev: Option<f64>,
    
    /// Evaluation metadata and diagnostics
    pub metadata: EvaluationMetadata,
    
    /// All candidate actions with EVs (for dev panel)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub candidates: Option<Vec<CandidateAction>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EvaluationMetadata {
    /// Elapsed time in milliseconds
    pub elapsed_ms: u64,
    
    /// Total number of rollouts executed
    pub rollouts_run: usize,
    
    /// Number of candidate actions evaluated
    pub candidates_evaluated: usize,
    
    /// Total legal actions before shortlisting
    pub total_legal_actions: usize,
    
    /// Evaluator seed used
    pub seed: u64,
    
    /// Whether evaluation completed within time budget
    pub completed_within_budget: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CandidateAction {
    pub action: DraftAction,
    pub ev: f64,
    pub rollouts: usize,
}
```

### EvaluatorError

Error conditions during evaluation.

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvaluatorError {
    /// No legal actions available
    NoLegalActions,
    
    /// Invalid player ID
    InvalidPlayer(u8),
    
    /// Rollout simulation failed
    RolloutFailure(String),
    
    /// Action application failed
    ActionFailed(String),
    
    /// Invalid parameters
    InvalidParams(String),
}

impl std::fmt::Display for EvaluatorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EvaluatorError::NoLegalActions => write!(f, "No legal actions available"),
            EvaluatorError::InvalidPlayer(id) => write!(f, "Invalid player ID: {}", id),
            EvaluatorError::RolloutFailure(msg) => write!(f, "Rollout failed: {}", msg),
            EvaluatorError::ActionFailed(msg) => write!(f, "Action failed: {}", msg),
            EvaluatorError::InvalidParams(msg) => write!(f, "Invalid parameters: {}", msg),
        }
    }
}

impl std::error::Error for EvaluatorError {}
```

---

## Algorithm: evaluate_best_move

### Function Signature

```rust
pub fn evaluate_best_move(
    state: &State,
    player_id: u8,
    params: &EvaluatorParams,
) -> Result<EvaluationResult, EvaluatorError>
```

### Pseudocode

```
function evaluate_best_move(state, player_id, params) -> Result<EvaluationResult>:
    // 1. Validate inputs
    if player_id != 0 && player_id != 1:
        return Err(InvalidPlayer(player_id))
    
    // 2. Get all legal actions
    legal_actions = list_legal_actions(state, player_id)
    if legal_actions.is_empty():
        return Err(NoLegalActions)
    
    total_legal_actions = legal_actions.len()
    
    // 3. Shortlist candidates
    candidates = if params.shortlist_size > 0 && legal_actions.len() > params.shortlist_size:
                     shortlist_actions(state, legal_actions, params.shortlist_size)
                 else:
                     legal_actions  // Use all if few actions
    
    // 4. Initialize tracking
    start_time = Instant::now()
    best_action = None
    best_ev = -INFINITY
    candidate_results = []
    rollouts_run = 0
    candidates_evaluated = 0
    
    // 5. Evaluate each candidate
    for action in candidates:
        // Check time budget
        elapsed_ms = start_time.elapsed().as_millis() as u64
        if elapsed_ms >= params.time_budget_ms:
            break  // Time expired, return best-so-far
        
        // Apply action
        state_after_action = apply_action(state, action)?
        
        // Run rollouts
        utilities = []
        for i in 0..params.rollouts_per_action:
            // Derive unique seed for this rollout
            rollout_seed = params.evaluator_seed + rollouts_run as u64
            
            rollout_config = RolloutConfig {
                active_player_policy: params.rollout_config.active_player_policy,
                opponent_policy: params.rollout_config.opponent_policy,
                seed: rollout_seed,
                max_actions: 100,
            }
            
            // Simulate rollout
            result = simulate_rollout(&state_after_action, &rollout_config)?
            rollouts_run += 1
            
            // Compute utility from active player's perspective
            utility = if player_id == 0:
                          result.player_0_score - result.player_1_score
                      else:
                          result.player_1_score - result.player_0_score
            
            utilities.push(utility)
        
        // Compute EV
        ev = mean(utilities)
        
        // Track candidate
        candidate_results.push(CandidateAction {
            action: action.clone(),
            ev: ev,
            rollouts: utilities.len(),
        })
        
        // Update best
        if ev > best_ev:
            best_ev = ev
            best_action = Some(action.clone())
        
        candidates_evaluated += 1
    
    // 6. Check if we found any action
    if best_action.is_none():
        return Err(NoLegalActions)  // Shouldn't happen
    
    // 7. Build result
    elapsed_ms = start_time.elapsed().as_millis() as u64
    completed_within_budget = elapsed_ms < params.time_budget_ms
    
    Ok(EvaluationResult {
        best_action: best_action.unwrap(),
        best_action_ev: best_ev,
        user_action_ev: None,  // Computed separately if needed
        delta_ev: None,
        metadata: EvaluationMetadata {
            elapsed_ms,
            rollouts_run,
            candidates_evaluated,
            total_legal_actions,
            seed: params.evaluator_seed,
            completed_within_budget,
        },
        candidates: Some(candidate_results),
    })
```

### Helper: mean

```rust
fn mean(values: &[i32]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    let sum: i32 = values.iter().sum();
    sum as f64 / values.len() as f64
}
```

---

## Algorithm: Action Shortlisting

Reduce candidate set using fast heuristic scoring.

### Function Signature

```rust
pub fn shortlist_actions(
    state: &State,
    legal_actions: &[DraftAction],
    shortlist_size: usize,
) -> Vec<DraftAction>
```

### Heuristic Scoring

Score actions based on immediate characteristics (no simulation):

```rust
fn score_action_heuristic(state: &State, action: &DraftAction) -> f64 {
    let mut score = 0.0;
    
    // Factor 1: Destination (strongly prefer pattern lines over floor)
    match action.destination {
        Destination::PatternLine(row) => {
            score += 100.0;  // Base bonus for pattern line
            
            // Bonus for completing the pattern line
            let pattern_line = &state.players[state.active_player_id as usize]
                .pattern_lines[row];
            let tiles_taken = count_tiles_in_source(state, &action.source, action.color);
            let tiles_after = pattern_line.count_filled + tiles_taken;
            
            if tiles_after >= pattern_line.capacity {
                score += 50.0;  // Strong bonus for completion
            }
            
            // Bonus for higher rows (higher capacity = more valuable)
            score += row as f64 * 5.0;
        }
        Destination::Floor => {
            score += 0.0;  // No bonus, sometimes necessary
        }
    }
    
    // Factor 2: Number of tiles taken (more is generally better)
    let tiles_taken = count_tiles_in_source(state, &action.source, action.color);
    score += tiles_taken as f64 * 10.0;
    
    // Factor 3: Source preference (center gives first-player token)
    match action.source {
        ActionSource::Center => {
            if state.center.has_first_player_token {
                score -= 15.0;  // Penalty for taking first player marker
            }
        }
        ActionSource::Factory(_) => {
            score += 5.0;  // Slight bonus for factory (avoid center penalty)
        }
    }
    
    // Factor 4: Color availability (prefer colors we can still place)
    // Check if we can place this color in multiple pattern lines
    let player = &state.players[state.active_player_id as usize];
    let placeable_rows = count_placeable_rows(player, action.color);
    score += placeable_rows as f64 * 3.0;
    
    score
}
```

### Shortlisting Algorithm

```
function shortlist_actions(state, legal_actions, shortlist_size) -> Vec<DraftAction>:
    // 1. Score all actions
    scored_actions = []
    for action in legal_actions:
        score = score_action_heuristic(state, action)
        scored_actions.push((action, score))
    
    // 2. Sort by score descending
    scored_actions.sort_by(|a, b| b.1.cmp(a.1))
    
    // 3. Take top N
    shortlisted = scored_actions[0..min(shortlist_size, scored_actions.len())]
        .map(|(action, _)| action.clone())
        .collect()
    
    return shortlisted
```

---

## Algorithm: Grade User Action

Compute EV of user's action and compare to best.

### Function Signature

```rust
pub fn grade_user_action(
    state: &State,
    player_id: u8,
    user_action: &DraftAction,
    params: &EvaluatorParams,
    best_result: &EvaluationResult,
) -> Result<EvaluationResult, EvaluatorError>
```

### Implementation

```
function grade_user_action(state, player_id, user_action, params, best_result):
    // 1. Verify user action is legal
    legal_actions = list_legal_actions(state, player_id)
    if !legal_actions.contains(user_action):
        return Err(ActionFailed("User action is not legal"))
    
    // 2. Apply user action
    state_after_action = apply_action(state, user_action)?
    
    // 3. Run rollouts for user action
    utilities = []
    for i in 0..params.rollouts_per_action:
        rollout_seed = params.evaluator_seed + 1_000_000 + i as u64  // Offset to avoid collision
        
        rollout_config = RolloutConfig {
            active_player_policy: params.rollout_config.active_player_policy,
            opponent_policy: params.rollout_config.opponent_policy,
            seed: rollout_seed,
            max_actions: 100,
        }
        
        result = simulate_rollout(&state_after_action, &rollout_config)?
        
        utility = if player_id == 0:
                      result.player_0_score - result.player_1_score
                  else:
                      result.player_1_score - result.player_0_score
        
        utilities.push(utility)
    
    // 4. Compute user EV
    user_ev = mean(utilities)
    
    // 5. Compute delta
    delta_ev = user_ev - best_result.best_action_ev
    
    // 6. Build updated result
    Ok(EvaluationResult {
        user_action_ev: Some(user_ev),
        delta_ev: Some(delta_ev),
        ..best_result.clone()
    })
```

---

## WASM API Integration

Export evaluation functions to JavaScript.

### Rust WASM Functions

Add to `rust/engine/src/wasm_api.rs`:

```rust
#[wasm_bindgen]
pub fn evaluate_best_move(
    state_json: &str,
    player_id: u8,
    params_json: &str,
) -> String {
    let state: State = match serde_json::from_str(state_json) {
        Ok(s) => s,
        Err(e) => return json!({
            "error": format!("Invalid state JSON: {}", e)
        }).to_string(),
    };
    
    let params: EvaluatorParams = match serde_json::from_str(params_json) {
        Ok(p) => p,
        Err(e) => return json!({
            "error": format!("Invalid params JSON: {}", e)
        }).to_string(),
    };
    
    match crate::rules::evaluate_best_move(&state, player_id, &params) {
        Ok(result) => serde_json::to_string(&result).unwrap_or_else(|e| {
            json!({ "error": format!("Serialization error: {}", e) }).to_string()
        }),
        Err(e) => json!({
            "error": format!("Evaluation failed: {}", e)
        }).to_string(),
    }
}

#[wasm_bindgen]
pub fn grade_user_action(
    state_json: &str,
    player_id: u8,
    user_action_json: &str,
    params_json: &str,
) -> String {
    // Similar structure: parse inputs, call grade_user_action, serialize result
    // Combines evaluate_best_move + grade_user_action internally
    
    let state: State = match serde_json::from_str(state_json) {
        Ok(s) => s,
        Err(e) => return json!({ "error": format!("Invalid state JSON: {}", e) }).to_string(),
    };
    
    let user_action: DraftAction = match serde_json::from_str(user_action_json) {
        Ok(a) => a,
        Err(e) => return json!({ "error": format!("Invalid action JSON: {}", e) }).to_string(),
    };
    
    let params: EvaluatorParams = match serde_json::from_str(params_json) {
        Ok(p) => p,
        Err(e) => return json!({ "error": format!("Invalid params JSON: {}", e) }).to_string(),
    };
    
    // First evaluate best move
    let best_result = match crate::rules::evaluate_best_move(&state, player_id, &params) {
        Ok(r) => r,
        Err(e) => return json!({ "error": format!("Evaluation failed: {}", e) }).to_string(),
    };
    
    // Then grade user action
    match crate::rules::grade_user_action(&state, player_id, &user_action, &params, &best_result) {
        Ok(result) => serde_json::to_string(&result).unwrap_or_else(|e| {
            json!({ "error": format!("Serialization error: {}", e) }).to_string()
        }),
        Err(e) => json!({ "error": format!("Grading failed: {}", e) }).to_string(),
    }
}
```

### TypeScript Wrapper

Add to `web/src/wasm/evaluator.ts`:

```typescript
import { AzulEngine } from './engine';

export interface EvaluatorParams {
  timeBudgetMs?: number;
  rolloutsPerAction?: number;
  evaluatorSeed: number;
  shortlistSize?: number;
  rolloutConfig?: {
    activePlayerPolicy: PolicyMix;
    opponentPolicy: PolicyMix;
  };
}

export interface PolicyMix {
  type: 'AllRandom' | 'AllGreedy' | 'Mixed';
  greedyProbability?: number;
}

export interface EvaluationResult {
  bestAction: DraftAction;
  bestActionEv: number;
  userActionEv?: number;
  deltaEv?: number;
  metadata: EvaluationMetadata;
  candidates?: CandidateAction[];
}

export interface EvaluationMetadata {
  elapsedMs: number;
  rolloutsRun: number;
  candidatesEvaluated: number;
  totalLegalActions: number;
  seed: number;
  completedWithinBudget: boolean;
}

export interface CandidateAction {
  action: DraftAction;
  ev: number;
  rollouts: number;
}

export async function evaluateBestMove(
  engine: AzulEngine,
  state: State,
  playerId: number,
  params: EvaluatorParams
): Promise<EvaluationResult> {
  const stateJson = JSON.stringify(state);
  const paramsJson = JSON.stringify(params);
  
  const resultJson = engine.evaluate_best_move(stateJson, playerId, paramsJson);
  const result = JSON.parse(resultJson);
  
  if (result.error) {
    throw new Error(`Evaluation failed: ${result.error}`);
  }
  
  return result as EvaluationResult;
}

export async function gradeUserAction(
  engine: AzulEngine,
  state: State,
  playerId: number,
  userAction: DraftAction,
  params: EvaluatorParams
): Promise<EvaluationResult> {
  const stateJson = JSON.stringify(state);
  const userActionJson = JSON.stringify(userAction);
  const paramsJson = JSON.stringify(params);
  
  const resultJson = engine.grade_user_action(
    stateJson,
    playerId,
    userActionJson,
    paramsJson
  );
  const result = JSON.parse(resultJson);
  
  if (result.error) {
    throw new Error(`Grading failed: ${result.error}`);
  }
  
  return result as EvaluationResult;
}
```

---

## Test Requirements

### Unit Tests

**Test 1: Evaluation completes within budget**
```rust
#[test]
fn test_evaluation_within_time_budget() {
    let state = create_test_state();
    let params = EvaluatorParams {
        time_budget_ms: 500,
        rollouts_per_action: 5,
        evaluator_seed: 12345,
        shortlist_size: 10,
        rollout_config: RolloutPolicyConfig::default(),
    };
    
    let start = Instant::now();
    let result = evaluate_best_move(&state, 0, &params).unwrap();
    let elapsed = start.elapsed().as_millis();
    
    // Should complete within reasonable time
    assert!(elapsed < 600); // Some overhead allowed
    
    // Should have evaluated at least one action
    assert!(result.metadata.candidates_evaluated > 0);
}
```

**Test 2: Shortlisting reduces candidate set**
```rust
#[test]
fn test_action_shortlisting() {
    let state = create_state_with_many_actions(); // ~40 legal actions
    let params = EvaluatorParams {
        time_budget_ms: 1000,
        rollouts_per_action: 5,
        evaluator_seed: 67890,
        shortlist_size: 15,
        rollout_config: RolloutPolicyConfig::default(),
    };
    
    let result = evaluate_best_move(&state, 0, &params).unwrap();
    
    // Should have many legal actions
    assert!(result.metadata.total_legal_actions > 30);
    
    // Should have evaluated only shortlist size
    assert_eq!(result.metadata.candidates_evaluated, 15);
}
```

**Test 3: Deterministic evaluation**
```rust
#[test]
fn test_deterministic_evaluation() {
    let state = create_test_state();
    let params = EvaluatorParams {
        time_budget_ms: 250,
        rollouts_per_action: 10,
        evaluator_seed: 42,
        shortlist_size: 0, // Disable shortlisting for full determinism
        rollout_config: RolloutPolicyConfig::default(),
    };
    
    let result1 = evaluate_best_move(&state, 0, &params).unwrap();
    let result2 = evaluate_best_move(&state, 0, &params).unwrap();
    
    // Same seed should produce identical results
    assert_eq!(
        serde_json::to_string(&result1.best_action).unwrap(),
        serde_json::to_string(&result2.best_action).unwrap()
    );
    assert!((result1.best_action_ev - result2.best_action_ev).abs() < 0.001);
}
```

**Test 4: Different seeds produce different results**
```rust
#[test]
fn test_different_seeds_different_evaluations() {
    let state = create_test_state();
    let params1 = EvaluatorParams {
        time_budget_ms: 250,
        rollouts_per_action: 10,
        evaluator_seed: 111,
        shortlist_size: 20,
        rollout_config: RolloutPolicyConfig::default(),
    };
    let params2 = EvaluatorParams {
        evaluator_seed: 222,
        ..params1.clone()
    };
    
    let result1 = evaluate_best_move(&state, 0, &params1).unwrap();
    let result2 = evaluate_best_move(&state, 0, &params2).unwrap();
    
    // Different seeds should likely produce different EVs (probabilistic)
    // Note: Actions might be same if clearly dominant
    assert_ne!(
        (result1.best_action_ev * 100.0) as i32,
        (result2.best_action_ev * 100.0) as i32
    );
}
```

**Test 5: User action grading**
```rust
#[test]
fn test_grade_user_action() {
    let state = create_test_state();
    let params = EvaluatorParams {
        time_budget_ms: 250,
        rollouts_per_action: 10,
        evaluator_seed: 555,
        shortlist_size: 20,
        rollout_config: RolloutPolicyConfig::default(),
    };
    
    // Evaluate best move
    let best_result = evaluate_best_move(&state, 0, &params).unwrap();
    
    // Get a different legal action (not the best)
    let legal_actions = list_legal_actions(&state, 0);
    let user_action = legal_actions.iter()
        .find(|a| *a != &best_result.best_action)
        .unwrap()
        .clone();
    
    // Grade user action
    let graded = grade_user_action(&state, 0, &user_action, &params, &best_result).unwrap();
    
    // Should have user EV and delta
    assert!(graded.user_action_ev.is_some());
    assert!(graded.delta_ev.is_some());
    
    // Delta should be negative (user action worse than best)
    let delta = graded.delta_ev.unwrap();
    assert!(delta <= 0.0);
}
```

**Test 6: Best action is legal**
```rust
#[test]
fn test_best_action_is_legal() {
    let state = create_test_state();
    let params = EvaluatorParams {
        time_budget_ms: 250,
        rollouts_per_action: 10,
        evaluator_seed: 777,
        shortlist_size: 20,
        rollout_config: RolloutPolicyConfig::default(),
    };
    
    let result = evaluate_best_move(&state, 0, &params).unwrap();
    let legal_actions = list_legal_actions(&state, 0);
    
    // Best action must be in legal action set
    assert!(legal_actions.contains(&result.best_action));
}
```

**Test 7: No shortlisting with few actions**
```rust
#[test]
fn test_no_shortlisting_when_few_actions() {
    let state = create_state_with_few_actions(); // Only 5 legal actions
    let params = EvaluatorParams {
        time_budget_ms: 250,
        rollouts_per_action: 10,
        evaluator_seed: 888,
        shortlist_size: 20, // Larger than available
        rollout_config: RolloutPolicyConfig::default(),
    };
    
    let result = evaluate_best_move(&state, 0, &params).unwrap();
    
    // Should evaluate all actions when fewer than shortlist size
    assert_eq!(result.metadata.candidates_evaluated, 5);
    assert_eq!(result.metadata.total_legal_actions, 5);
}
```

**Test 8: Time budget cutoff**
```rust
#[test]
fn test_time_budget_cutoff() {
    let state = create_state_with_many_actions();
    let params = EvaluatorParams {
        time_budget_ms: 50, // Very short budget
        rollouts_per_action: 50, // Many rollouts per action
        evaluator_seed: 999,
        shortlist_size: 20,
        rollout_config: RolloutPolicyConfig::default(),
    };
    
    let start = Instant::now();
    let result = evaluate_best_move(&state, 0, &params).unwrap();
    let elapsed = start.elapsed().as_millis();
    
    // Should respect time budget (with some tolerance)
    assert!(elapsed < 200);
    
    // Likely didn't evaluate all candidates
    assert!(result.metadata.candidates_evaluated < 20);
    
    // Should still return a valid best action
    assert!(result.best_action_ev.is_finite());
}
```

---

## Performance Targets

### Target Metrics

**Default Budget (250ms):**
- Evaluate 15-20 shortlisted actions
- 10 rollouts per action
- Total: 150-200 rollouts
- ~1.25-1.67ms per rollout (achievable)

**Think Longer (750ms):**
- Evaluate 20-25 actions
- 20 rollouts per action
- Total: 400-500 rollouts
- ~1.5-1.9ms per rollout

**Max Budget (1500ms):**
- Evaluate 25-30 actions
- 30 rollouts per action
- Total: 750-900 rollouts
- ~1.67-2ms per rollout

### Profiling Points

1. **Shortlisting overhead:** Should be <5ms
2. **Rollout time:** 1-2ms per rollout typical
3. **State cloning:** ~0.1ms per clone
4. **JSON serialization:** Only at WASM boundary

---

## Acceptance Criteria

- [ ] `evaluate_best_move()` function implemented
- [ ] `grade_user_action()` function implemented
- [ ] Action shortlisting reduces candidate set appropriately
- [ ] Time budget respected (returns best-so-far)
- [ ] EV calculation correct (mean of rollout utilities)
- [ ] Deterministic with fixed seed
- [ ] WASM API exported and working
- [ ] TypeScript wrappers functional
- [ ] All 8+ unit tests pass
- [ ] Performance: 250ms budget achievable for typical scenarios

---

## Files to Create/Modify

### New Files

```
rust/engine/src/
└── rules/
    ├── evaluator.rs               (~500-600 lines)
    │   ├── EvaluatorParams
    │   ├── EvaluationResult
    │   ├── EvaluationMetadata
    │   ├── CandidateAction
    │   ├── EvaluatorError
    │   ├── evaluate_best_move()
    │   ├── grade_user_action()
    │   ├── shortlist_actions()
    │   ├── score_action_heuristic()
    │   └── helper functions
    │
    └── tests/
        └── evaluator_tests.rs     (~250 lines)

web/src/
└── wasm/
    └── evaluator.ts               (~120 lines)
        ├── Type definitions
        ├── evaluateBestMove()
        └── gradeUserAction()
```

### Modified Files

```
rust/engine/src/
├── wasm_api.rs                    (~100 lines added)
│   ├── evaluate_best_move export
│   └── grade_user_action export
│
└── rules/
    └── mod.rs                     (~3 lines added)
        └── pub mod evaluator;
        └── pub use evaluator::*;
```

---

## Related Documentation

- [Sprint 05A: Rollout Simulation](Sprint_05A_Rollout_Simulation.md) - Foundation for evaluation
- [Sprint 05C: Feedback + UI](Sprint_05C_Feedback_UI.md) - Uses evaluation results
- [Research Synthesis](../engineering/azul_best_move_algorithm_research_synthesis.md) - Section 4.2: Tier 2 rollouts
- [Spec: Best Move Evaluation](../specs/08_best_move_evaluation_and_feedback.md)

---

## Next Steps

After completing Sprint 05B:
- Proceed to [Sprint 05C: Feedback System + UI Integration](Sprint_05C_Feedback_UI.md)
- Sprint 5C adds rich feedback, grading, and complete UI integration
- The evaluator core from 5B provides the foundation for user experience
