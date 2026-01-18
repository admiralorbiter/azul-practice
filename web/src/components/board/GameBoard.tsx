import { GameState, ActionSource } from '../../wasm/engine';
import { Factory } from './Factory';
import { CenterArea } from './CenterArea';
import { PlayerBoard } from './PlayerBoard';
import './GameBoard.css';

interface GameBoardProps {
  gameState: GameState;
  selectedSource?: { source: ActionSource; color: string };
  highlightedDestinations?: Set<number>;
  onFactorySelect: (factoryIndex: number) => void;
  onCenterSelect: () => void;
  onPatternLineSelect: (row: number) => void;
  onFloorSelect: () => void;
}

export function GameBoard({
  gameState,
  selectedSource,
  highlightedDestinations,
  onFactorySelect,
  onCenterSelect,
  onPatternLineSelect,
  onFloorSelect,
}: GameBoardProps) {
  const activePlayer = gameState.players[gameState.active_player_id];
  const opponentPlayer = gameState.players[1 - gameState.active_player_id];

  const isFactorySelected = (index: number): boolean => {
    return selectedSource?.source !== undefined &&
      typeof selectedSource.source !== 'string' &&
      'Factory' in selectedSource.source &&
      selectedSource.source.Factory === index;
  };

  const isCenterSelected = (): boolean => {
    return selectedSource?.source === 'Center';
  };

  return (
    <div className="game-board">
      <div className="table-area">
        <div className="factories-grid">
          {gameState.factories.map((tiles, index) => (
            <Factory
              key={index}
              factoryIndex={index}
              tiles={tiles}
              isSelected={isFactorySelected(index)}
              isSelectable={!selectedSource}
              onSelect={onFactorySelect}
            />
          ))}
        </div>

        <CenterArea
          tiles={gameState.center.tiles}
          hasFirstPlayerToken={gameState.center.has_first_player_token}
          isSelected={isCenterSelected()}
          isSelectable={!selectedSource}
          onSelect={onCenterSelect}
        />
      </div>

      <div className="players-area">
        <PlayerBoard
          player={activePlayer}
          playerIndex={gameState.active_player_id}
          isActive={true}
          highlightedDestinations={highlightedDestinations}
          onPatternLineSelect={onPatternLineSelect}
          onFloorSelect={onFloorSelect}
        />

        <PlayerBoard
          player={opponentPlayer}
          playerIndex={1 - gameState.active_player_id}
          isActive={false}
          compact={true}
        />
      </div>
    </div>
  );
}
