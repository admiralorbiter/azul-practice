use crate::model::{State, DraftAction, Destination, ActionSource, TileColor};
use crate::rules::{
    list_legal_actions,
    apply_action,
    simulate_rollout,
    RolloutConfig,
    PolicyMix,
    ActionFeatures,
    FeedbackBullet,
    Grade,
    count_pattern_lines_completed,
    calculate_floor_penalty_for_player,
    count_tiles_in_action,
    generate_feedback_bullets,
    compute_grade,
};
use serde::{Deserialize, Serialize};

#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant;

/// Error conditions during evaluation
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

/// Configuration for rollout policies
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

/// Parameters for evaluation
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

/// Candidate action with evaluation metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CandidateAction {
    pub action: DraftAction,
    pub ev: f64,
    pub rollouts: usize,
}

/// Metadata about the evaluation process
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EvaluationMetadata {
    pub elapsed_ms: u64,
    pub rollouts_run: usize,
    pub candidates_evaluated: usize,
    pub total_legal_actions: usize,
    pub seed: u64,
    pub completed_within_budget: bool,
}

/// Result of best-move evaluation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EvaluationResult {
    pub best_action: DraftAction,
    pub best_action_ev: f64,
    pub user_action_ev: Option<f64>,
    pub delta_ev: Option<f64>,
    pub metadata: EvaluationMetadata,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub candidates: Option<Vec<CandidateAction>>,
    
    // Feature tracking and feedback
    pub best_features: ActionFeatures,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_features: Option<ActionFeatures>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback: Option<Vec<FeedbackBullet>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grade: Option<Grade>,
}

/// Calculate mean of integer values
fn mean(values: &[i32]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    let sum: i32 = values.iter().sum();
    sum as f64 / values.len() as f64
}

/// Count total tiles of a color in a source
fn count_tiles_in_source(state: &State, source: &ActionSource, color: TileColor) -> u8 {
    match source {
        ActionSource::Factory(idx) => {
            state.factories.get(*idx)
                .and_then(|f| f.get(&color))
                .copied()
                .unwrap_or(0)
        }
        ActionSource::Center => {
            state.center.tiles.get(&color).copied().unwrap_or(0)
        }
    }
}

/// Count how many pattern lines can accept this color
fn count_placeable_rows(state: &State, player_id: u8, color: TileColor) -> usize {
    let player = &state.players[player_id as usize];
    let mut count = 0;
    
    for (row_idx, pattern_line) in player.pattern_lines.iter().enumerate() {
        // Check if row is available for this color
        if pattern_line.count_filled == 0 || pattern_line.color == Some(color) {
            // Check wall conflict
            let col = crate::rules::get_wall_column_for_color(row_idx, color);
            if !player.wall[row_idx][col] {
                count += 1;
            }
        }
    }
    
    count
}

/// Score an action using fast heuristics (no simulation)
fn score_action_heuristic(state: &State, action: &DraftAction) -> f64 {
    let mut score = 0.0;
    let player = &state.players[state.active_player_id as usize];
    
    // Factor 1: Destination preference
    match action.destination {
        Destination::PatternLine(row) => {
            score += 100.0;  // Strong preference for pattern lines
            
            let pattern_line = &player.pattern_lines[row];
            let tiles_taken = count_tiles_in_source(state, &action.source, action.color);
            let tiles_after = pattern_line.count_filled + tiles_taken;
            
            // Bonus for completing the line
            if tiles_after >= pattern_line.capacity {
                score += 50.0;
            }
            
            // Bonus for higher rows (more valuable)
            score += row as f64 * 5.0;
        }
        Destination::Floor => {
            score += 0.0;  // No bonus for floor
        }
    }
    
    // Factor 2: Number of tiles
    let tiles_taken = count_tiles_in_source(state, &action.source, action.color);
    score += tiles_taken as f64 * 10.0;
    
    // Factor 3: Source preference
    match action.source {
        ActionSource::Center => {
            if state.center.has_first_player_token {
                score -= 15.0;  // Penalty for first player token
            }
        }
        ActionSource::Factory(_) => {
            score += 5.0;  // Slight factory bonus
        }
    }
    
    // Factor 4: Color versatility
    let placeable_rows = count_placeable_rows(state, state.active_player_id, action.color);
    score += placeable_rows as f64 * 3.0;
    
    score
}

/// Shortlist top N actions using heuristic scoring
pub fn shortlist_actions(
    state: &State,
    legal_actions: &[DraftAction],
    shortlist_size: usize,
) -> Vec<DraftAction> {
    // Score all actions
    let mut scored: Vec<(DraftAction, f64)> = legal_actions
        .iter()
        .map(|action| (action.clone(), score_action_heuristic(state, action)))
        .collect();
    
    // Sort by score descending
    scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    
    // Take top N
    scored.into_iter()
        .take(shortlist_size)
        .map(|(action, _)| action)
        .collect()
}

/// Evaluate best move using rollout-based Monte Carlo sampling
///
/// Takes a game state and evaluates all legal actions by running Monte Carlo
/// rollout simulations. Returns the action with the highest expected value.
///
/// # Arguments
///
/// * `state` - Current game state
/// * `player_id` - Player whose turn it is (0 or 1)
/// * `params` - Evaluation parameters (time budget, rollouts, seed, etc.)
///
/// # Returns
///
/// * `Ok(EvaluationResult)` - Best action found with metrics
/// * `Err(EvaluatorError)` - Evaluation failed
///
/// # Example
///
/// ```no_run
/// use engine::{State, EvaluatorParams, RolloutPolicyConfig, evaluate_best_move};
///
/// let state = State::new_test_state();
/// let params = EvaluatorParams {
///     time_budget_ms: 250,
///     rollouts_per_action: 10,
///     evaluator_seed: 12345,
///     shortlist_size: 20,
///     rollout_config: RolloutPolicyConfig::default(),
/// };
///
/// let result = evaluate_best_move(&state, 0, &params).unwrap();
/// println!("Best action: {:?}, EV: {}", result.best_action, result.best_action_ev);
/// ```
pub fn evaluate_best_move(
    state: &State,
    player_id: u8,
    params: &EvaluatorParams,
) -> Result<EvaluationResult, EvaluatorError> {
    // 1. Validate inputs
    if player_id > 1 {
        return Err(EvaluatorError::InvalidPlayer(player_id));
    }
    
    // 2. Get all legal actions
    let legal_actions = list_legal_actions(state, player_id);
    if legal_actions.is_empty() {
        return Err(EvaluatorError::NoLegalActions);
    }
    
    let total_legal_actions = legal_actions.len();
    
    // 3. Shortlist candidates
    let candidates = if params.shortlist_size > 0 && legal_actions.len() > params.shortlist_size {
        shortlist_actions(state, &legal_actions, params.shortlist_size)
    } else {
        legal_actions
    };
    
    // 4. Initialize tracking
    #[cfg(not(target_arch = "wasm32"))]
    let start_time = Instant::now();
    
    let total_candidates = candidates.len();
    let mut best_action: Option<DraftAction> = None;
    let mut best_ev = f64::NEG_INFINITY;
    let mut best_features = ActionFeatures::default();
    let mut candidate_results = Vec::new();
    let mut rollouts_run = 0;
    let mut candidates_evaluated = 0;
    
    // 5. Evaluate each candidate
    for action in candidates {
        // Check time budget (skip in WASM where timing is not available)
        #[cfg(not(target_arch = "wasm32"))]
        {
            let elapsed_ms = start_time.elapsed().as_millis() as u64;
            if elapsed_ms >= params.time_budget_ms && candidates_evaluated > 0 {
                break;  // Time expired, return best so far
            }
        }
        
        // Apply action
        let state_after_action = apply_action(state, &action)
            .map_err(|e| EvaluatorError::ActionFailed(e.message.clone()))?;
        
        // Run rollouts and track features
        let mut utilities = Vec::new();
        let mut features = ActionFeatures::default();
        let player_before = &state_after_action.players[player_id as usize];
        
        for _i in 0..params.rollouts_per_action {
            // Unique seed per rollout
            let rollout_seed = params.evaluator_seed.wrapping_add(rollouts_run as u64);
            
            let rollout_config = RolloutConfig {
                active_player_policy: params.rollout_config.active_player_policy,
                opponent_policy: params.rollout_config.opponent_policy,
                seed: rollout_seed,
                max_actions: 100,
            };
            
            // Simulate
            let result = simulate_rollout(&state_after_action, &rollout_config)
                .map_err(|e| EvaluatorError::RolloutFailure(e.to_string()))?;
            
            rollouts_run += 1;
            
            // Compute utility from active player's perspective
            let utility = if player_id == 0 {
                result.player_0_score - result.player_1_score
            } else {
                result.player_1_score - result.player_0_score
            };
            
            utilities.push(utility);
            
            // Track features
            let player_after = &result.final_state.players[player_id as usize];
            
            let floor_penalty = calculate_floor_penalty_for_player(player_after);
            features.expected_floor_penalty += floor_penalty as f64;
            
            let completions = count_pattern_lines_completed(player_before, player_after);
            features.expected_completions += completions as f64;
            
            let tiles_to_floor = player_after.floor_line.tiles.len();
            features.expected_tiles_to_floor += tiles_to_floor as f64;
        }
        
        // Average features across rollouts
        let rollout_count = utilities.len() as f64;
        if rollout_count > 0.0 {
            features.expected_floor_penalty /= rollout_count;
            features.expected_completions /= rollout_count;
            features.expected_tiles_to_floor /= rollout_count;
        }
        
        // Static features
        features.tiles_acquired = count_tiles_in_action(state, &action);
        features.takes_first_player_token = matches!(action.source, ActionSource::Center) 
            && state.center.has_first_player_token;
        
        // Compute EV
        let ev = mean(&utilities);
        
        // Track candidate
        candidate_results.push(CandidateAction {
            action: action.clone(),
            ev,
            rollouts: utilities.len(),
        });
        
        // Update best
        if ev > best_ev {
            best_ev = ev;
            best_action = Some(action.clone());
            best_features = features.clone();
        }
        
        candidates_evaluated += 1;
    }
    
    // 6. Ensure we found an action
    let best_action = best_action.ok_or(EvaluatorError::NoLegalActions)?;
    
    // 7. Build result
    #[cfg(not(target_arch = "wasm32"))]
    let elapsed_ms = start_time.elapsed().as_millis() as u64;
    #[cfg(target_arch = "wasm32")]
    let elapsed_ms = 0; // Timing not available in WASM
    
    let completed_within_budget = candidates_evaluated >= total_candidates;
    
    Ok(EvaluationResult {
        best_action,
        best_action_ev: best_ev,
        user_action_ev: None,
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
        best_features,
        user_features: None,
        feedback: None,
        grade: None,
    })
}

/// Grade user's action by comparing its EV to the best action
///
/// Evaluates the user's action using rollout sampling and compares it to
/// the best action found by `evaluate_best_move`.
///
/// # Arguments
///
/// * `state` - Current game state
/// * `player_id` - Player whose turn it is (0 or 1)
/// * `user_action` - Action chosen by the user
/// * `params` - Evaluation parameters
/// * `best_result` - Result from `evaluate_best_move`
///
/// # Returns
///
/// * `Ok(EvaluationResult)` - Updated result with user action EV and delta
/// * `Err(EvaluatorError)` - Grading failed
pub fn grade_user_action(
    state: &State,
    player_id: u8,
    user_action: &DraftAction,
    params: &EvaluatorParams,
    best_result: &EvaluationResult,
) -> Result<EvaluationResult, EvaluatorError> {
    // 1. Verify user action is legal
    let legal_actions = list_legal_actions(state, player_id);
    if !legal_actions.contains(user_action) {
        return Err(EvaluatorError::ActionFailed("User action is not legal".to_string()));
    }
    
    // 2. Check if user action was already evaluated in candidates
    // If so, reuse that EV for consistency (avoids seed variance)
    let user_ev_from_candidates = if let Some(candidates) = &best_result.candidates {
        candidates.iter().find(|c| &c.action == user_action).map(|c| c.ev)
    } else {
        None
    };
    
    // 3. Apply user action
    let state_after_action = apply_action(state, user_action)
        .map_err(|e| EvaluatorError::ActionFailed(e.message.clone()))?;
    
    // 4. Run rollouts to track features (always needed for feedback)
    // If user action was in candidates, we'll use the original EV but still need features
    let mut utilities = Vec::new();
    let mut user_features = ActionFeatures::default();
    let player_before = &state_after_action.players[player_id as usize];
    
    for i in 0..params.rollouts_per_action {
        // Offset seed to avoid collision with best-move evaluation
        let rollout_seed = params.evaluator_seed.wrapping_add(1_000_000 + i as u64);
        
        let rollout_config = RolloutConfig {
            active_player_policy: params.rollout_config.active_player_policy,
            opponent_policy: params.rollout_config.opponent_policy,
            seed: rollout_seed,
            max_actions: 100,
        };
        
        let result = simulate_rollout(&state_after_action, &rollout_config)
            .map_err(|e| EvaluatorError::RolloutFailure(e.to_string()))?;
        
        let utility = if player_id == 0 {
            result.player_0_score - result.player_1_score
        } else {
            result.player_1_score - result.player_0_score
        };
        
        utilities.push(utility);
        
        // Track features
        let player_after = &result.final_state.players[player_id as usize];
        
        let floor_penalty = calculate_floor_penalty_for_player(player_after);
        user_features.expected_floor_penalty += floor_penalty as f64;
        
        let completions = count_pattern_lines_completed(player_before, player_after);
        user_features.expected_completions += completions as f64;
        
        let tiles_to_floor = player_after.floor_line.tiles.len();
        user_features.expected_tiles_to_floor += tiles_to_floor as f64;
    }
    
    // Average features across rollouts
    let rollout_count = utilities.len() as f64;
    if rollout_count > 0.0 {
        user_features.expected_floor_penalty /= rollout_count;
        user_features.expected_completions /= rollout_count;
        user_features.expected_tiles_to_floor /= rollout_count;
    }
    
    // Static features
    user_features.tiles_acquired = count_tiles_in_action(state, user_action);
    user_features.takes_first_player_token = matches!(user_action.source, ActionSource::Center) 
        && state.center.has_first_player_token;
    
    // 5. Compute user EV
    // Use EV from original evaluation if available, otherwise use new rollouts
    let user_ev = user_ev_from_candidates.unwrap_or_else(|| mean(&utilities));
    
    // 6. Compute delta
    let delta_ev = user_ev - best_result.best_action_ev;
    
    // 7. Compute grade
    let grade = compute_grade(delta_ev);
    
    // 8. Generate feedback
    let feedback = generate_feedback_bullets(&user_features, &best_result.best_features);
    
    // 9. Return updated result
    Ok(EvaluationResult {
        user_action_ev: Some(user_ev),
        delta_ev: Some(delta_ev),
        user_features: Some(user_features),
        feedback: Some(feedback),
        grade: Some(grade),
        ..best_result.clone()
    })
}
