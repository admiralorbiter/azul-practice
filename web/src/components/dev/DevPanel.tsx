import { useState, useEffect } from 'react';
import { GameState, DraftAction, resolveEndOfRound, isError } from '../../wasm/engine';
import { getEngineVersion, VersionInfo } from '../../wasm/loader';
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
