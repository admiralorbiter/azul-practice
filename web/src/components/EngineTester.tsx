import { useState } from 'react';
import * as engine from '../wasm/engine';
import './EngineTester.css';

// Hard-coded test scenario from documentation
const TEST_SCENARIO: engine.GameState = {
  state_version: 1,
  ruleset_id: 'azul_v1_2p',
  scenario_seed: 'demo_scenario',
  active_player_id: 0,
  round_number: 2,
  draft_phase_progress: 'MID',
  bag: { Blue: 10, Yellow: 12, Red: 9, Black: 11, White: 13 },
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
