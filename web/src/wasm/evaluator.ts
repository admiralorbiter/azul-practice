import * as wasm from './pkg';
import type { GameState, DraftAction } from './engine';

export interface EvaluatorParams {
  time_budget_ms?: number;
  rollouts_per_action?: number;
  evaluator_seed: number;
  shortlist_size?: number;
  rollout_config?: RolloutPolicyConfig;
}

export interface RolloutPolicyConfig {
  active_player_policy: PolicyMix;
  opponent_policy: PolicyMix;
}

export type PolicyMix =
  | 'all_random'
  | 'all_greedy'
  | { mixed: { greedy_ratio: number } };

export interface ActionFeatures {
  expected_floor_penalty: number;
  expected_completions: number;
  expected_adjacency_points: number;
  expected_tiles_to_floor: number;
  takes_first_player_token: boolean;
  tiles_acquired: number;
}

export type FeedbackCategory =
  | 'floor_penalty'
  | 'line_completion'
  | 'wasted_tiles'
  | 'adjacency'
  | 'first_player_token';

export interface FeedbackBullet {
  category: FeedbackCategory;
  text: string;
  delta: number;
}

export type Grade = 'EXCELLENT' | 'GOOD' | 'OKAY' | 'MISS';

export interface EvaluationResult {
  best_action: DraftAction;
  best_action_ev: number;
  user_action_ev?: number;
  delta_ev?: number;
  metadata: EvaluationMetadata;
  candidates?: CandidateAction[];
  best_features: ActionFeatures;
  user_features?: ActionFeatures;
  feedback?: FeedbackBullet[];
  grade?: Grade;
}

export interface EvaluationMetadata {
  elapsed_ms: number;
  rollouts_run: number;
  candidates_evaluated: number;
  total_legal_actions: number;
  seed: number;
  completed_within_budget: boolean;
}

export interface CandidateAction {
  action: DraftAction;
  ev: number;
  rollouts: number;
}

/**
 * Evaluate the best move for the given state
 */
export function evaluateBestMove(
  state: GameState,
  playerId: number,
  params: EvaluatorParams
): EvaluationResult {
  const stateJson = JSON.stringify(state);
  const paramsJson = JSON.stringify(params);
  
  const resultJson = wasm.evaluate_best_move(stateJson, playerId, paramsJson);
  const result = JSON.parse(resultJson);
  
  if (result.error) {
    throw new Error(`Evaluation failed: ${result.error.message}`);
  }
  
  return result as EvaluationResult;
}

/**
 * Grade user's action compared to the best move
 */
export function gradeUserAction(
  state: GameState,
  playerId: number,
  userAction: DraftAction,
  params: EvaluatorParams
): EvaluationResult {
  const stateJson = JSON.stringify(state);
  const userActionJson = JSON.stringify(userAction);
  const paramsJson = JSON.stringify(params);
  
  const resultJson = wasm.grade_user_action(
    stateJson,
    playerId,
    userActionJson,
    paramsJson
  );
  const result = JSON.parse(resultJson);
  
  if (result.error) {
    throw new Error(`Grading failed: ${result.error.message}`);
  }
  
  return result as EvaluationResult;
}
