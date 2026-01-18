use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::{TileColor, DraftPhase, PlayerBoard};

/// Multiset of tiles represented as HashMap
///
/// Maps tile colors to counts. Only colors with non-zero counts are stored.
/// This provides a sparse representation that is memory-efficient and easy to work with.
pub type TileMultiset = HashMap<TileColor, u8>;

/// Center area with tiles and first-player token
///
/// The center accumulates tiles from factories as players take tiles.
/// It also holds the first-player token at the start of each round.
///
/// When a player takes tiles from the center with the token present,
/// they receive the token (which goes to their floor line).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CenterArea {
    pub tiles: TileMultiset,
    pub has_first_player_token: bool,
}

/// Complete game state for 2-player Azul
///
/// This struct represents the entire state of an Azul game during the draft phase,
/// including the supply (bag/lid), table state (factories/center), and both player boards.
///
/// # Serialization
///
/// The state serializes to JSON with snake_case field names. The `scenario_seed` field
/// is omitted from JSON when None.
///
/// # Invariants
///
/// - Total tiles in the game must equal 100 (tile conservation)
/// - `active_player_id` must be 0 or 1
/// - Pattern lines must satisfy their invariants (see `PatternLine`)
///
/// # Example
///
/// ```
/// use engine::State;
/// let state = State::new_test_state();
/// let json = serde_json::to_string_pretty(&state).unwrap();
/// let restored: State = serde_json::from_str(&json).unwrap();
/// assert_eq!(state, restored);
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct State {
    // Metadata
    pub state_version: u32,
    pub ruleset_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario_seed: Option<String>,
    pub active_player_id: u8,
    pub round_number: u8,
    pub draft_phase_progress: DraftPhase,
    
    // Supply
    pub bag: TileMultiset,
    pub lid: TileMultiset,
    
    // Table
    pub factories: Vec<TileMultiset>,
    pub center: CenterArea,
    
    // Players
    pub players: [PlayerBoard; 2],
}

impl State {
    /// Create a new state with default values for testing
    ///
    /// Creates a minimal valid state with:
    /// - Version 1, ruleset "azul_v1_2p"
    /// - Round 1, active player 0
    /// - Empty supply (bag/lid)
    /// - 5 empty factories
    /// - Empty center with first-player token
    /// - Two empty player boards
    ///
    /// This is useful for unit tests and can be modified to create specific scenarios.
    pub fn new_test_state() -> Self {
        Self {
            state_version: 1,
            ruleset_id: "azul_v1_2p".to_string(),
            scenario_seed: None,
            active_player_id: 0,
            round_number: 1,
            draft_phase_progress: DraftPhase::Early,
            bag: HashMap::new(),
            lid: HashMap::new(),
            factories: vec![HashMap::new(); 5],
            center: CenterArea {
                tiles: HashMap::new(),
                has_first_player_token: true,
            },
            players: [PlayerBoard::new(), PlayerBoard::new()],
        }
    }
}
