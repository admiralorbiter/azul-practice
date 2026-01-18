use wasm_bindgen::prelude::*;
use serde_json::json;
use crate::{State, DraftAction};
use crate::rules::{
    list_legal_actions as list_legal_actions_internal,
    apply_action as apply_action_internal
};

/// Helper function to serialize errors consistently
fn serialize_error(code: &str, message: &str, context: Option<serde_json::Value>) -> String {
    let error = json!({
        "error": {
            "code": code,
            "message": message,
            "context": context,
        }
    });
    serde_json::to_string(&error).unwrap()
}

/// List all legal draft actions for the given player
///
/// # Arguments
/// * `state_json` - JSON string representing game state
/// * `player_id` - Player ID (0 or 1)
///
/// # Returns
/// JSON string: either action array or error object
#[wasm_bindgen]
pub fn list_legal_actions(state_json: &str, player_id: u8) -> String {
    // Parse state JSON
    let state: State = match serde_json::from_str(state_json) {
        Ok(s) => s,
        Err(e) => {
            return serialize_error(
                "INVALID_JSON",
                &format!("Failed to parse state JSON: {}", e),
                Some(json!({"parse_error": e.to_string()}))
            );
        }
    };
    
    // Validate player_id
    if player_id > 1 {
        return serialize_error(
            "INVALID_PLAYER",
            &format!("Player ID {} is out of range (must be 0 or 1)", player_id),
            Some(json!({"player_id": player_id}))
        );
    }
    
    // Call engine function
    let actions = list_legal_actions_internal(&state, player_id);
    
    // Serialize result
    match serde_json::to_string(&actions) {
        Ok(json) => json,
        Err(e) => {
            serialize_error(
                "SERIALIZATION_ERROR",
                &format!("Failed to serialize actions: {}", e),
                None
            )
        }
    }
}

/// Apply a draft action to the game state
///
/// # Arguments
/// * `state_json` - JSON string representing game state
/// * `action_json` - JSON string representing draft action
///
/// # Returns
/// JSON string: either new state or error object
#[wasm_bindgen]
pub fn apply_action(state_json: &str, action_json: &str) -> String {
    // Parse state JSON
    let state: State = match serde_json::from_str(state_json) {
        Ok(s) => s,
        Err(e) => {
            return serialize_error(
                "INVALID_STATE_JSON",
                &format!("Failed to parse state JSON: {}", e),
                Some(json!({"parse_error": e.to_string()}))
            );
        }
    };
    
    // Parse action JSON
    let action: DraftAction = match serde_json::from_str(action_json) {
        Ok(a) => a,
        Err(e) => {
            return serialize_error(
                "INVALID_ACTION_JSON",
                &format!("Failed to parse action JSON: {}", e),
                Some(json!({"parse_error": e.to_string()}))
            );
        }
    };
    
    // Call engine function
    match apply_action_internal(&state, &action) {
        Ok(new_state) => {
            // Success: return new state as JSON
            match serde_json::to_string(&new_state) {
                Ok(json) => json,
                Err(e) => {
                    serialize_error(
                        "SERIALIZATION_ERROR",
                        &format!("Failed to serialize state: {}", e),
                        None
                    )
                }
            }
        },
        Err(validation_error) => {
            // Engine validation error - pass through with error wrapper
            let error = json!({
                "error": {
                    "code": validation_error.code,
                    "message": validation_error.message,
                    "context": validation_error.context,
                }
            });
            serde_json::to_string(&error).unwrap()
        }
    }
}
