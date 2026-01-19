use wasm_bindgen::prelude::*;
use serde_json::json;
use crate::{State, DraftAction};
use crate::rules::{
    list_legal_actions as list_legal_actions_internal,
    apply_action as apply_action_internal,
    resolve_end_of_round as resolve_end_of_round_internal,
    GeneratorParamsJson,
    generate_scenario_with_filters,
    evaluate_best_move as evaluate_best_move_internal,
    grade_user_action as grade_user_action_internal,
    EvaluatorParams,
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

/// Resolve end of round: score tiles, apply penalties, refill factories
///
/// # Arguments
/// * `state_json` - JSON string representing game state
///
/// # Returns
/// JSON string: either new state or error object
#[wasm_bindgen]
pub fn resolve_end_of_round(state_json: &str) -> String {
    // Parse state
    let state: State = match serde_json::from_str(state_json) {
        Ok(s) => s,
        Err(e) => {
            return serialize_error(
                "INVALID_STATE_JSON",
                &format!("Failed to parse state: {}", e),
                None
            );
        }
    };
    
    // Resolve end of round
    match resolve_end_of_round_internal(&state) {
        Ok(new_state) => {
            match serde_json::to_string(&new_state) {
                Ok(json) => json,
                Err(e) => serialize_error(
                    "SERIALIZATION_ERROR",
                    &format!("Failed to serialize state: {}", e),
                    None
                )
            }
        }
        Err(e) => {
            let error = json!({
                "error": {
                    "code": e.code,
                    "message": e.message,
                    "context": e.context,
                }
            });
            serde_json::to_string(&error).unwrap()
        }
    }
}

/// Generate a practice scenario using play-forward method
///
/// Creates a plausible game state by:
/// 1. Starting from legal round start
/// 2. Playing forward N moves with policy bots
/// 3. Applying quality filters
/// 4. Tagging phase based on progress
///
/// # Arguments
/// * `params_json` - JSON string with optional parameters:
///   - targetPhase: "EARLY" | "MID" | "LATE" (default: random)
///   - seed: string seed for reproducibility (default: random)
///   - policyMix: "random" | "greedy" | "mixed" (default: "mixed")
///   - filterConfig: { minLegalActions, minUniqueDestinations }
///
/// # Returns
/// JSON string: either new game state or error object
///
/// # Example
/// ```javascript
/// const params = {
///   targetPhase: "MID",
///   seed: "12345",
///   policyMix: "mixed"
/// };
/// const result = generate_scenario(JSON.stringify(params));
/// ```
#[wasm_bindgen]
pub fn generate_scenario(params_json: &str) -> String {
    // Parse params (empty object is valid - all fields optional)
    let params: GeneratorParamsJson = match serde_json::from_str(params_json) {
        Ok(p) => p,
        Err(e) => {
            return serialize_error(
                "INVALID_PARAMS_JSON",
                &format!("Failed to parse params: {}", e),
                Some(json!({"parse_error": e.to_string()}))
            );
        }
    };
    
    // Convert to internal params
    let (generator_params, filter_config) = match params.to_internal() {
        Ok(p) => p,
        Err(e) => {
            return serialize_error(
                "INVALID_PARAMS",
                &e,
                None
            );
        }
    };
    
    // Generate with filters and retry logic (max 500 attempts).
    // Now strictly enforces stage matching, so may need more attempts to find valid seed.
    match generate_scenario_with_filters(generator_params, filter_config, 500) {
        Ok(state) => {
            match serde_json::to_string(&state) {
                Ok(json) => json,
                Err(e) => serialize_error(
                    "SERIALIZATION_ERROR",
                    &format!("Failed to serialize state: {}", e),
                    None
                )
            }
        }
        Err(e) => {
            serialize_error(
                "GENERATION_FAILED",
                &format!("Scenario generation failed after 500 attempts: {}", e),
                Some(json!({"max_attempts": 500, "error": format!("{:?}", e)}))
            )
        }
    }
}

/// Evaluate best move using rollout-based Monte Carlo evaluation
///
/// # Arguments
/// * `state_json` - JSON string representing game state
/// * `player_id` - Player ID (0 or 1)
/// * `params_json` - JSON string with EvaluatorParams
///
/// # Returns
/// JSON string: either EvaluationResult or error object
#[wasm_bindgen]
pub fn evaluate_best_move(
    state_json: &str,
    player_id: u8,
    params_json: &str,
) -> String {
    let state: State = match serde_json::from_str(state_json) {
        Ok(s) => s,
        Err(e) => return serialize_error(
            "INVALID_STATE_JSON",
            &format!("Failed to parse state JSON: {}", e),
            Some(json!({"parse_error": e.to_string()}))
        ),
    };
    
    let params: EvaluatorParams = match serde_json::from_str(params_json) {
        Ok(p) => p,
        Err(e) => return serialize_error(
            "INVALID_PARAMS_JSON",
            &format!("Failed to parse params JSON: {}", e),
            Some(json!({"parse_error": e.to_string()}))
        ),
    };
    
    match evaluate_best_move_internal(&state, player_id, &params) {
        Ok(result) => match serde_json::to_string(&result) {
            Ok(json) => json,
            Err(e) => serialize_error(
                "SERIALIZATION_ERROR",
                &format!("Failed to serialize result: {}", e),
                None
            ),
        },
        Err(e) => serialize_error(
            "EVALUATION_FAILED",
            &e.to_string(),
            None
        ),
    }
}

/// Grade user's action compared to best move
///
/// # Arguments
/// * `state_json` - JSON string representing game state
/// * `player_id` - Player ID (0 or 1)
/// * `user_action_json` - JSON string with user's DraftAction
/// * `params_json` - JSON string with EvaluatorParams
///
/// # Returns
/// JSON string: either EvaluationResult with user action grading or error object
#[wasm_bindgen]
pub fn grade_user_action(
    state_json: &str,
    player_id: u8,
    user_action_json: &str,
    params_json: &str,
) -> String {
    let state: State = match serde_json::from_str(state_json) {
        Ok(s) => s,
        Err(e) => return serialize_error(
            "INVALID_STATE_JSON",
            &format!("Failed to parse state JSON: {}", e),
            Some(json!({"parse_error": e.to_string()}))
        ),
    };
    
    let user_action: DraftAction = match serde_json::from_str(user_action_json) {
        Ok(a) => a,
        Err(e) => return serialize_error(
            "INVALID_ACTION_JSON",
            &format!("Failed to parse action JSON: {}", e),
            Some(json!({"parse_error": e.to_string()}))
        ),
    };
    
    let params: EvaluatorParams = match serde_json::from_str(params_json) {
        Ok(p) => p,
        Err(e) => return serialize_error(
            "INVALID_PARAMS_JSON",
            &format!("Failed to parse params JSON: {}", e),
            Some(json!({"parse_error": e.to_string()}))
        ),
    };
    
    // First evaluate best move
    let best_result = match evaluate_best_move_internal(&state, player_id, &params) {
        Ok(r) => r,
        Err(e) => return serialize_error(
            "EVALUATION_FAILED",
            &e.to_string(),
            None
        ),
    };
    
    // Then grade user action
    match grade_user_action_internal(&state, player_id, &user_action, &params, &best_result) {
        Ok(result) => match serde_json::to_string(&result) {
            Ok(json) => json,
            Err(e) => serialize_error(
                "SERIALIZATION_ERROR",
                &format!("Failed to serialize result: {}", e),
                None
            ),
        },
        Err(e) => serialize_error(
            "GRADING_FAILED",
            &e.to_string(),
            None
        ),
    }
}
