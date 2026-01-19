import { useState, useEffect } from 'react';
import { GameState, DraftAction, ActionSource, isError, listLegalActions, applyAction, describeAction, generateScenario } from '../wasm/engine';
import { gradeUserAction, EvaluationResult as EvalResult } from '../wasm/evaluator';
import { GameBoard } from './board/GameBoard';
import { ColorPicker } from './ui/ColorPicker';
import { ErrorToast } from './ui/ErrorToast';
import { DevPanel } from './dev/DevPanel';
import { EvaluationResult } from './EvaluationResult';
import { ThinkLongerControl, TimeBudget } from './ui/ThinkLongerControl';
import { useActionSelection } from '../hooks/useActionSelection';
import { TEST_SCENARIOS } from '../test-scenarios';
import './PracticeScreen.css';

export function PracticeScreen() {
  const [gameState, setGameState] = useState<GameState | null>(null);
  const [legalActions, setLegalActions] = useState<DraftAction[] | null>(null);
  const [error, setError] = useState<{ message: string; code?: string } | null>(null);
  const [colorPickerState, setColorPickerState] = useState<{ source: ActionSource; colors: string[] } | null>(null);
  const [selectedGameStage, setSelectedGameStage] = useState<'ANY' | 'EARLY' | 'MID' | 'LATE'>('ANY');
  const [selectedRoundStage, setSelectedRoundStage] = useState<'ANY' | 'START' | 'MID' | 'END'>('ANY');
  
  // Evaluation state
  const [timeBudget, setTimeBudget] = useState<TimeBudget>(250);
  const [isEvaluating, setIsEvaluating] = useState(false);
  const [evaluationResult, setEvaluationResult] = useState<EvalResult | null>(null);
  const [userAction, setUserAction] = useState<DraftAction | null>(null);
  const [stateBeforeMove, setStateBeforeMove] = useState<GameState | null>(null);

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

  const handleGenerateScenario = () => {
    const result = generateScenario({
      targetGameStage: selectedGameStage === 'ANY' ? undefined : selectedGameStage,
      targetRoundStage: selectedRoundStage === 'ANY' ? undefined : selectedRoundStage,
      policyMix: 'mixed', // Good default: 70% greedy, 30% random
    });

    if (isError(result)) {
      setError({ message: result.error.message, code: result.error.code });
    } else {
      setGameState(result);
      cancelSelection();
      setError(null);
    }
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

    const action = selectionState.action;
    setUserAction(action); // Store for evaluation
    setStateBeforeMove(gameState); // Store state BEFORE applying action
    
    const result = applyAction(gameState, action);

    if (isError(result)) {
      setError({ message: result.error.message, code: result.error.code });
    } else {
      setGameState(result);
      cancelSelection();
      setError(null);
    }
  };

  const handleEvaluate = () => {
    if (!stateBeforeMove || !userAction) return;
    
    setIsEvaluating(true);
    
    try {
      // Adjust rollouts based on time budget (more time = more rollouts)
      const rolloutsPerAction = Math.floor(timeBudget / 25);
      
      // Evaluate against the state BEFORE the move was applied
      const result = gradeUserAction(stateBeforeMove, stateBeforeMove.active_player_id, userAction, {
        evaluator_seed: Date.now(),
        time_budget_ms: timeBudget,
        rollouts_per_action: rolloutsPerAction,
        shortlist_size: 20,
        rollout_config: {
          active_player_policy: 'all_greedy',
          opponent_policy: 'all_greedy'
        }
      });
      
      setEvaluationResult(result);
    } catch (error) {
      console.error('Evaluation failed:', error);
      setError({ 
        message: error instanceof Error ? error.message : 'Evaluation failed',
        code: 'EVALUATION_ERROR'
      });
    } finally {
      setIsEvaluating(false);
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
          <div className="phase-selector">
            <label htmlFor="game-stage-select">Game Stage:</label>
            <select 
              id="game-stage-select"
              value={selectedGameStage}
              onChange={(e) => setSelectedGameStage(e.target.value as 'ANY' | 'EARLY' | 'MID' | 'LATE')}
              className="phase-select"
            >
              <option value="ANY">Any</option>
              <option value="EARLY">Early Game</option>
              <option value="MID">Mid Game</option>
              <option value="LATE">Late Game</option>
            </select>
          </div>
          
          <div className="phase-selector">
            <label htmlFor="round-stage-select">Round Stage:</label>
            <select 
              id="round-stage-select"
              value={selectedRoundStage}
              onChange={(e) => setSelectedRoundStage(e.target.value as 'ANY' | 'START' | 'MID' | 'END')}
              className="phase-select"
            >
              <option value="ANY">Any</option>
              <option value="START">Start</option>
              <option value="MID">Mid-Round</option>
              <option value="END">End</option>
            </select>
          </div>
          
          <button onClick={handleGenerateScenario} className="btn btn-primary">
            New Scenario
          </button>
          
          <details className="legacy-scenarios">
            <summary>Load Test Scenarios</summary>
            <div className="legacy-buttons">
              <button onClick={() => loadScenario('early')} className="btn btn-secondary btn-small">
                Test Early
              </button>
              <button onClick={() => loadScenario('mid')} className="btn btn-secondary btn-small">
                Test Mid
              </button>
              <button onClick={() => loadScenario('late')} className="btn btn-secondary btn-small">
                Test Late
              </button>
            </div>
          </details>
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

          {/* Evaluation Controls - Show after user makes a move */}
          {userAction && !evaluationResult && (
            <div className="evaluation-controls">
              <ThinkLongerControl 
                value={timeBudget}
                onChange={setTimeBudget}
                disabled={isEvaluating}
              />
              <button 
                className="btn btn-evaluate"
                onClick={handleEvaluate}
                disabled={isEvaluating}
              >
                {isEvaluating ? 'Evaluating...' : 'Evaluate My Move'}
              </button>
            </div>
          )}

          {/* Evaluation Result - Show after evaluation */}
          {evaluationResult && (
            <EvaluationResult 
              result={evaluationResult}
              onNextScenario={() => {
                setEvaluationResult(null);
                setUserAction(null);
                setStateBeforeMove(null);
                handleGenerateScenario();
              }}
            />
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

      <DevPanel 
        gameState={gameState} 
        legalActions={legalActions}
        onStateChange={setGameState}
      />
    </div>
  );
}
