use crate::model::{State, DraftAction, Destination};
use rand::Rng;
use rand::seq::SliceRandom;

/// Trait for selecting draft actions during scenario generation
///
/// Policy bots are used to play forward from initial states to create
/// plausible mid-game scenarios.
pub trait DraftPolicy {
    /// Select an action from the list of legal actions
    ///
    /// # Arguments
    ///
    /// * `state` - Current game state
    /// * `legal_actions` - List of legal actions to choose from
    /// * `rng` - Random number generator for tie-breaking
    ///
    /// # Returns
    ///
    /// * `Some(action)` - Selected action
    /// * `None` - No action could be selected (shouldn't happen with legal actions)
    fn select_action<R: Rng>(
        &self,
        state: &State,
        legal_actions: &[DraftAction],
        rng: &mut R,
    ) -> Option<DraftAction>;
}

/// Random policy that selects actions uniformly at random
///
/// This is the simplest policy and creates highly varied scenarios.
pub struct RandomPolicy;

impl DraftPolicy for RandomPolicy {
    fn select_action<R: Rng>(
        &self,
        _state: &State,
        legal_actions: &[DraftAction],
        rng: &mut R,
    ) -> Option<DraftAction> {
        legal_actions.choose(rng).cloned()
    }
}

/// Greedy policy that uses simple heuristics to make reasonable moves
///
/// Heuristics (in priority order):
/// 1. Prefer pattern line placements over floor
/// 2. For pattern lines: prefer rows with more empty spaces
/// 3. Prefer taking more tiles (maximize acquisition)
/// 4. Break ties randomly
///
/// This creates more realistic game states than pure random selection.
pub struct GreedyPolicy;

impl GreedyPolicy {
    /// Score an action based on greedy heuristics (higher is better)
    fn score_action(state: &State, action: &DraftAction) -> i32 {
        let mut score = 0;
        
        // Count tiles being taken
        let tile_count = count_tiles_in_source(state, action);
        score += tile_count as i32 * 10; // High weight on acquiring tiles
        
        // Prefer pattern line placements
        match action.destination {
            Destination::PatternLine(row) => {
                score += 100; // Strong preference for pattern lines
                
                // Prefer rows with more empty spaces (easier to complete later)
                let pattern_line = &state.players[state.active_player_id as usize].pattern_lines[row];
                let empty_spaces = pattern_line.capacity as i32 - pattern_line.count_filled as i32;
                score += empty_spaces * 5;
                
                // Slight preference for filling partially-filled lines
                if pattern_line.count_filled > 0 && pattern_line.color == Some(action.color) {
                    score += 15;
                }
            }
            Destination::Floor => {
                // Floor is least preferred (score = tile_count * 10 only)
            }
        }
        
        score
    }
}

impl DraftPolicy for GreedyPolicy {
    fn select_action<R: Rng>(
        &self,
        state: &State,
        legal_actions: &[DraftAction],
        rng: &mut R,
    ) -> Option<DraftAction> {
        if legal_actions.is_empty() {
            return None;
        }
        
        // Score all actions
        let scored_actions: Vec<(i32, &DraftAction)> = legal_actions
            .iter()
            .map(|action| (Self::score_action(state, action), action))
            .collect();
        
        // Find maximum score
        let max_score = scored_actions.iter().map(|(score, _)| *score).max().unwrap();
        
        // Collect all actions with max score (for tie-breaking)
        let best_actions: Vec<&DraftAction> = scored_actions
            .iter()
            .filter(|(score, _)| *score == max_score)
            .map(|(_, action)| *action)
            .collect();
        
        // Break ties randomly
        best_actions.choose(rng).map(|&action| action.clone())
    }
}

/// Count how many tiles are being taken in this action
fn count_tiles_in_source(state: &State, action: &DraftAction) -> u8 {
    match &action.source {
        crate::model::ActionSource::Factory(idx) => {
            state.factories[*idx].get(&action.color).copied().unwrap_or(0)
        }
        crate::model::ActionSource::Center => {
            state.center.tiles.get(&action.color).copied().unwrap_or(0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{ActionSource, TileColor};
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    #[test]
    fn test_random_policy_selects_from_legal_actions() {
        let state = State::new_test_state();
        let mut rng = StdRng::seed_from_u64(12345);
        
        let actions = vec![
            DraftAction {
                source: ActionSource::Factory(0),
                color: TileColor::Blue,
                destination: Destination::PatternLine(0),
            },
            DraftAction {
                source: ActionSource::Factory(1),
                color: TileColor::Red,
                destination: Destination::Floor,
            },
        ];
        
        let policy = RandomPolicy;
        let selected = policy.select_action(&state, &actions, &mut rng);
        
        assert!(selected.is_some());
        assert!(actions.contains(&selected.unwrap()));
    }

    #[test]
    fn test_random_policy_returns_none_for_empty_list() {
        let state = State::new_test_state();
        let mut rng = StdRng::seed_from_u64(12345);
        
        let policy = RandomPolicy;
        let selected = policy.select_action(&state, &[], &mut rng);
        
        assert!(selected.is_none());
    }

    #[test]
    fn test_greedy_policy_prefers_pattern_lines() {
        let mut state = State::new_test_state();
        let mut rng = StdRng::seed_from_u64(12345);
        
        // Add some tiles to factory
        state.factories[0].insert(TileColor::Blue, 3);
        
        let actions = vec![
            DraftAction {
                source: ActionSource::Factory(0),
                color: TileColor::Blue,
                destination: Destination::PatternLine(2), // Pattern line
            },
            DraftAction {
                source: ActionSource::Factory(0),
                color: TileColor::Blue,
                destination: Destination::Floor, // Floor
            },
        ];
        
        let policy = GreedyPolicy;
        let selected = policy.select_action(&state, &actions, &mut rng).unwrap();
        
        // Should prefer pattern line over floor
        match selected.destination {
            Destination::PatternLine(_) => {}, // Good!
            Destination::Floor => panic!("Greedy policy should prefer pattern line over floor"),
        }
    }

    #[test]
    fn test_greedy_policy_prefers_more_tiles() {
        let mut state = State::new_test_state();
        let mut rng = StdRng::seed_from_u64(12345);
        
        // Factory 0 has 3 blue tiles
        state.factories[0].insert(TileColor::Blue, 3);
        // Factory 1 has 1 blue tile
        state.factories[1].insert(TileColor::Blue, 1);
        
        let actions = vec![
            DraftAction {
                source: ActionSource::Factory(0),
                color: TileColor::Blue,
                destination: Destination::PatternLine(2),
            },
            DraftAction {
                source: ActionSource::Factory(1),
                color: TileColor::Blue,
                destination: Destination::PatternLine(2),
            },
        ];
        
        let policy = GreedyPolicy;
        let selected = policy.select_action(&state, &actions, &mut rng).unwrap();
        
        // Should prefer taking 3 tiles over 1 tile
        match selected.source {
            ActionSource::Factory(0) => {}, // Good!
            ActionSource::Factory(1) => panic!("Greedy policy should prefer more tiles"),
            _ => panic!("Unexpected source"),
        }
    }

    #[test]
    fn test_greedy_policy_returns_none_for_empty_list() {
        let state = State::new_test_state();
        let mut rng = StdRng::seed_from_u64(12345);
        
        let policy = GreedyPolicy;
        let selected = policy.select_action(&state, &[], &mut rng);
        
        assert!(selected.is_none());
    }

    #[test]
    fn test_greedy_policy_tie_breaking_is_random() {
        let mut state = State::new_test_state();
        
        // Both factories have same tiles
        state.factories[0].insert(TileColor::Blue, 2);
        state.factories[1].insert(TileColor::Blue, 2);
        
        let actions = vec![
            DraftAction {
                source: ActionSource::Factory(0),
                color: TileColor::Blue,
                destination: Destination::PatternLine(2),
            },
            DraftAction {
                source: ActionSource::Factory(1),
                color: TileColor::Blue,
                destination: Destination::PatternLine(2),
            },
        ];
        
        let policy = GreedyPolicy;
        
        // Run multiple times with different seeds to verify randomness
        let mut selected_factory_0 = 0;
        let mut selected_factory_1 = 0;
        
        for seed in 0..100 {
            let mut rng = StdRng::seed_from_u64(seed);
            let selected = policy.select_action(&state, &actions, &mut rng).unwrap();
            
            match selected.source {
                ActionSource::Factory(0) => selected_factory_0 += 1,
                ActionSource::Factory(1) => selected_factory_1 += 1,
                _ => {}
            }
        }
        
        // Both should be selected at least once (very high probability)
        assert!(selected_factory_0 > 0, "Factory 0 should be selected sometimes");
        assert!(selected_factory_1 > 0, "Factory 1 should be selected sometimes");
    }
}
