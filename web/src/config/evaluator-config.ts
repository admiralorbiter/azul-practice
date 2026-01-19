import { EvaluatorParams } from '../wasm/evaluator';
import { TimeBudget } from '../components/ui/ThinkLongerControl';

/**
 * Default time budget for evaluations (in milliseconds)
 */
export const DEFAULT_TIME_BUDGET: TimeBudget = 1500;

/**
 * Calculate rollouts per action based on time budget
 * Rule: ~25ms per rollout average
 */
export function calculateRolloutsForBudget(timeBudgetMs: number): number {
  return Math.floor(timeBudgetMs / 25);
}

/**
 * Create evaluator parameters with consistent defaults
 * 
 * @param timeBudgetMs - Time budget in milliseconds (default: 1500ms)
 * @param seed - Random seed for deterministic evaluation (default: Date.now())
 * @returns Complete EvaluatorParams object
 */
export function createEvaluatorParams(
  timeBudgetMs: number = DEFAULT_TIME_BUDGET,
  seed?: number
): Omit<EvaluatorParams, 'evaluator_seed'> & { evaluator_seed: number } {
  return {
    evaluator_seed: seed ?? Date.now(),
    time_budget_ms: timeBudgetMs,
    rollouts_per_action: calculateRolloutsForBudget(timeBudgetMs),
    shortlist_size: 20,
    rollout_config: {
      active_player_policy: 'all_greedy',
      opponent_policy: 'all_greedy'
    }
  };
}
