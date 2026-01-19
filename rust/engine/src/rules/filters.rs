use crate::model::{State, DraftAction, Destination};
use crate::rules::list_legal_actions;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Error types for quality filtering
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FilterError {
    /// Scenario has too few legal actions (forced move)
    TooFewActions { actual: usize, minimum: usize },
    /// Scenario has too few unique destination options (degenerate)
    DegenerateOptions { unique_destinations: usize, minimum: usize },
}

impl std::fmt::Display for FilterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FilterError::TooFewActions { actual, minimum } => {
                write!(f, "Too few legal actions: {} (minimum: {})", actual, minimum)
            }
            FilterError::DegenerateOptions { unique_destinations, minimum } => {
                write!(
                    f,
                    "Too few unique destinations: {} (minimum: {})",
                    unique_destinations, minimum
                )
            }
        }
    }
}

impl std::error::Error for FilterError {}

/// Configuration for quality filters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterConfig {
    /// Minimum number of legal actions required
    /// Default: 3 (avoid forced/nearly-forced moves)
    #[serde(default = "default_min_legal_actions")]
    pub min_legal_actions: usize,
    
    /// Minimum number of unique destination types
    /// Default: 2 (avoid scenarios where all actions go to floor)
    #[serde(default = "default_min_unique_destinations")]
    pub min_unique_destinations: usize,
}

fn default_min_legal_actions() -> usize {
    3
}

fn default_min_unique_destinations() -> usize {
    2
}

impl Default for FilterConfig {
    fn default() -> Self {
        Self {
            min_legal_actions: default_min_legal_actions(),
            min_unique_destinations: default_min_unique_destinations(),
        }
    }
}

/// Count unique destination types in action list
///
/// Returns the number of distinct destinations (Floor counts as 1, each pattern line as separate).
fn count_unique_destinations(actions: &[DraftAction]) -> usize {
    let mut destinations = HashSet::new();
    
    for action in actions {
        match action.destination {
            Destination::Floor => {
                destinations.insert("floor");
            }
            Destination::PatternLine(row) => {
                // Each pattern line is a unique destination
                destinations.insert(match row {
                    0 => "pl0",
                    1 => "pl1",
                    2 => "pl2",
                    3 => "pl3",
                    4 => "pl4",
                    _ => "pl_other",
                });
            }
        }
    }
    
    destinations.len()
}

/// Apply quality filters to a scenario
///
/// Checks if the scenario meets minimum quality standards for practice.
///
/// # Arguments
///
/// * `state` - Game state to evaluate
/// * `config` - Filter configuration
///
/// # Returns
///
/// * `Ok(())` - Scenario passes all filters
/// * `Err(FilterError)` - Scenario fails a filter (with reason)
pub fn apply_quality_filters(
    state: &State,
    config: &FilterConfig,
) -> Result<(), FilterError> {
    let legal_actions = list_legal_actions(state, state.active_player_id);
    
    // Filter 1: Minimum legal actions (avoid forced moves)
    if legal_actions.len() < config.min_legal_actions {
        return Err(FilterError::TooFewActions {
            actual: legal_actions.len(),
            minimum: config.min_legal_actions,
        });
    }
    
    // Filter 2: Destination diversity (avoid degenerate floor-dump scenarios)
    let unique_dests = count_unique_destinations(&legal_actions);
    if unique_dests < config.min_unique_destinations {
        return Err(FilterError::DegenerateOptions {
            unique_destinations: unique_dests,
            minimum: config.min_unique_destinations,
        });
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{ActionSource, TileColor};

    #[test]
    fn test_default_filter_config() {
        let config = FilterConfig::default();
        assert_eq!(config.min_legal_actions, 3);
        assert_eq!(config.min_unique_destinations, 2);
    }

    #[test]
    fn test_count_unique_destinations_all_floor() {
        let actions = vec![
            DraftAction {
                source: ActionSource::Factory(0),
                color: TileColor::Blue,
                destination: Destination::Floor,
            },
            DraftAction {
                source: ActionSource::Factory(1),
                color: TileColor::Red,
                destination: Destination::Floor,
            },
        ];
        
        assert_eq!(count_unique_destinations(&actions), 1);
    }

    #[test]
    fn test_count_unique_destinations_mixed() {
        let actions = vec![
            DraftAction {
                source: ActionSource::Factory(0),
                color: TileColor::Blue,
                destination: Destination::Floor,
            },
            DraftAction {
                source: ActionSource::Factory(1),
                color: TileColor::Red,
                destination: Destination::PatternLine(0),
            },
            DraftAction {
                source: ActionSource::Factory(2),
                color: TileColor::Yellow,
                destination: Destination::PatternLine(0),
            },
        ];
        
        // Floor + PatternLine(0) = 2 unique destinations
        assert_eq!(count_unique_destinations(&actions), 2);
    }

    #[test]
    fn test_count_unique_destinations_multiple_pattern_lines() {
        let actions = vec![
            DraftAction {
                source: ActionSource::Factory(0),
                color: TileColor::Blue,
                destination: Destination::PatternLine(0),
            },
            DraftAction {
                source: ActionSource::Factory(1),
                color: TileColor::Red,
                destination: Destination::PatternLine(1),
            },
            DraftAction {
                source: ActionSource::Factory(2),
                color: TileColor::Yellow,
                destination: Destination::PatternLine(2),
            },
        ];
        
        // 3 different pattern lines
        assert_eq!(count_unique_destinations(&actions), 3);
    }

    #[test]
    fn test_apply_quality_filters_too_few_actions() {
        let mut state = State::new_test_state();
        let config = FilterConfig {
            min_legal_actions: 100, // Impossible to have 100 legal actions
            min_unique_destinations: 2,
        };
        
        // Add minimal tiles to create few actions
        state.factories[0].insert(TileColor::Blue, 2);
        
        let result = apply_quality_filters(&state, &config);
        
        match result {
            Err(FilterError::TooFewActions { actual, minimum }) => {
                assert!(actual < 100);
                assert_eq!(minimum, 100);
            }
            _ => panic!("Expected TooFewActions error, got: {:?}", result),
        }
    }

    #[test]
    fn test_apply_quality_filters_passes() {
        let mut state = State::new_test_state();
        let config = FilterConfig::default();
        
        // Create scenario with multiple good options
        state.factories[0].insert(TileColor::Blue, 2);
        state.factories[1].insert(TileColor::Red, 2);
        state.factories[2].insert(TileColor::Yellow, 2);
        
        let result = apply_quality_filters(&state, &config);
        
        // Should pass (multiple factories × multiple destinations = many actions)
        assert!(result.is_ok(), "Quality filters should pass with multiple options");
    }

    #[test]
    fn test_filter_error_display() {
        let err = FilterError::TooFewActions { actual: 2, minimum: 3 };
        let msg = format!("{}", err);
        assert!(msg.contains("2"));
        assert!(msg.contains("3"));
        
        let err = FilterError::DegenerateOptions { unique_destinations: 1, minimum: 2 };
        let msg = format!("{}", err);
        assert!(msg.contains("1"));
        assert!(msg.contains("2"));
    }
}
