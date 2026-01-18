import * as wasm from './pkg';

// ============================================================================
// Type Definitions
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
  [color: string]: number;
}

export interface CenterArea {
  tiles: TileMultiset;
  has_first_player_token: boolean;
}

export interface PlayerBoard {
  score: number;
  pattern_lines: PatternLine[];
  wall: boolean[][];
  floor_line: FloorLine;
}

export interface PatternLine {
  capacity: number;
  color?: string;
  count_filled: number;
}

export interface FloorLine {
  tiles: string[];
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
    context?: unknown;
  };
}

// ============================================================================
// Type Guards
// ============================================================================

export function isError(result: unknown): result is EngineError {
  return result !== null && typeof result === 'object' && 'error' in result;
}

export function isGameState(result: unknown): result is GameState {
  return result !== null && typeof result === 'object' && 'state_version' in result;
}

export function isDraftActionArray(result: unknown): result is DraftAction[] {
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

/**
 * Resolve end-of-round: score tiles, apply penalties, refill factories.
 * 
 * Orchestrates complete end-of-round flow:
 * 1. Pattern line resolution with wall scoring
 * 2. Floor penalty application
 * 3. Floor cleanup and first player determination
 * 4. Game end detection
 * 5. Factory refill for next round (if game continues)
 * 
 * @param state - Current game state (drafting phase should be complete)
 * @returns Updated state for next round or error
 */
export function resolveEndOfRound(
  state: GameState
): GameState | EngineError {
  try {
    const resultJson = wasm.resolve_end_of_round(JSON.stringify(state));
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
