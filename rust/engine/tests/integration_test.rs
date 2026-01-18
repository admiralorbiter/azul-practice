use serde_json::Value;

// Note: We can't use #[wasm_bindgen] functions directly in tests,
// but we can test the underlying logic by calling the functions directly
// with the pub modifier. In a real integration test, you'd test via
// the actual WASM interface using a browser test runner.

#[test]
fn test_list_legal_actions_returns_json_string() {
    let state_json = include_str!("fixtures/mid_game_state.json");
    let result = engine::wasm_api::list_legal_actions(state_json, 0);
    
    // Should be valid JSON
    let parsed: Value = serde_json::from_str(&result)
        .expect("Result should be valid JSON");
    
    // Check if it's an array or error
    assert!(parsed.is_array() || parsed.get("error").is_some());
}

#[test]
fn test_list_legal_actions_valid_state() {
    let state_json = include_str!("fixtures/mid_game_state.json");
    let result = engine::wasm_api::list_legal_actions(state_json, 0);
    
    let parsed: Value = serde_json::from_str(&result).unwrap();
    
    // Should be an array (not error)
    assert!(parsed.is_array(), "Expected action array, got: {}", result);
    
    let actions = parsed.as_array().unwrap();
    assert!(actions.len() > 0, "Should have at least one action");
}

#[test]
fn test_list_legal_actions_invalid_json() {
    let invalid_json = "{ not valid json";
    let result = engine::wasm_api::list_legal_actions(invalid_json, 0);
    
    let error: Value = serde_json::from_str(&result).unwrap();
    assert!(error.get("error").is_some());
    assert_eq!(error["error"]["code"], "INVALID_JSON");
}

#[test]
fn test_list_legal_actions_invalid_player() {
    let state_json = include_str!("fixtures/mid_game_state.json");
    let result = engine::wasm_api::list_legal_actions(state_json, 5);
    
    let error: Value = serde_json::from_str(&result).unwrap();
    assert!(error.get("error").is_some());
    assert_eq!(error["error"]["code"], "INVALID_PLAYER");
}

#[test]
fn test_apply_action_returns_json_string() {
    let state_json = include_str!("fixtures/mid_game_state.json");
    let action_json = r#"{"source":{"Factory":0},"color":"Blue","destination":"Floor"}"#;
    
    let result = engine::wasm_api::apply_action(state_json, action_json);
    
    // Should be valid JSON
    let parsed: Value = serde_json::from_str(&result)
        .expect("Result should be valid JSON");
    
    // Check if it's a state or error
    assert!(parsed.get("state_version").is_some() || parsed.get("error").is_some());
}

#[test]
fn test_apply_action_success() {
    let state_json = include_str!("fixtures/mid_game_state.json");
    // Take Blue from Factory 0 to Floor (should always be legal)
    let action_json = r#"{"source":{"Factory":0},"color":"Blue","destination":"Floor"}"#;
    
    let result = engine::wasm_api::apply_action(state_json, action_json);
    
    let parsed: Value = serde_json::from_str(&result).unwrap();
    
    // Should be a state (not error)
    assert!(parsed.get("state_version").is_some(), "Expected state, got: {}", result);
    
    // Active player should have changed
    let original_state: Value = serde_json::from_str(state_json).unwrap();
    assert_ne!(
        parsed["active_player_id"],
        original_state["active_player_id"],
        "Active player should toggle"
    );
}

#[test]
fn test_apply_action_invalid_state_json() {
    let invalid_json = "{ not valid }";
    let action_json = r#"{"source":"Center","color":"Blue","destination":"Floor"}"#;
    
    let result = engine::wasm_api::apply_action(invalid_json, action_json);
    
    let error: Value = serde_json::from_str(&result).unwrap();
    assert!(error.get("error").is_some());
    assert_eq!(error["error"]["code"], "INVALID_STATE_JSON");
}

#[test]
fn test_apply_action_invalid_action_json() {
    let state_json = include_str!("fixtures/mid_game_state.json");
    let invalid_action = "{ not valid }";
    
    let result = engine::wasm_api::apply_action(state_json, invalid_action);
    
    let error: Value = serde_json::from_str(&result).unwrap();
    assert!(error.get("error").is_some());
    assert_eq!(error["error"]["code"], "INVALID_ACTION_JSON");
}

#[test]
fn test_apply_action_illegal_move() {
    let state_json = include_str!("fixtures/mid_game_state.json");
    // Try to take from empty factory
    let action_json = r#"{"source":{"Factory":2},"color":"Red","destination":"Floor"}"#;
    
    let result = engine::wasm_api::apply_action(state_json, action_json);
    
    let error: Value = serde_json::from_str(&result).unwrap();
    assert!(error.get("error").is_some());
    // Should be an engine validation error (SOURCE_EMPTY)
    assert_eq!(error["error"]["code"], "SOURCE_EMPTY");
}
