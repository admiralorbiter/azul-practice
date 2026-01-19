import { useState, useEffect } from 'react';
import { GameState, DraftAction, resolveEndOfRound, isError, describeAction } from '../../wasm/engine';
import { getEngineVersion, VersionInfo } from '../../wasm/loader';
import { evaluateBestMove, EvaluationResult } from '../../wasm/evaluator';
import './DevPanel.css';

interface DevPanelProps {
  gameState: GameState | null;
  legalActions: DraftAction[] | null;
  onStateChange?: (newState: GameState) => void;
}

export function DevPanel({ gameState, legalActions, onStateChange }: DevPanelProps) {
  const [isExpanded, setIsExpanded] = useState(false);
  const [version, setVersion] = useState<VersionInfo | null>(null);
  const [copyMessage, setCopyMessage] = useState<string>('');
  const [evaluating, setEvaluating] = useState(false);
  const [evaluationResult, setEvaluationResult] = useState<EvaluationResult | null>(null);
  const [evalError, setEvalError] = useState<string>('');

  useEffect(() => {
    getEngineVersion()
      .then(setVersion)
      .catch(err => console.error('Failed to get version:', err));
  }, []);

  const handleCopyState = () => {
    if (!gameState) return;

    const stateJson = JSON.stringify(gameState, null, 2);
    navigator.clipboard.writeText(stateJson)
      .then(() => {
        setCopyMessage('State copied to clipboard!');
        setTimeout(() => setCopyMessage(''), 3000);
      })
      .catch(err => {
        console.error('Failed to copy:', err);
        setCopyMessage('Failed to copy');
        setTimeout(() => setCopyMessage(''), 3000);
      });
  };

  const handleResolveRound = () => {
    if (!gameState || !onStateChange) return;
    
    const result = resolveEndOfRound(gameState);
    
    if (isError(result)) {
      alert(`Error resolving round: ${result.error.message}`);
    } else {
      onStateChange(result);
    }
  };

  const handleEvaluate = () => {
    if (!gameState) return;
    
    setEvaluating(true);
    setEvalError('');
    setEvaluationResult(null);
    
    try {
      const result = evaluateBestMove(gameState, gameState.active_player_id, {
        evaluator_seed: Date.now(),
        time_budget_ms: 250,
        rollouts_per_action: 10,
        shortlist_size: 20,
        rollout_config: {
          active_player_policy: 'all_greedy',
          opponent_policy: 'all_greedy'
        }
      });
      
      setEvaluationResult(result);
    } catch (error) {
      console.error('Evaluation failed:', error);
      setEvalError(error instanceof Error ? error.message : 'Unknown error');
    } finally {
      setEvaluating(false);
    }
  };

  const toggleExpanded = () => {
    setIsExpanded(!isExpanded);
  };

  return (
    <div className="dev-panel">
      <button className="dev-panel-toggle" onClick={toggleExpanded}>
        <span className="dev-panel-icon">{isExpanded ? '▼' : '▶'}</span>
        Dev Panel
      </button>

      {isExpanded && (
        <div className="dev-panel-content">
          <div className="dev-panel-section">
            <h4>Engine Info</h4>
            {version && (
              <div className="dev-info-grid">
                <div className="dev-info-item">
                  <span className="dev-label">Engine Version:</span>
                  <span className="dev-value">{version.engine_version}</span>
                </div>
                <div className="dev-info-item">
                  <span className="dev-label">State Version:</span>
                  <span className="dev-value">{version.state_version}</span>
                </div>
                <div className="dev-info-item">
                  <span className="dev-label">Ruleset ID:</span>
                  <span className="dev-value">{version.ruleset_id}</span>
                </div>
              </div>
            )}
          </div>

          {gameState && (
            <>
              <div className="dev-panel-section">
                <h4>Game State</h4>
                <div className="dev-info-grid">
                  <div className="dev-info-item">
                    <span className="dev-label">Legal Actions:</span>
                    <span className="dev-value">{legalActions?.length || 0}</span>
                  </div>
                  <div className="dev-info-item">
                    <span className="dev-label">Round Stage:</span>
                    <span className="dev-value">{gameState.draft_phase_progress}</span>
                  </div>
                  {gameState.scenario_game_stage && (
                    <div className="dev-info-item">
                      <span className="dev-label">Game Stage:</span>
                      <span className="dev-value">{gameState.scenario_game_stage}</span>
                    </div>
                  )}
                  <div className="dev-info-item dev-info-item-full">
                    <span className="dev-label">Scenario Seed:</span>
                    <span className="dev-value dev-seed">
                      {gameState.scenario_seed || 'N/A'}
                    </span>
                    {gameState.scenario_seed && (
                      <button 
                        onClick={() => {
                          navigator.clipboard.writeText(gameState.scenario_seed!)
                            .then(() => {
                              setCopyMessage('Seed copied!');
                              setTimeout(() => setCopyMessage(''), 2000);
                            })
                            .catch(err => console.error('Failed to copy seed:', err));
                        }}
                        className="dev-btn-inline"
                        title="Copy seed"
                      >
                        📋
                      </button>
                    )}
                  </div>
                </div>
                <div className="dev-actions">
                  <button onClick={handleCopyState} className="dev-btn">
                    Copy State JSON
                  </button>
                  {copyMessage && <span className="dev-copy-message">{copyMessage}</span>}
                </div>
              </div>

              <div className="dev-panel-section">
                <details className="dev-details">
                  <summary>State JSON</summary>
                  <pre className="dev-json">{JSON.stringify(gameState, null, 2)}</pre>
                </details>
              </div>

              {legalActions && legalActions.length > 0 && (
                <div className="dev-panel-section">
                  <details className="dev-details">
                    <summary>Legal Actions ({legalActions.length})</summary>
                    <pre className="dev-json">{JSON.stringify(legalActions, null, 2)}</pre>
                  </details>
                </div>
              )}

              <div className="dev-panel-section">
                <h4>Evaluation (Sprint 5B Test)</h4>
                <button 
                  onClick={handleEvaluate}
                  disabled={evaluating || !legalActions || legalActions.length === 0}
                  className="dev-btn evaluate-btn"
                >
                  {evaluating ? 'Evaluating...' : 'Evaluate Best Move'}
                </button>
                {evalError && (
                  <div className="eval-error">Error: {evalError}</div>
                )}
                {evaluationResult && (
                  <div className="eval-result">
                    <div className="eval-metrics">
                      <div><strong>Best EV:</strong> {evaluationResult.best_action_ev?.toFixed(2) || 'N/A'}</div>
                      <div><strong>Time:</strong> {evaluationResult.metadata?.elapsed_ms || 0}ms</div>
                      <div><strong>Rollouts:</strong> {evaluationResult.metadata?.rollouts_run || 0}</div>
                      <div><strong>Candidates:</strong> {evaluationResult.metadata?.candidates_evaluated || 0} / {evaluationResult.metadata?.total_legal_actions || 0}</div>
                      <div><strong>Completed:</strong> {evaluationResult.metadata?.completed_within_budget ? '✓' : '✗'}</div>
                    </div>
                    <details className="dev-details">
                      <summary>Best Action</summary>
                      <div className="action-display">
                        <div className="action-description">
                          <strong>Move:</strong> {describeAction(evaluationResult.best_action)}
                        </div>
                        <details className="action-raw">
                          <summary>Raw JSON (0-indexed)</summary>
                          <pre className="dev-json">{JSON.stringify(evaluationResult.best_action, null, 2)}</pre>
                        </details>
                      </div>
                    </details>
                    {evaluationResult.candidates && (
                      <details className="dev-details">
                        <summary>All Candidates ({evaluationResult.candidates.length})</summary>
                        <div className="candidates-list">
                          {evaluationResult.candidates
                            .sort((a, b) => b.ev - a.ev)
                            .map((c, i) => (
                              <div key={i} className="candidate-item">
                                <div className="candidate-rank">#{i + 1}</div>
                                <div className="candidate-ev">EV: {c.ev.toFixed(2)}</div>
                                <div className="candidate-action">
                                  {describeAction(c.action)}
                                </div>
                              </div>
                            ))}
                        </div>
                      </details>
                    )}
                  </div>
                )}
              </div>

              <div className="dev-panel-section">
                <h4>Round Actions</h4>
                <button 
                  onClick={handleResolveRound}
                  disabled={!onStateChange}
                  className="dev-btn resolve-round-btn"
                >
                  Resolve End of Round
                </button>
              </div>
            </>
          )}
        </div>
      )}
    </div>
  );
}
