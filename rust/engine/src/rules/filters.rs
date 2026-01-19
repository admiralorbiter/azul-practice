use crate::model::{State, DraftAction, Destination};
use crate::rules::list_legal_actions;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Error types for quality filtering
#[derive(Debug, Clone, PartialEq)]
pub enum FilterError {
    /// Scenario has too few legal actions (forced move)
    TooFewActions { actual: usize, minimum: usize },
    /// Scenario has too few unique destination options (degenerate)
    DegenerateOptions { unique_destinations: usize, minimum: usize },
    /// No non-floor options available
    NoNonFloorOption,
    /// Too many floor-only actions
    TooManyFloorActions { ratio: f32, max_allowed: f32 },
    /// Value gap too small
    ValueGapTooSmall { actual: f32, minimum: f32 },
    /// Value gap too large
    ValueGapTooLarge { actual: f32, maximum: f32 },
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
            FilterError::NoNonFloorOption => {
                write!(f, "No non-floor options available")
            }
            FilterError::TooManyFloorActions { ratio, max_allowed } => {
                write!(
                    f,
                    "Too many floor actions: {:.1}% (max: {:.1}%)",
                    ratio * 100.0, max_allowed * 100.0
                )
            }
            FilterError::ValueGapTooSmall { actual, minimum } => {
                write!(f, "Value gap too small: {:.1} (minimum: {:.1})", actual, minimum)
            }
            FilterError::ValueGapTooLarge { actual, maximum } => {
                write!(f, "Value gap too large: {:.1} (maximum: {:.1})", actual, maximum)
            }
        }
    }
}

impl std::error::Error for FilterError {}

/// Configuration for quality filters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterConfig {
    /// Minimum number of legal actions required
    /// Default: 6 (raised from 3 for better puzzles)
    #[serde(default = "default_min_legal_actions")]
    pub min_legal_actions: usize,
    
    /// Minimum number of unique destination types
    /// Default: 2 (avoid scenarios where all actions go to floor)
    #[serde(default = "default_min_unique_destinations")]
    pub min_unique_destinations: usize,
    
    /// Require at least one non-floor destination option
    /// Default: true (avoid pure dump scenarios)
    #[serde(default = "default_require_non_floor_option")]
    pub require_non_floor_option: bool,
    
    /// Maximum ratio of actions that only go to floor
    /// Default: 0.5 (at most half can be floor-only)
    #[serde(default = "default_max_floor_ratio")]
    pub max_floor_ratio: f32,
    
    /// Minimum EV gap between best and 2nd best move (points)
    /// None means no minimum gap required
    /// Default: None (balanced mix allows close decisions)
    #[serde(default)]
    pub min_value_gap: Option<f32>,
    
    /// Maximum EV gap between best and 2nd best move (points)
    /// None means no maximum gap
    /// Default: None (balanced mix allows clear best moves)
    #[serde(default)]
    pub max_value_gap: Option<f32>,
}

fn default_min_legal_actions() -> usize {
    6  // Raised from 3 for better puzzles
}

fn default_min_unique_destinations() -> usize {
    2
}

fn default_require_non_floor_option() -> bool {
    true
}

fn default_max_floor_ratio() -> f32 {
    0.5
}

impl Default for FilterConfig {
    fn default() -> Self {
        Self {
            min_legal_actions: default_min_legal_actions(),
            min_unique_destinations: default_min_unique_destinations(),
            require_non_floor_option: default_require_non_floor_option(),
            max_floor_ratio: default_max_floor_ratio(),
            min_value_gap: None,
            max_value_gap: None,
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

/// Check if an action goes to floor
fn is_floor_action(action: &DraftAction) -> bool {
    matches!(action.destination, Destination::Floor)
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
    
    // Filter 3: Require non-floor option
    if config.require_non_floor_option {
        let has_non_floor = legal_actions.iter().any(|a| !is_floor_action(a));
        if !has_non_floor {
            return Err(FilterError::NoNonFloorOption);
        }
    }
    
    // Filter 4: Floor action ratio
    let floor_count = legal_actions.iter().filter(|a| is_floor_action(a)).count();
    let floor_ratio = floor_count as f32 / legal_actions.len() as f32;
    if floor_ratio > config.max_floor_ratio {
        return Err(FilterError::TooManyFloorActions {
            ratio: floor_ratio,
            max_allowed: config.max_floor_ratio,
        });
    }
    
    // Note: EV gap filtering is handled separately in generate_scenario_with_filters
    // because it requires rollout evaluation which is expensive
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{ActionSource, TileColor};

    #[test]
    fn test_default_filter_config() {
        let config = FilterConfig::default();
        assert_eq!(config.min_legal_actions, 6);  // Raised from 3
        assert_eq!(config.min_unique_destinations, 2);
        assert_eq!(config.require_non_floor_option, true);
        assert_eq!(config.max_floor_ratio, 0.5);
        assert_eq!(config.min_value_gap, None);
        assert_eq!(config.max_value_gap, None);
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
            require_non_floor_option: true,
            max_floor_ratio: 0.5,
            min_value_gap: None,
            max_value_gap: None,
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
