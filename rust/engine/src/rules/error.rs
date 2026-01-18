use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::{ActionSource, TileColor};

/// Validation error returned when an action cannot be applied
///
/// Contains a machine-readable error code, a human-readable message,
/// and optional context data for debugging.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ValidationError {
    pub code: String,
    pub message: String,
    pub context: Option<serde_json::Value>,
}

impl ValidationError {
    /// Player ID is out of valid range (0-1)
    pub fn invalid_player(player_id: u8) -> Self {
        Self {
            code: "INVALID_PLAYER".to_string(),
            message: format!("Player ID {} is out of range", player_id),
            context: Some(json!({"player_id": player_id})),
        }
    }
    
    /// Factory index is out of bounds
    pub fn invalid_source(factory_idx: usize) -> Self {
        Self {
            code: "INVALID_SOURCE".to_string(),
            message: format!("Factory index {} is out of bounds", factory_idx),
            context: Some(json!({"factory_index": factory_idx})),
        }
    }
    
    /// Source does not contain the requested color
    pub fn source_empty(source: ActionSource, color: TileColor) -> Self {
        Self {
            code: "SOURCE_EMPTY".to_string(),
            message: format!("Source {:?} does not contain {:?} tiles", source, color),
            context: Some(json!({"source": source, "color": color})),
        }
    }
    
    /// Attempted to place a different color than what's already in the pattern line
    pub fn color_mismatch(row: usize, existing: TileColor, attempted: TileColor) -> Self {
        Self {
            code: "COLOR_MISMATCH".to_string(),
            message: format!(
                "Cannot place {:?} tiles into pattern line {} which contains {:?}",
                attempted, row, existing
            ),
            context: Some(json!({
                "row": row,
                "existing_color": existing,
                "attempted_color": attempted
            })),
        }
    }
    
    /// Attempted to place a color that already exists in the wall for that row
    pub fn wall_conflict(row: usize, color: TileColor) -> Self {
        Self {
            code: "WALL_CONFLICT".to_string(),
            message: format!(
                "Color {:?} already exists in wall row {}",
                color, row
            ),
            context: Some(json!({"row": row, "color": color})),
        }
    }
    
    /// Pattern line is already complete and cannot accept more tiles
    pub fn pattern_line_complete(row: usize) -> Self {
        Self {
            code: "PATTERN_LINE_COMPLETE".to_string(),
            message: format!("Pattern line {} is already complete", row),
            context: Some(json!({"row": row})),
        }
    }
    
    /// Pattern line row is out of valid range (0-4)
    pub fn invalid_destination(row: usize) -> Self {
        Self {
            code: "INVALID_DESTINATION".to_string(),
            message: format!("Pattern line row {} is out of bounds", row),
            context: Some(json!({"row": row})),
        }
    }
    
    /// Internal invariant was violated (programming error)
    pub fn invariant_violation(message: String) -> Self {
        Self {
            code: "INVARIANT_VIOLATION".to_string(),
            message,
            context: None,
        }
    }
}
