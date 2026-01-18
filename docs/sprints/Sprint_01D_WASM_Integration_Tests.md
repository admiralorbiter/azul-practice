# Sprint 01D — WASM Integration & Tests

**Status:** Draft  
**Prerequisites:** Sprint 01A, 01B, 01C complete  
**Dependencies:** Full engine implementation (models, rules, apply)  
**Estimated Complexity:** Medium

---

## Goal

Expose engine functions via WASM boundary with proper error handling, create TypeScript wrappers, build demo UI, and add integration tests.

## Outcomes

- ✓ WASM exports for `list_legal_actions` and `apply_action`
- ✓ TypeScript wrapper with proper types and error handling
- ✓ Error handling across WASM boundary
- ✓ Integration tests passing (Rust + manual browser tests)
- ✓ Demo UI showing functionality (load scenario, list actions, apply action)

---

## WASM API Design

### Function Signatures

```rust
#[wasm_bindgen]
pub fn list_legal_actions(state_json: &str, player_id: u8) -> String

#[wasm_bindgen]
pub fn apply_action(state_json: &str, action_json: &str) -> String
```

**Both functions:**
- Accept JSON strings as input
- Return JSON strings as output
- Success: JSON representation of result
- Error: JSON with `{"error": {...}}` structure

**Rationale:**
- Simple string-based API works across WASM boundary
- JSON is human-readable for debugging
- Can optimize to binary later if needed

---

## Implementation Details

### list_legal_actions WASM Export

```rust
use wasm_bindgen::prelude::*;
use serde_json::json;
use crate::model::{State, DraftAction};
use crate::rules::list_legal_actions as list_legal_actions_internal;

#[wasm_bindgen]
pub fn list_legal_actions(state_json: &str, player_id: u8) -> String {
    // Parse state JSON
    let state: State = match serde_json::from_str(state_json) {
        Ok(s) => s,
        Err(e) => {
            let error = json!({
                "error": {
                    "code": "INVALID_JSON",
                    "message": format!("Failed to parse state JSON: {}", e),
                    "context": {
                        "parse_error": e.to_string()
                    }
                }
            });
            return serde_json::to_string(&error).unwrap();
        }
    };
    
    // Validate player_id
    if player_id > 1 {
        let error = json!({
            "error": {
                "code": "INVALID_PLAYER",
                "message": format!("Player ID {} is out of range (must be 0 or 1)", player_id),
                "context": {
                    "player_id": player_id
                }
            }
        });
        return serde_json::to_string(&error).unwrap();
    }
    
    // Call engine function
    let actions = list_legal_actions_internal(&state, player_id);
    
    // Serialize result
    match serde_json::to_string(&actions) {
        Ok(json) => json,
        Err(e) => {
            let error = json!({
                "error": {
                    "code": "SERIALIZATION_ERROR",
                    "message": format!("Failed to serialize actions: {}", e),
                }
            });
            serde_json::to_string(&error).unwrap()
        }
    }
}
```

---

### apply_action WASM Export

```rust
use crate::rules::apply_action as apply_action_internal;

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
            // Engine validation error
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

// Helper function
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
```

---

## Error Response Format

### Standard Error Structure

```json
{
  "error": {
    "code": "ERROR_CODE",
    "message": "Human-readable description",
    "context": {
      "field": "value"
    }
  }
}
```

### Error Codes Reference

**WASM Layer Errors:**
- `INVALID_JSON` - Generic JSON parsing failed
- `INVALID_STATE_JSON` - State JSON malformed
- `INVALID_ACTION_JSON` - Action JSON malformed
- `SERIALIZATION_ERROR` - Internal serialization failed
- `INVALID_PLAYER` - Player ID out of range

**Engine Layer Errors (from ValidationError):**
- `INVALID_SOURCE` - Factory index invalid
- `SOURCE_EMPTY` - No tiles of color in source
- `COLOR_MISMATCH` - Pattern line color conflict
- `WALL_CONFLICT` - Color already in wall
- `PATTERN_LINE_COMPLETE` - Row already full
- `INVALID_DESTINATION` - Destination row invalid
- `INVARIANT_VIOLATION` - Internal consistency check failed

---

## TypeScript Types & Wrappers

### File: `web/src/wasm/engine.ts`

```typescript
// Import WASM module
import * as wasm from './pkg';

// ============================================================================
// Type Definitions (matching Rust structs)
// ============================================================================

export interface GameState {
  state_version: number;
  ruleset_id: string;
  scenario_seed?: string;
  active_player_id: number;
  round_number: number;
  draft_phase_progress: 'EARLY' | 'MID' | 'LATE';
  bag: TileMultiset;
  lid: TileMultiset;
  factories: TileMultiset[];
  center: CenterArea;
  players: [PlayerBoard, PlayerBoard];
}

export interface TileMultiset {
  [color: string]: number;  // e.g., {"Blue": 3, "Red": 2}
}

export interface CenterArea {
  tiles: TileMultiset;
  has_first_player_token: boolean;
}

export interface PlayerBoard {
  score: number;
  pattern_lines: PatternLine[];
  wall: boolean[][];  // 5x5 grid
  floor_line: FloorLine;
}

export interface PatternLine {
  capacity: number;
  color?: string;
  count_filled: number;
}

export interface FloorLine {
  tiles: string[];  // Array of tile colors
  has_first_player_token: boolean;
}

export interface DraftAction {
  source: ActionSource;
  color: string;
  destination: Destination;
}

export type ActionSource = 
  | { Factory: number }
  | 'Center';

export type Destination = 
  | { PatternLine: number }
  | 'Floor';

export interface EngineError {
  error: {
    code: string;
    message: string;
    context?: any;
  };
}

// ============================================================================
// Type Guards
// ============================================================================

export function isError(result: any): result is EngineError {
  return result && typeof result === 'object' && 'error' in result;
}

export function isGameState(result: any): result is GameState {
  return result && typeof result === 'object' && 'state_version' in result;
}

export function isDraftActionArray(result: any): result is DraftAction[] {
  return Array.isArray(result);
}

// ============================================================================
// Wrapper Functions
// ============================================================================

/**
 * List all legal draft actions for the given player in the given state.
 * 
 * @param state - The current game state
 * @param playerId - The player ID (0 or 1)
 * @returns Array of legal actions or error
 */
export function listLegalActions(
  state: GameState, 
  playerId: number
): DraftAction[] | EngineError {
  try {
    const resultJson = wasm.list_legal_actions(
      JSON.stringify(state), 
      playerId
    );
    const result = JSON.parse(resultJson);
    
    if (isError(result)) {
      console.error('Engine error:', result.error);
    }
    
    return result;
  } catch (e) {
    return {
      error: {
        code: 'JS_ERROR',
        message: `JavaScript error: ${e}`,
        context: { exception: String(e) }
      }
    };
  }
}

/**
 * Apply a draft action to the given state.
 * 
 * @param state - The current game state
 * @param action - The action to apply
 * @returns New game state or error
 */
export function applyAction(
  state: GameState, 
  action: DraftAction
): GameState | EngineError {
  try {
    const resultJson = wasm.apply_action(
      JSON.stringify(state), 
      JSON.stringify(action)
    );
    const result = JSON.parse(resultJson);
    
    if (isError(result)) {
      console.error('Engine error:', result.error);
    }
    
    return result;
  } catch (e) {
    return {
      error: {
        code: 'JS_ERROR',
        message: `JavaScript error: ${e}`,
        context: { exception: String(e) }
      }
    };
  }
}

// ============================================================================
// Helper Functions
// ============================================================================

/**
 * Get a human-readable description of an action.
 */
export function describeAction(action: DraftAction): string {
  const sourceStr = typeof action.source === 'string' 
    ? 'Center' 
    : `Factory ${action.source.Factory}`;
  
  const destStr = typeof action.destination === 'string'
    ? 'Floor'
    : `Pattern Line ${action.destination.PatternLine}`;
  
  return `Take ${action.color} from ${sourceStr} to ${destStr}`;
}

/**
 * Count tiles of a specific color in a multiset.
 */
export function countTiles(multiset: TileMultiset, color: string): number {
  return multiset[color] || 0;
}

/**
 * Get total tile count in a multiset.
 */
export function totalTiles(multiset: TileMultiset): number {
  return Object.values(multiset).reduce((sum, count) => sum + count, 0);
}
```

---

## Demo UI Component

### File: `web/src/components/EngineTester.tsx`

```typescript
import React, { useState } from 'react';
import * as engine from '../wasm/engine';
import './EngineTester.css';

// Hard-coded test scenario
const TEST_SCENARIO: engine.GameState = {
  state_version: 1,
  ruleset_id: 'azul_v1_2p',
  scenario_seed: 'demo_scenario',
  active_player_id: 0,
  round_number: 2,
  draft_phase_progress: 'MID',
  bag: { Blue: 8, Yellow: 10, Red: 7, Black: 9, White: 11 },
  lid: { Blue: 2, Red: 3 },
  factories: [
    { Blue: 2, Red: 1, Yellow: 1 },
    { Black: 3, White: 1 },
    {},
    { Blue: 1, Yellow: 3 },
    { Red: 2, Black: 2 }
  ],
  center: {
    tiles: { White: 2, Red: 1 },
    has_first_player_token: true
  },
  players: [
    {
      score: 12,
      pattern_lines: [
        { capacity: 1, color: 'Blue', count_filled: 1 },
        { capacity: 2, color: 'Red', count_filled: 2 },
        { capacity: 3, color: undefined, count_filled: 0 },
        { capacity: 4, color: 'Yellow', count_filled: 3 },
        { capacity: 5, color: undefined, count_filled: 0 }
      ],
      wall: [
        [true, false, false, false, false],
        [false, false, true, false, false],
        [false, false, false, true, false],
        [false, false, false, false, false],
        [false, false, false, false, false]
      ],
      floor_line: { tiles: ['Black'], has_first_player_token: false }
    },
    {
      score: 15,
      pattern_lines: [
        { capacity: 1, color: undefined, count_filled: 0 },
        { capacity: 2, color: undefined, count_filled: 0 },
        { capacity: 3, color: 'Blue', count_filled: 3 },
        { capacity: 4, color: undefined, count_filled: 0 },
        { capacity: 5, color: 'White', count_filled: 4 }
      ],
      wall: [
        [false, true, false, false, false],
        [false, false, false, true, false],
        [false, false, false, false, true],
        [true, false, false, false, false],
        [false, false, false, false, false]
      ],
      floor_line: { tiles: [], has_first_player_token: false }
    }
  ]
};

export function EngineTester() {
  const [state, setState] = useState<engine.GameState | null>(null);
  const [actions, setActions] = useState<engine.DraftAction[] | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [message, setMessage] = useState<string>('');

  const loadScenario = () => {
    setState(TEST_SCENARIO);
    setActions(null);
    setError(null);
    setMessage('Test scenario loaded successfully');
  };

  const showLegalActions = () => {
    if (!state) {
      setError('No state loaded');
      return;
    }

    const result = engine.listLegalActions(state, state.active_player_id);
    
    if (engine.isError(result)) {
      setError(`Error: ${result.error.message}`);
      setActions(null);
    } else {
      setActions(result);
      setError(null);
      setMessage(`Found ${result.length} legal actions`);
    }
  };

  const applyFirstAction = () => {
    if (!state || !actions || actions.length === 0) {
      setError('No actions available');
      return;
    }

    const action = actions[0];
    const result = engine.applyAction(state, action);
    
    if (engine.isError(result)) {
      setError(`Error: ${result.error.message}`);
    } else {
      setState(result);
      setActions(null);
      setError(null);
      setMessage(`Applied action: ${engine.describeAction(action)}`);
    }
  };

  return (
    <div className="engine-tester">
      <h2>Engine Tester</h2>
      
      <div className="controls">
        <button onClick={loadScenario}>Load Test Scenario</button>
        <button onClick={showLegalActions} disabled={!state}>
          Show Legal Actions
        </button>
        <button onClick={applyFirstAction} disabled={!actions || actions.length === 0}>
          Apply First Action
        </button>
      </div>

      {message && <div className="message">{message}</div>}
      {error && <div className="error">{error}</div>}

      {state && (
        <div className="state-display">
          <h3>Current State</h3>
          <p>Active Player: {state.active_player_id}</p>
          <p>Round: {state.round_number}</p>
          <p>Player 0 Score: {state.players[0].score}</p>
          <p>Player 1 Score: {state.players[1].score}</p>
          
          <details>
            <summary>Full State JSON</summary>
            <pre>{JSON.stringify(state, null, 2)}</pre>
          </details>
        </div>
      )}

      {actions && (
        <div className="actions-display">
          <h3>Legal Actions ({actions.length})</h3>
          <ul>
            {actions.slice(0, 10).map((action, i) => (
              <li key={i}>{engine.describeAction(action)}</li>
            ))}
            {actions.length > 10 && <li>... and {actions.length - 10} more</li>}
          </ul>
        </div>
      )}
    </div>
  );
}
```

---

## Integration Testing Strategy

### Rust Integration Tests

**File:** `rust/engine/tests/integration_test.rs`

```rust
use engine::wasm_api::{list_legal_actions, apply_action};
use engine::model::{State, DraftAction, ActionSource, Destination, TileColor};
use serde_json::Value;

#[test]
fn test_list_legal_actions_valid_state() {
    let state_json = include_str!("fixtures/mid_game_state.json");
    let result = list_legal_actions(state_json, 0);
    
    // Should parse without error
    let actions: Vec<DraftAction> = serde_json::from_str(&result)
        .expect("Result should be valid actions JSON");
    
    // Should have reasonable number of actions
    assert!(actions.len() > 0, "Should have at least one action");
    assert!(actions.len() < 100, "Should have fewer than 100 actions");
}

#[test]
fn test_list_legal_actions_invalid_json() {
    let invalid_json = "{ not valid json";
    let result = list_legal_actions(invalid_json, 0);
    
    // Should return error
    let error: Value = serde_json::from_str(&result).unwrap();
    assert!(error.get("error").is_some());
    assert_eq!(error["error"]["code"], "INVALID_JSON");
}

#[test]
fn test_list_legal_actions_invalid_player() {
    let state_json = include_str!("fixtures/mid_game_state.json");
    let result = list_legal_actions(state_json, 5);
    
    // Should return error
    let error: Value = serde_json::from_str(&result).unwrap();
    assert!(error.get("error").is_some());
    assert_eq!(error["error"]["code"], "INVALID_PLAYER");
}

#[test]
fn test_apply_action_success() {
    let state_json = include_str!("fixtures/mid_game_state.json");
    let state: State = serde_json::from_str(state_json).unwrap();
    
    // Create a valid action
    let action = DraftAction {
        source: ActionSource::Factory(0),
        color: TileColor::Blue,
        destination: Destination::Floor,
    };
    let action_json = serde_json::to_string(&action).unwrap();
    
    let result = apply_action(state_json, &action_json);
    
    // Should parse as new state
    let new_state: State = serde_json::from_str(&result)
        .expect("Result should be valid state JSON");
    
    // Active player should have toggled
    assert_ne!(new_state.active_player_id, state.active_player_id);
}

#[test]
fn test_apply_action_illegal_action() {
    let state_json = include_str!("fixtures/mid_game_state.json");
    
    // Create an illegal action (source doesn't have this color)
    let action = DraftAction {
        source: ActionSource::Factory(2),  // Empty factory
        color: TileColor::Red,
        destination: Destination::Floor,
    };
    let action_json = serde_json::to_string(&action).unwrap();
    
    let result = apply_action(state_json, &action_json);
    
    // Should return error
    let error: Value = serde_json::from_str(&result).unwrap();
    assert!(error.get("error").is_some());
    assert_eq!(error["error"]["code"], "SOURCE_EMPTY");
}

#[test]
fn test_apply_action_invalid_state_json() {
    let invalid_json = "{ not valid }";
    let action_json = r#"{"source":"Center","color":"Blue","destination":"Floor"}"#;
    
    let result = apply_action(invalid_json, action_json);
    
    // Should return error
    let error: Value = serde_json::from_str(&result).unwrap();
    assert!(error.get("error").is_some());
    assert_eq!(error["error"]["code"], "INVALID_STATE_JSON");
}

#[test]
fn test_apply_action_invalid_action_json() {
    let state_json = include_str!("fixtures/mid_game_state.json");
    let invalid_action = "{ not valid }";
    
    let result = apply_action(state_json, invalid_action);
    
    // Should return error
    let error: Value = serde_json::from_str(&result).unwrap();
    assert!(error.get("error").is_some());
    assert_eq!(error["error"]["code"], "INVALID_ACTION_JSON");
}
```

### Test Fixture

**File:** `rust/engine/tests/fixtures/mid_game_state.json`

Use the example JSON from Sprint 01A documentation.

---

## Browser Console Testing Checklist

### Manual Verification Steps

1. **Load page with dev tools open**
   - Open browser dev tools (F12)
   - Navigate to Console tab

2. **Import engine functions**
   ```javascript
   // Should be available globally or via module import
   const { listLegalActions, applyAction } = window.engine;
   ```

3. **Load test scenario**
   - Click "Load Test Scenario" button
   - Verify no errors in console
   - Verify state displays

4. **Test listLegalActions**
   ```javascript
   // Get current state from UI
   const state = getCurrentState();
   const result = listLegalActions(state, 0);
   console.log('Actions:', result);
   ```
   - Verify result is an array (not error object)
   - Verify action count is reasonable

5. **Test applyAction**
   ```javascript
   const actions = listLegalActions(state, 0);
   const newState = applyAction(state, actions[0]);
   console.log('New state:', newState);
   ```
   - Verify result is a state object (not error object)
   - Verify active_player_id changed

6. **Test error handling**
   ```javascript
   // Try illegal action
   const badAction = {
     source: 'Center',
     color: 'NonExistentColor',
     destination: 'Floor'
   };
   const result = applyAction(state, badAction);
   console.log('Error:', result.error);
   ```
   - Verify error object is returned
   - Verify error message is helpful

---

## Acceptance Criteria

- [ ] WASM functions callable from browser console
- [ ] `list_legal_actions` returns action array or error object
- [ ] `apply_action` returns new state or error object
- [ ] Error format is consistent across all error types
- [ ] TypeScript types accurately match Rust types
- [ ] Type guard functions work correctly
- [ ] Demo UI loads scenario successfully
- [ ] Demo UI displays legal actions count
- [ ] Demo UI can apply action and show result
- [ ] All Rust integration tests pass
- [ ] Manual browser console tests pass

---

## Files to Create/Modify

```
rust/engine/src/
├── lib.rs                  (UPDATE: export wasm_api module)
├── wasm_api.rs             (NEW: WASM exports)
└── tests/
    ├── integration_test.rs (NEW: integration tests)
    └── fixtures/
        └── mid_game_state.json (NEW: test data)

web/src/
├── wasm/
│   └── engine.ts           (NEW: TypeScript wrapper)
└── components/
    ├── EngineTester.tsx    (NEW: demo component)
    └── EngineTester.css    (NEW: styles)
```

### Update lib.rs

```rust
mod version;
mod model;
mod rules;
mod wasm_api;  // NEW

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());
}

// Re-export WASM API
pub use wasm_api::*;

// Keep existing exports
pub use version::*;
```

---

## Demo Flow

### User Journey

1. **User opens app**
   - Sees "Engine Tester" component
   - Three buttons visible (two disabled)

2. **User clicks "Load Test Scenario"**
   - Hard-coded mid-game state loads
   - State summary displays:
     - Active Player: 0
     - Round: 2
     - Scores: P0: 12, P1: 15
   - Full JSON available in collapsible section
   - "Show Legal Actions" button enables

3. **User clicks "Show Legal Actions"**
   - Calls `listLegalActions(state, 0)`
   - Displays: "Found 23 legal actions"
   - Lists first 10 actions in human-readable format:
     - "Take Blue from Factory 0 to Pattern Line 2"
     - "Take Blue from Factory 0 to Floor"
     - ...
   - "Apply First Action" button enables

4. **User clicks "Apply First Action"**
   - Calls `applyAction(state, actions[0])`
   - Updates state
   - Displays: "Applied action: Take Blue from Factory 0 to Pattern Line 2"
   - Active player changes: 0 → 1
   - Scores may change
   - Actions list clears
   - "Show Legal Actions" button remains enabled

5. **User can repeat**
   - Click "Show Legal Actions" again
   - Apply more actions
   - Load new scenario to reset

---

## Related Documentation

- [Sprint 01A: Data Models](Sprint_01A_Data_Models_Serialization.md)
- [Sprint 01B: Legality Checks](Sprint_01B_Rules_Legality_Checks.md)
- [Sprint 01C: Apply Action](Sprint_01C_Apply_Action_Transitions.md)
- [WASM Architecture](../engineering/05_architecture_and_wasm_boundary.md)

---

## Next Steps

After completing Sprint 01D:
- Sprint 01 is complete! All deliverables met
- Can proceed to Sprint 02 (UI rendering)
- Or Sprint 03 (end-of-round scoring)
- Or Sprint 04 (scenario generation)

The core engine is now fully functional and accessible from the web UI.
