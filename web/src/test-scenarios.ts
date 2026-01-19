import { GameState } from './wasm/engine';

// Test scenario from Sprint 01D - mid-game state
export const MID_GAME_SCENARIO: GameState = {
  state_version: 1,
  ruleset_id: 'azul_v1_2p',
  scenario_seed: 'demo_scenario',
  active_player_id: 0,
  round_number: 2,
  draft_phase_progress: 'MID',
  scenario_game_stage: 'MID',
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

// Early game scenario - empty boards, full factories  
// Tile count: 80 (bag) + 20 (factories) = 100
export const EARLY_GAME_SCENARIO: GameState = {
  state_version: 1,
  ruleset_id: 'azul_v1_2p',
  scenario_seed: 'early_game',
  active_player_id: 0,
  round_number: 1,
  draft_phase_progress: 'START',
  scenario_game_stage: 'EARLY',
  bag: { Blue: 16, Yellow: 16, Red: 16, Black: 16, White: 16 },
  lid: {},
  factories: [
    { Blue: 2, Red: 2 },
    { Yellow: 3, Black: 1 },
    { White: 2, Blue: 2 },
    { Red: 1, Yellow: 1, Black: 2 },
    { White: 4 }
  ],
  center: {
    tiles: {},
    has_first_player_token: true
  },
  players: [
    {
      score: 0,
      pattern_lines: [
        { capacity: 1, color: undefined, count_filled: 0 },
        { capacity: 2, color: undefined, count_filled: 0 },
        { capacity: 3, color: undefined, count_filled: 0 },
        { capacity: 4, color: undefined, count_filled: 0 },
        { capacity: 5, color: undefined, count_filled: 0 }
      ],
      wall: [
        [false, false, false, false, false],
        [false, false, false, false, false],
        [false, false, false, false, false],
        [false, false, false, false, false],
        [false, false, false, false, false]
      ],
      floor_line: { tiles: [], has_first_player_token: false }
    },
    {
      score: 0,
      pattern_lines: [
        { capacity: 1, color: undefined, count_filled: 0 },
        { capacity: 2, color: undefined, count_filled: 0 },
        { capacity: 3, color: undefined, count_filled: 0 },
        { capacity: 4, color: undefined, count_filled: 0 },
        { capacity: 5, color: undefined, count_filled: 0 }
      ],
      wall: [
        [false, false, false, false, false],
        [false, false, false, false, false],
        [false, false, false, false, false],
        [false, false, false, false, false],
        [false, false, false, false, false]
      ],
      floor_line: { tiles: [], has_first_player_token: false }
    }
  ]
};

// Late game scenario - most pattern lines full
// Tile count: 9 (bag) + 76 (lid) + 4 (factories) + 2 (center) + 20 (P0) + 19 (P1) = 130 - need to fix
export const LATE_GAME_SCENARIO: GameState = {
  state_version: 1,
  ruleset_id: 'azul_v1_2p',
  scenario_seed: 'late_game',
  active_player_id: 1,
  round_number: 5,
  draft_phase_progress: 'END',
  scenario_game_stage: 'LATE',
  bag: { Blue: 2, Yellow: 2, Red: 2, Black: 2, White: 2 },
  lid: { Blue: 6, Yellow: 5, Red: 7, Black: 6, White: 10 },
  factories: [
    { Blue: 1, Red: 1 },
    {},
    { Yellow: 1 },
    {},
    {}
  ],
  center: {
    tiles: { Black: 1, White: 1 },
    has_first_player_token: false
  },
  players: [
    {
      score: 45,
      pattern_lines: [
        { capacity: 1, color: 'Blue', count_filled: 1 },
        { capacity: 2, color: 'Red', count_filled: 2 },
        { capacity: 3, color: 'Yellow', count_filled: 3 },
        { capacity: 4, color: 'Black', count_filled: 4 },
        { capacity: 5, color: undefined, count_filled: 0 }
      ],
      wall: [
        [true, true, true, false, false],
        [false, true, true, true, false],
        [false, false, true, true, true],
        [true, false, false, true, false],
        [false, true, false, false, true]
      ],
      floor_line: { tiles: ['White', 'Red'], has_first_player_token: false }
    },
    {
      score: 52,
      pattern_lines: [
        { capacity: 1, color: 'White', count_filled: 1 },
        { capacity: 2, color: undefined, count_filled: 0 },
        { capacity: 3, color: 'Black', count_filled: 3 },
        { capacity: 4, color: 'Yellow', count_filled: 4 },
        { capacity: 5, color: 'Blue', count_filled: 5 }
      ],
      wall: [
        [false, true, true, true, true],
        [true, true, false, true, true],
        [true, true, true, false, false],
        [true, true, true, true, false],
        [false, false, true, true, true]
      ],
      floor_line: { tiles: [], has_first_player_token: false }
    }
  ]
};

export const TEST_SCENARIOS = {
  early: EARLY_GAME_SCENARIO,
  mid: MID_GAME_SCENARIO,
  late: LATE_GAME_SCENARIO,
} as const;
