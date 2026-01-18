import { useState, useEffect } from 'react';
import { GameState, DraftAction } from '../../wasm/engine';
import { getEngineVersion, VersionInfo } from '../../wasm/loader';
import './DevPanel.css';

interface DevPanelProps {
  gameState: GameState | null;
  legalActions: DraftAction[] | null;
}

export function DevPanel({ gameState, legalActions }: DevPanelProps) {
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
                    <span className="dev-label">Scenario Seed:</span>
                    <span className="dev-value">{gameState.scenario_seed || 'N/A'}</span>
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
            </>
          )}
        </div>
      )}
    </div>
  );
}
