import { useState, useEffect } from 'react';
import { PlayerBoard as PlayerBoardType, Destination } from '../../wasm/engine';
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
  getDropTargetProps?: (destination: Destination) => any;
  isDragging?: boolean;
}

export function PlayerBoard({
  player,
  playerIndex,
  isActive,
  compact = false,
  highlightedDestinations,
  onPatternLineSelect,
  onFloorSelect,
  getDropTargetProps,
  isDragging,
}: PlayerBoardProps) {
  const [prevScore, setPrevScore] = useState(player.score);
  const [isScoreChanging, setIsScoreChanging] = useState(false);

  useEffect(() => {
    if (player.score !== prevScore) {
      setIsScoreChanging(true);
      const timer = setTimeout(() => {
        setIsScoreChanging(false);
        setPrevScore(player.score);
      }, 400);
      return () => clearTimeout(timer);
    }
  }, [player.score, prevScore]);

  const isFloorHighlighted = highlightedDestinations?.has(-1) || false;

  return (
    <div className={`player-board ${isActive ? 'player-board--active' : ''} ${compact ? 'player-board--compact' : ''}`}>
      <div className="player-board-header">
        <h3>Player {playerIndex}</h3>
        <div className={`player-score ${isScoreChanging ? 'changing' : ''}`}>Score: {player.score}</div>
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
                getDropTargetProps={getDropTargetProps}
                isDragging={isDragging}
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
        getDropTargetProps={getDropTargetProps}
        isDragging={isDragging}
      />
    </div>
  );
}
