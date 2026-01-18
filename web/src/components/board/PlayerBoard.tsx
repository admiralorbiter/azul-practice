import { PlayerBoard as PlayerBoardType } from '../../wasm/engine';
import { PatternLine } from './PatternLine';
import { WallGrid } from './WallGrid';
import { FloorLine } from './FloorLine';
import './PlayerBoard.css';

interface PlayerBoardProps {
  player: PlayerBoardType;
  playerIndex: number;
  isActive: boolean;
  compact?: boolean;
  highlightedDestinations?: Set<number>;
  onPatternLineSelect?: (row: number) => void;
  onFloorSelect?: () => void;
}

export function PlayerBoard({
  player,
  playerIndex,
  isActive,
  compact = false,
  highlightedDestinations,
  onPatternLineSelect,
  onFloorSelect,
}: PlayerBoardProps) {
  const isFloorHighlighted = highlightedDestinations?.has(-1) || false;

  return (
    <div className={`player-board ${isActive ? 'player-board--active' : ''} ${compact ? 'player-board--compact' : ''}`}>
      <div className="player-board-header">
        <h3>Player {playerIndex}</h3>
        <div className="player-score">Score: {player.score}</div>
      </div>

      <div className="player-board-content">
        <div className="pattern-lines-section">
          <div className="section-label">Pattern Lines</div>
          {player.pattern_lines.map((patternLine, row) => {
            const isHighlighted = highlightedDestinations?.has(row) || false;
            return (
              <PatternLine
                key={row}
                row={row}
                patternLine={patternLine}
                isHighlighted={isHighlighted}
                isDestination={isHighlighted}
                onSelect={onPatternLineSelect ? () => onPatternLineSelect(row) : undefined}
              />
            );
          })}
        </div>

        <div className="wall-section">
          <div className="section-label">Wall</div>
          <WallGrid wall={player.wall} />
        </div>
      </div>

      <FloorLine
        floorLine={player.floor_line}
        isHighlighted={isFloorHighlighted}
        isDestination={isFloorHighlighted}
        onSelect={onFloorSelect}
      />
    </div>
  );
}
