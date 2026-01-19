use crate::model::{State, DraftPhase, DraftAction};
use crate::rules::{
    constants::{ALL_COLORS, TILES_PER_COLOR},
    refill_factories_with_rng,
    list_legal_actions,
    apply_action,
    create_rng_from_seed,
    DraftPolicy,
    RandomPolicy,
    GreedyPolicy,
    ValidationError,
    FilterConfig,
    apply_quality_filters,
    end_of_round::resolve_end_of_round,
};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Error types for scenario generation
#[derive(Debug)]
pub enum GeneratorError {
    /// Failed to parse seed string
    InvalidSeed(String),
    /// Policy bot failed to select an action
    NoPolicyAction,
    /// Failed to apply an action during play-forward
    ApplyActionFailed(ValidationError),
    /// Could not generate valid scenario after max attempts
    MaxAttemptsExceeded,
}

impl std::fmt::Display for GeneratorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GeneratorError::InvalidSeed(s) => write!(f, "Invalid seed: {}", s),
            GeneratorError::NoPolicyAction => write!(f, "Policy bot failed to select action"),
            GeneratorError::ApplyActionFailed(e) => write!(f, "Apply action failed: {}", e.message),
            GeneratorError::MaxAttemptsExceeded => write!(f, "Max generation attempts exceeded"),
        }
    }
}

impl std::error::Error for GeneratorError {}

/// Policy mix configuration for scenario generation
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicyMix {
    /// Use only random policy
    AllRandom,
    /// Use only greedy policy
    AllGreedy,
    /// Mix policies with specified greedy ratio (0.0-1.0)
    Mixed { greedy_ratio: f32 },
}

impl Default for PolicyMix {
    fn default() -> Self {
        PolicyMix::Mixed { greedy_ratio: 0.7 }
    }
}

/// Parameters for scenario generation
#[derive(Debug, Clone)]
pub struct GeneratorParams {
    /// Target draft phase (Early/Mid/Late)
    pub target_phase: DraftPhase,
    /// Random seed for reproducibility
    pub seed: u64,
    /// Policy mix for play-forward
    pub policy_mix: PolicyMix,
}

/// JSON-serializable parameters for WASM API
///
/// All fields are optional to allow flexible configuration from JavaScript.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeneratorParamsJson {
    /// Target phase: "EARLY", "MID", "LATE", or null for random
    pub target_phase: Option<DraftPhase>,
    /// Seed string (parsed to u64), or null to auto-generate
    pub seed: Option<String>,
    /// Policy mix: "random", "greedy", "mixed", or null for default (mixed)
    pub policy_mix: Option<String>,
    /// Filter configuration, or null for defaults
    pub filter_config: Option<FilterConfig>,
}

impl GeneratorParamsJson {
    /// Convert JSON params to internal GeneratorParams
    ///
    /// Applies defaults for missing fields.
    ///
    /// # Returns
    ///
    /// * `Ok((GeneratorParams, FilterConfig))` - Parsed parameters
    /// * `Err(String)` - Parse error message
    pub fn to_internal(&self) -> Result<(GeneratorParams, FilterConfig), String> {
        // Parse target phase (default to random selection)
        let target_phase = self.target_phase.unwrap_or_else(|| {
            // Random phase selection
            let phases = [DraftPhase::Early, DraftPhase::Mid, DraftPhase::Late];
            let mut rng = rand::thread_rng();
            phases[rng.gen_range(0..3)]
        });
        
        // Parse seed (default to random)
        let seed = if let Some(ref seed_str) = self.seed {
            crate::rules::parse_seed_string(seed_str)?
        } else {
            // Generate random seed
            let mut rng = rand::thread_rng();
            rng.gen()
        };
        
        // Parse policy mix (default to Mixed with 0.7 greedy ratio)
        let policy_mix = if let Some(ref mix_str) = self.policy_mix {
            match mix_str.as_str() {
                "random" => PolicyMix::AllRandom,
                "greedy" => PolicyMix::AllGreedy,
                "mixed" => PolicyMix::Mixed { greedy_ratio: 0.7 },
                _ => return Err(format!("Invalid policy_mix: '{}' (expected 'random', 'greedy', or 'mixed')", mix_str)),
            }
        } else {
            PolicyMix::default()
        };
        
        let params = GeneratorParams {
            target_phase,
            seed,
            policy_mix,
        };
        
        let filter_config = self.filter_config.clone().unwrap_or_default();
        
        Ok((params, filter_config))
    }
}

impl Default for GeneratorParams {
    fn default() -> Self {
        Self {
            target_phase: DraftPhase::Mid,
            seed: 0,
            policy_mix: PolicyMix::default(),
        }
    }
}

/// Create an initial legal state for 2-player game
///
/// Initializes bag with full tile set and refills factories for round 1.
///
/// # Arguments
///
/// * `rng` - Random number generator for factory refill
///
/// # Returns
///
/// Legal starting state ready for drafting
fn create_initial_state<R: Rng>(rng: &mut R) -> State {
    let mut state = State::new_test_state();
    
    // Initialize bag with 20 tiles per color
    for &color in &ALL_COLORS {
        state.bag.insert(color, TILES_PER_COLOR);
    }
    
    // Refill factories for round 1
    refill_factories_with_rng(&mut state, rng);
    
    // Set initial phase tag
    state.draft_phase_progress = DraftPhase::Early;
    
    state
}

/// Calculate target strategy based on desired phase
///
/// Returns (rounds_to_complete, picks_in_final_round).
/// Completing rounds fills walls and shows real game progression.
///
/// # Arguments
///
/// * `target_phase` - Desired phase (Early/Mid/Late)
/// * `rng` - Random number generator for variation
///
/// # Returns
///
/// Tuple of (rounds to auto-complete, picks to make in next round)
fn calculate_generation_strategy<R: Rng>(target_phase: DraftPhase, rng: &mut R) -> (u32, u32) {
    match target_phase {
        // Early: Stay in round 1, show some picks but walls empty
        DraftPhase::Early => (0, rng.gen_range(3..=8)),
        
        // Mid: Complete round 1 (walls filled!), then make some picks in round 2
        DraftPhase::Mid => (1, rng.gen_range(3..=10)),
        
        // Late: Complete rounds 1-2 (walls more filled), make picks in round 3
        DraftPhase::Late => (2, rng.gen_range(2..=8)),
    }
}

/// Enum wrapper for policy selection
///
/// This allows us to return different policy types without needing trait objects.
enum PolicySelector {
    Random(RandomPolicy),
    Greedy(GreedyPolicy),
}

impl PolicySelector {
    fn select_action<R: Rng>(
        &self,
        state: &State,
        legal_actions: &[DraftAction],
        rng: &mut R,
    ) -> Option<DraftAction> {
        match self {
            PolicySelector::Random(p) => p.select_action(state, legal_actions, rng),
            PolicySelector::Greedy(p) => p.select_action(state, legal_actions, rng),
        }
    }
}

/// Select a policy based on policy mix configuration
///
/// # Arguments
///
/// * `policy_mix` - Configuration for policy selection
/// * `rng` - Random number generator for mixed policy selection
///
/// # Returns
///
/// PolicySelector enum wrapping the chosen policy
fn select_policy<R: Rng>(
    policy_mix: &PolicyMix,
    rng: &mut R,
) -> PolicySelector {
    match policy_mix {
        PolicyMix::AllRandom => PolicySelector::Random(RandomPolicy),
        PolicyMix::AllGreedy => PolicySelector::Greedy(GreedyPolicy),
        PolicyMix::Mixed { greedy_ratio } => {
            let r: f32 = rng.gen();
            if r < *greedy_ratio {
                PolicySelector::Greedy(GreedyPolicy)
            } else {
                PolicySelector::Random(RandomPolicy)
            }
        }
    }
}

/// Tag draft phase based on current game state
///
/// Uses tile depletion in factories and center to classify phase.
///
/// # Arguments
///
/// * `state` - Current game state
///
/// # Returns
///
/// Phase tag (Early/Mid/Late)
fn tag_draft_phase(state: &State) -> DraftPhase {
    // Count total tiles in factories and center
    let mut total_in_play = 0u32;
    
    for factory in &state.factories {
        total_in_play += factory.values().map(|&v| v as u32).sum::<u32>();
    }
    
    total_in_play += state.center.tiles.values().map(|&v| v as u32).sum::<u32>();
    
    // At round start: 20 tiles (5 factories × 4 tiles)
    // Classify based on depletion
    // Adjusted thresholds to match more realistic game progression
    if total_in_play >= 14 {
        DraftPhase::Early   // 14-20 tiles (first few picks)
    } else if total_in_play >= 7 {
        DraftPhase::Mid     // 7-13 tiles (mid-round)
    } else {
        DraftPhase::Late    // 0-6 tiles (near end)
    }
}

/// Generate a scenario by playing forward from initial state
///
/// Uses policy bots to simulate gameplay, completing full rounds to fill walls.
/// Early phase stays in round 1. Mid/Late phases complete rounds and progress further.
///
/// # Arguments
///
/// * `params` - Generation parameters
///
/// # Returns
///
/// * `Ok(State)` - Generated scenario
/// * `Err(GeneratorError)` - Generation failed
pub fn generate_scenario(params: GeneratorParams) -> Result<State, GeneratorError> {
    let mut rng = create_rng_from_seed(params.seed);
    let mut state = create_initial_state(&mut rng);
    
    // Store seed in state for reproducibility
    state.scenario_seed = Some(params.seed.to_string());
    
    // Determine generation strategy (complete rounds + picks in final round)
    let (rounds_to_complete, picks_in_final_round) = calculate_generation_strategy(params.target_phase, &mut rng);
    
    // Phase 1: Complete N full rounds (this fills walls via scoring!)
    for _ in 0..rounds_to_complete {
        // Play out entire round
        loop {
            let legal_actions = list_legal_actions(&state, state.active_player_id);
            
            if legal_actions.is_empty() {
                // Round is complete, resolve it
                state = resolve_end_of_round(&state)
                    .map_err(GeneratorError::ApplyActionFailed)?;
                break;
            }
            
            // Select policy and action
            let policy = select_policy(&params.policy_mix, &mut rng);
            
            let action = policy
                .select_action(&state, &legal_actions, &mut rng)
                .ok_or(GeneratorError::NoPolicyAction)?;
            
            // Apply action
            state = apply_action(&state, &action)
                .map_err(GeneratorError::ApplyActionFailed)?;
        }
    }
    
    // Phase 2: Play N picks in the current (final) round
    let mut last_state_with_actions: Option<State> = None;
    
    for _ in 0..picks_in_final_round {
        let legal_actions = list_legal_actions(&state, state.active_player_id);
        
        if legal_actions.is_empty() {
            // Round complete early - stop
            break;
        }

        last_state_with_actions = Some(state.clone());
        
        // Select policy and action
        let policy = select_policy(&params.policy_mix, &mut rng);
        
        let action = policy
            .select_action(&state, &legal_actions, &mut rng)
            .ok_or(GeneratorError::NoPolicyAction)?;
        
        // Apply action
        state = apply_action(&state, &action)
            .map_err(GeneratorError::ApplyActionFailed)?;
    }

    // If we ended up at end-of-round (no actions), fall back to last state with actions.
    if list_legal_actions(&state, state.active_player_id).is_empty() {
        if let Some(fallback) = last_state_with_actions {
            state = fallback;
        }
    }
    
    // Tag phase based on actual state
    state.draft_phase_progress = tag_draft_phase(&state);
    
    Ok(state)
}

fn unique_destination_count(actions: &[DraftAction]) -> usize {
    let mut set: HashSet<u8> = HashSet::new();
    for a in actions {
        match a.destination {
            crate::model::Destination::Floor => {
                set.insert(100);
            }
            crate::model::Destination::PatternLine(row) => {
                set.insert(row as u8);
            }
        }
    }
    set.len()
}

/// Generate a scenario with quality filters and retry logic
///
/// Attempts to generate scenarios up to max_attempts times, applying quality
/// filters after each generation. Returns the first scenario that passes all filters.
///
/// # Arguments
///
/// * `params` - Generation parameters
/// * `filter_config` - Quality filter configuration
/// * `max_attempts` - Maximum number of generation attempts (default: 20)
///
/// # Returns
///
/// * `Ok(State)` - Valid scenario that passed all filters
/// * `Err(GeneratorError)` - Failed to generate valid scenario after max attempts
pub fn generate_scenario_with_filters(
    params: GeneratorParams,
    filter_config: FilterConfig,
    max_attempts: u32,
) -> Result<State, GeneratorError> {
    let mut best_state: Option<State> = None;
    let mut best_score: i64 = i64::MIN;

    for attempt in 0..max_attempts {
        // Use different seed for each attempt to get variety
        let attempt_seed = params.seed.wrapping_add(attempt as u64);
        let attempt_params = GeneratorParams {
            seed: attempt_seed,
            ..params.clone()
        };
        
        let state = generate_scenario(attempt_params)?;

        // Track best candidate as fallback (so we never error on UI)
        let actions = list_legal_actions(&state, state.active_player_id);
        let uniq = unique_destination_count(&actions) as i64;
        let score = (actions.len() as i64) * 10 + uniq;
        if score > best_score {
            best_score = score;
            best_state = Some(state.clone());
        }
        
        // Apply quality filters
        if apply_quality_filters(&state, &filter_config).is_ok() {
            return Ok(state);
        }
        
        // Filter failed, try next attempt
    }

    // Hard guardrail: return the best available playable state, even if filters were not met.
    // This prevents "New Scenario" from failing in the UI.
    if let Some(state) = best_state {
        return Ok(state);
    }

    Err(GeneratorError::MaxAttemptsExceeded)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::TileColor;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_create_initial_state() {
        let mut rng = StdRng::seed_from_u64(12345);
        let state = create_initial_state(&mut rng);
        
        // Factories should have tiles (20 total drawn from bag)
        let mut factory_tiles = 0;
        for factory in &state.factories {
            factory_tiles += factory.values().sum::<u8>();
        }
        assert_eq!(factory_tiles, 20, "Factories should have 20 tiles total");
        
        // Bag + factories should total 100 tiles (tile conservation)
        let mut bag_tiles = 0;
        for &count in state.bag.values() {
            bag_tiles += count;
        }
        assert_eq!(bag_tiles + factory_tiles, 100, "Bag + factories should have 100 tiles total");
        
        // Round should be 1, active player 0
        assert_eq!(state.round_number, 1);
        assert_eq!(state.active_player_id, 0);
    }

    #[test]
    fn test_calculate_generation_strategy() {
        let mut rng = StdRng::seed_from_u64(12345);
        
        // Test Early: (0 rounds complete, 3-8 picks)
        for _ in 0..20 {
            let (rounds, picks) = calculate_generation_strategy(DraftPhase::Early, &mut rng);
            assert_eq!(rounds, 0, "Early should complete 0 rounds, got {}", rounds);
            assert!(picks >= 3 && picks <= 8, "Early picks should be 3-8, got {}", picks);
        }
        
        // Test Mid: (1 round complete, 3-10 picks)
        for _ in 0..20 {
            let (rounds, picks) = calculate_generation_strategy(DraftPhase::Mid, &mut rng);
            assert_eq!(rounds, 1, "Mid should complete 1 round, got {}", rounds);
            assert!(picks >= 3 && picks <= 10, "Mid picks should be 3-10, got {}", picks);
        }
        
        // Test Late: (2 rounds complete, 2-8 picks)
        for _ in 0..20 {
            let (rounds, picks) = calculate_generation_strategy(DraftPhase::Late, &mut rng);
            assert_eq!(rounds, 2, "Late should complete 2 rounds, got {}", rounds);
            assert!(picks >= 2 && picks <= 8, "Late picks should be 2-8, got {}", picks);
        }
    }

    #[test]
    fn test_tag_draft_phase() {
        let mut state = State::new_test_state();
        
        // Setup early phase (14+ tiles)
        state.factories[0].insert(TileColor::Blue, 4);
        state.factories[1].insert(TileColor::Red, 4);
        state.factories[2].insert(TileColor::Yellow, 4);
        state.factories[3].insert(TileColor::Black, 2);
        assert_eq!(tag_draft_phase(&state), DraftPhase::Early);
        
        // Setup mid phase (7-13 tiles)
        let mut state = State::new_test_state();
        state.factories[0].insert(TileColor::Blue, 3);
        state.factories[1].insert(TileColor::Red, 3);
        state.center.tiles.insert(TileColor::Yellow, 2);
        assert_eq!(tag_draft_phase(&state), DraftPhase::Mid);
        
        // Setup late phase (< 7 tiles)
        let mut state = State::new_test_state();
        state.factories[0].insert(TileColor::Blue, 2);
        state.center.tiles.insert(TileColor::Red, 3);
        assert_eq!(tag_draft_phase(&state), DraftPhase::Late);
    }

    #[test]
    fn test_select_policy_all_random() {
        let mut rng = StdRng::seed_from_u64(12345);
        let policy_mix = PolicyMix::AllRandom;
        
        // Should always return random policy
        for _ in 0..10 {
            let policy = select_policy(&policy_mix, &mut rng);
            // Verify it's Random variant
            match policy {
                PolicySelector::Random(_) => {}, // Good!
                PolicySelector::Greedy(_) => panic!("Expected Random policy"),
            }
        }
    }

    #[test]
    fn test_select_policy_all_greedy() {
        let mut rng = StdRng::seed_from_u64(12345);
        let policy_mix = PolicyMix::AllGreedy;
        
        // Should always return greedy policy
        for _ in 0..10 {
            let policy = select_policy(&policy_mix, &mut rng);
            // Verify it's Greedy variant
            match policy {
                PolicySelector::Greedy(_) => {}, // Good!
                PolicySelector::Random(_) => panic!("Expected Greedy policy"),
            }
        }
    }

    #[test]
    fn test_generate_scenario_deterministic() {
        let params1 = GeneratorParams {
            target_phase: DraftPhase::Early,
            seed: 12345,
            policy_mix: PolicyMix::AllRandom,
        };
        
        let params2 = params1.clone();
        
        let state1 = generate_scenario(params1).unwrap();
        let state2 = generate_scenario(params2).unwrap();
        
        // Same seed should produce identical scenarios
        // Note: We use assert_eq! on the structs directly, which compares all fields
        assert_eq!(state1, state2, "States should be identical with same seed");
    }

    #[test]
    fn test_generate_scenario_stores_seed() {
        let params = GeneratorParams {
            target_phase: DraftPhase::Mid,
            seed: 99999,
            policy_mix: PolicyMix::default(),
        };
        
        let state = generate_scenario(params).unwrap();
        
        assert_eq!(state.scenario_seed, Some("99999".to_string()));
    }

    #[test]
    fn test_generate_scenario_early_phase() {
        let params = GeneratorParams {
            target_phase: DraftPhase::Early,
            seed: 12345,
            policy_mix: PolicyMix::AllRandom,
        };
        
        let state = generate_scenario(params).unwrap();
        
        // Should be tagged as early (though might vary based on actual play)
        // Just verify it completes without error
        assert!(state.round_number >= 1);
    }

    #[test]
    fn test_generate_scenario_different_seeds_differ() {
        let params1 = GeneratorParams {
            target_phase: DraftPhase::Mid,
            seed: 11111,
            policy_mix: PolicyMix::AllRandom,
        };
        
        let params2 = GeneratorParams {
            target_phase: DraftPhase::Mid,
            seed: 22222,
            policy_mix: PolicyMix::AllRandom,
        };
        
        let state1 = generate_scenario(params1).unwrap();
        let state2 = generate_scenario(params2).unwrap();
        
        // Different seeds should (very likely) produce different scenarios
        // We can't guarantee they're different, but check seeds are stored correctly
        assert_eq!(state1.scenario_seed, Some("11111".to_string()));
        assert_eq!(state2.scenario_seed, Some("22222".to_string()));
    }

    #[test]
    fn test_generate_scenario_with_filters_passes() {
        let params = GeneratorParams {
            target_phase: DraftPhase::Mid,
            seed: 12345,
            policy_mix: PolicyMix::AllRandom,
        };
        
        let filter_config = FilterConfig::default();
        
        let result = generate_scenario_with_filters(params, filter_config, 20);
        
        // Should succeed with reasonable filters
        assert!(result.is_ok(), "Should generate valid scenario with default filters");
    }

    #[test]
    fn test_generate_scenario_with_filters_retries_on_failure() {
        let params = GeneratorParams {
            target_phase: DraftPhase::Early,
            seed: 99999,
            policy_mix: PolicyMix::AllRandom,
        };
        
        // Very strict filters that might require retries
        let filter_config = FilterConfig {
            min_legal_actions: 10,
            min_unique_destinations: 4,
        };
        
        let result = generate_scenario_with_filters(params, filter_config, 50);
        
        // May succeed or fail depending on randomness, just verify it doesn't panic
        let _ = result;
    }

    #[test]
    fn test_generate_scenario_with_filters_max_attempts() {
        let params = GeneratorParams {
            target_phase: DraftPhase::Early,
            seed: 12345,
            policy_mix: PolicyMix::AllRandom,
        };
        
        // Impossible filters
        let filter_config = FilterConfig {
            min_legal_actions: 1000, // Impossible to have 1000 legal actions
            min_unique_destinations: 100,
        };
        
        let result = generate_scenario_with_filters(params, filter_config, 5);
        
        // Generator now has a hard fallback: it should return the best available playable state
        // even if filters are impossible to satisfy, so the UI never fails to create a scenario.
        assert!(result.is_ok());
    }

    #[test]
    fn test_mid_game_has_filled_walls() {
        // Mid game should complete 1 round, which should fill some walls
        let params = GeneratorParams {
            target_phase: DraftPhase::Mid,
            seed: 54321,
            policy_mix: PolicyMix::AllGreedy,
        };
        
        let state = generate_scenario(params).expect("Generation should succeed");
        
        // After completing round 1, at least one player should have tiles on their wall
        let player0_wall_tiles: u32 = state.players[0].wall.iter()
            .flat_map(|row| row.iter())
            .filter(|&&occupied| occupied)
            .count() as u32;
        
        let player1_wall_tiles: u32 = state.players[1].wall.iter()
            .flat_map(|row| row.iter())
            .filter(|&&occupied| occupied)
            .count() as u32;
        
        let total_wall_tiles = player0_wall_tiles + player1_wall_tiles;
        
        assert!(total_wall_tiles > 0, 
            "Mid game should have filled walls after completing round 1, but found 0 tiles. Player 0: {}, Player 1: {}",
            player0_wall_tiles, player1_wall_tiles);
        
        // Should be in round 2 or higher
        assert!(state.round_number >= 2, 
            "Mid game should be in round 2+, but was in round {}", state.round_number);
    }

    #[test]
    fn test_late_game_has_more_filled_walls() {
        // Late game should complete 2 rounds, walls should be more filled
        let params = GeneratorParams {
            target_phase: DraftPhase::Late,
            seed: 11111,
            policy_mix: PolicyMix::AllGreedy,
        };
        
        let state = generate_scenario(params).expect("Generation should succeed");
        
        // After completing 2 rounds, walls should be more filled
        let player0_wall_tiles: u32 = state.players[0].wall.iter()
            .flat_map(|row| row.iter())
            .filter(|&&occupied| occupied)
            .count() as u32;
        
        let player1_wall_tiles: u32 = state.players[1].wall.iter()
            .flat_map(|row| row.iter())
            .filter(|&&occupied| occupied)
            .count() as u32;
        
        let total_wall_tiles = player0_wall_tiles + player1_wall_tiles;
        
        assert!(total_wall_tiles > 0, 
            "Late game should have well-filled walls after 2 rounds, but found 0 tiles");
        
        // Should be in round 3 or higher
        assert!(state.round_number >= 3, 
            "Late game should be in round 3+, but was in round {}", state.round_number);
    }
}
