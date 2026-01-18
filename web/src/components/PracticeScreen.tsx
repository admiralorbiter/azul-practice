import { useState, useEffect } from 'react';
import { GameState, DraftAction, ActionSource, isError, listLegalActions, applyAction, describeAction } from '../wasm/engine';
import { GameBoard } from './board/GameBoard';
import { ColorPicker } from './ui/ColorPicker';
import { ErrorToast } from './ui/ErrorToast';
import { DevPanel } from './dev/DevPanel';
import { useActionSelection } from '../hooks/useActionSelection';
import { TEST_SCENARIOS } from '../test-scenarios';
import './PracticeScreen.css';

export function PracticeScreen() {
  const [gameState, setGameState] = useState<GameState | null>(null);
  const [legalActions, setLegalActions] = useState<DraftAction[] | null>(null);
  const [error, setError] = useState<{ message: string; code?: string } | null>(null);
  const [colorPickerState, setColorPickerState] = useState<{ source: ActionSource; colors: string[] } | null>(null);

  const {
    selectionState,
    selectSource,
    selectDestination,
    cancelSelection,
    getHighlightedDestinations,
  } = useActionSelection({ gameState, legalActions });

  // Fetch legal actions whenever state changes
  useEffect(() => {
    if (gameState) {
      const result = listLegalActions(gameState, gameState.active_player_id);
      if (isError(result)) {
        setError({ message: result.error.message, code: result.error.code });
        setLegalActions(null);
      } else {
        setLegalActions(result);
        setError(null);
      }
    }
  }, [gameState]);

  const loadScenario = (scenarioKey: keyof typeof TEST_SCENARIOS) => {
    setGameState(TEST_SCENARIOS[scenarioKey]);
    cancelSelection();
    setError(null);
  };

  const handleFactorySelect = (factoryIndex: number) => {
    if (!gameState || !legalActions) return;

    const factory = gameState.factories[factoryIndex];
    const colors = Object.keys(factory).filter(color => factory[color] > 0);

    if (colors.length === 0) return;

    const source: ActionSource = { Factory: factoryIndex };

    if (colors.length === 1) {
      // Auto-select the only color
      selectSource(source, colors[0]);
    } else {
      // Show color picker
      setColorPickerState({ source, colors });
    }
  };

  const handleCenterSelect = () => {
    if (!gameState || !legalActions) return;

    const colors = Object.keys(gameState.center.tiles).filter(
      color => gameState.center.tiles[color] > 0
    );

    if (colors.length === 0) return;

    const source: ActionSource = 'Center';

    if (colors.length === 1) {
      // Auto-select the only color
      selectSource(source, colors[0]);
    } else {
      // Show color picker
      setColorPickerState({ source, colors });
    }
  };

  const handleColorSelect = (color: string) => {
    if (colorPickerState) {
      selectSource(colorPickerState.source, color);
      setColorPickerState(null);
    }
  };

  const handleColorPickerCancel = () => {
    setColorPickerState(null);
  };

  const handlePatternLineSelect = (row: number) => {
    selectDestination({ PatternLine: row });
  };

  const handleFloorSelect = () => {
    selectDestination('Floor');
  };

  const handleApplyAction = () => {
    if (selectionState.stage !== 'action-ready' || !gameState) return;

    const result = applyAction(gameState, selectionState.action);

    if (isError(result)) {
      setError({ message: result.error.message, code: result.error.code });
    } else {
      setGameState(result);
      cancelSelection();
      setError(null);
    }
  };

  const handleCancelAction = () => {
    cancelSelection();
  };

  const highlightedDestinations = selectionState.stage === 'source-selected' 
    ? getHighlightedDestinations() 
    : undefined;

  const selectedSourceInfo = selectionState.stage === 'source-selected' || selectionState.stage === 'action-ready'
    ? { source: selectionState.stage === 'source-selected' ? selectionState.source : selectionState.action.source, 
        color: selectionState.stage === 'source-selected' ? selectionState.color : selectionState.action.color }
    : undefined;

  return (
    <div className="practice-screen">
      <div className="practice-header">
        <h2>Practice Mode</h2>
        <div className="practice-controls">
          <button onClick={() => loadScenario('early')} className="btn btn-secondary">
            Load Early Game
          </button>
          <button onClick={() => loadScenario('mid')} className="btn btn-secondary">
            Load Mid Game
          </button>
          <button onClick={() => loadScenario('late')} className="btn btn-secondary">
            Load Late Game
          </button>
        </div>
      </div>

      {gameState ? (
        <>
          <div className="practice-info">
            <div className="info-item">
              <span className="info-label">Round:</span>
              <span className="info-value">{gameState.round_number}</span>
            </div>
            <div className="info-item">
              <span className="info-label">Active Player:</span>
              <span className="info-value">Player {gameState.active_player_id}</span>
            </div>
            <div className="info-item">
              <span className="info-label">Legal Actions:</span>
              <span className="info-value">{legalActions?.length || 0}</span>
            </div>
          </div>

          <GameBoard
            gameState={gameState}
            selectedSource={selectedSourceInfo}
            highlightedDestinations={highlightedDestinations}
            onFactorySelect={handleFactorySelect}
            onCenterSelect={handleCenterSelect}
            onPatternLineSelect={handlePatternLineSelect}
            onFloorSelect={handleFloorSelect}
          />

          {selectionState.stage === 'action-ready' && (
            <div className="action-confirmation">
              <div className="action-description">
                <strong>Ready to apply:</strong> {describeAction(selectionState.action)}
              </div>
              <div className="action-buttons">
                <button onClick={handleApplyAction} className="btn btn-primary">
                  Apply Move
                </button>
                <button onClick={handleCancelAction} className="btn btn-secondary">
                  Cancel
                </button>
              </div>
            </div>
          )}

          {selectionState.stage === 'source-selected' && (
            <div className="selection-hint">
              <p>Selected {selectionState.color} tiles. Click a pattern line or floor to place them.</p>
              <button onClick={handleCancelAction} className="btn btn-secondary btn-small">
                Cancel Selection
              </button>
            </div>
          )}
        </>
      ) : (
        <div className="practice-empty">
          <p>Load a scenario to start practicing</p>
        </div>
      )}

      {colorPickerState && (
        <ColorPicker
          colors={colorPickerState.colors}
          onSelect={handleColorSelect}
          onCancel={handleColorPickerCancel}
        />
      )}

      {error && (
        <ErrorToast
          message={error.message}
          code={error.code}
          onDismiss={() => setError(null)}
        />
      )}

      <DevPanel gameState={gameState} legalActions={legalActions} />
    </div>
  );
}
