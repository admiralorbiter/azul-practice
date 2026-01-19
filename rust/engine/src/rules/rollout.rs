use crate::model::{State, DraftAction};
use crate::rules::{
    list_legal_actions,
    apply_action,
    resolve_end_of_round,
    create_rng_from_seed,
    DraftPolicy,
    RandomPolicy,
    GreedyPolicy,
    PolicyMix,
};
use rand::Rng;
use serde::{Deserialize, Serialize};

/// Error conditions during rollout simulation
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

/// Configuration for a single rollout simulation
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

/// Output from a rollout simulation
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

/// Select an action using the specified policy mix
fn select_action_with_policy<R: Rng>(
    state: &State,
    legal_actions: &[DraftAction],
    policy_mix: PolicyMix,
    rng: &mut R,
) -> Option<DraftAction> {
    match policy_mix {
        PolicyMix::AllRandom => {
            RandomPolicy.select_action(state, legal_actions, rng)
        }
        PolicyMix::AllGreedy => {
            GreedyPolicy.select_action(state, legal_actions, rng)
        }
        PolicyMix::Mixed { greedy_ratio } => {
            let use_greedy = rng.gen::<f32>() < greedy_ratio;
            if use_greedy {
                GreedyPolicy.select_action(state, legal_actions, rng)
            } else {
                RandomPolicy.select_action(state, legal_actions, rng)
            }
        }
    }
}

/// Simulate game from current state to end of round
///
/// Takes a game state in the middle of a drafting round and simulates
/// play using policy bots until all factories and center are empty.
/// Then resolves end-of-round scoring.
///
/// # Arguments
///
/// * `initial_state` - Current game state
/// * `config` - Rollout configuration (policies, seed, limits)
///
/// # Returns
///
/// * `Ok(RolloutResult)` - Simulation completed successfully
/// * `Err(RolloutError)` - Simulation failed (deadlock, max actions, etc.)
///
/// # Example
///
/// ```no_run
/// use engine::{State, RolloutConfig, PolicyMix, simulate_rollout};
///
/// let state = State::new_test_state();
/// let config = RolloutConfig {
///     active_player_policy: PolicyMix::AllGreedy,
///     opponent_policy: PolicyMix::AllGreedy,
///     seed: 12345,
///     max_actions: 100,
/// };
///
/// let result = simulate_rollout(&state, &config).unwrap();
/// println!("Final score: P0={}, P1={}", result.player_0_score, result.player_1_score);
/// ```
pub fn simulate_rollout(
    initial_state: &State,
    config: &RolloutConfig,
) -> Result<RolloutResult, RolloutError> {
    // 1. Clone state (rollouts are speculative)
    let mut state = initial_state.clone();
    let mut rng = create_rng_from_seed(config.seed);
    let mut actions_simulated = 0;
    
    // 2. Simulate drafting phase
    loop {
        // Check termination: round complete
        if is_round_complete(&state) {
            break;
        }
        
        // Check termination: safety limit
        if actions_simulated >= config.max_actions {
            return Err(RolloutError::MaxActionsExceeded);
        }
        
        // Get legal actions for current player
        let legal_actions = list_legal_actions(&state, state.active_player_id);
        if legal_actions.is_empty() {
            return Err(RolloutError::Deadlock(
                format!("No legal actions but round not complete (player {})", 
                    state.active_player_id)
            ));
        }
        
        // Select action via policy
        let current_player = state.active_player_id;
        let policy_mix = if current_player == 0 {
            config.active_player_policy
        } else {
            config.opponent_policy
        };
        
        let action = select_action_with_policy(&state, &legal_actions, policy_mix, &mut rng)
            .ok_or_else(|| RolloutError::PolicyFailure(
                format!("Policy returned no action for player {}", current_player)
            ))?;
        
        // Apply action
        state = apply_action(&state, &action)
            .map_err(|e| RolloutError::IllegalAction(e.message.clone()))?;
        
        actions_simulated += 1;
    }
    
    // 3. Resolve end of round
    state = resolve_end_of_round(&state)
        .map_err(|e| RolloutError::IllegalAction(e.message.clone()))?;
    
    // 4. Return result
    Ok(RolloutResult {
        final_state: state.clone(),
        player_0_score: state.players[0].score,
        player_1_score: state.players[1].score,
        actions_simulated,
        completed_normally: true,
    })
}
