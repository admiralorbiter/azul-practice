import { PatternLine as PatternLineType } from '../../wasm/engine';
import { getTileColor } from '../../styles/colors';
import './PatternLine.css';

interface PatternLineProps {
  row: number;
  patternLine: PatternLineType;
  isHighlighted: boolean;
  isDestination: boolean;
  onSelect?: () => void;
}

export function PatternLine({ row, patternLine, isHighlighted, isDestination, onSelect }: PatternLineProps) {
  const { capacity, color, count_filled } = patternLine;
  const isClickable = isDestination && onSelect;

  const handleClick = () => {
    if (isClickable) {
      onSelect();
    }
  };

  return (
    <div
      className={`pattern-line ${isHighlighted ? 'pattern-line--highlighted' : ''} ${isClickable ? 'pattern-line--clickable' : ''}`}
      onClick={handleClick}
    >
      <div className="pattern-line-label">{row + 1}</div>
      <div className="pattern-line-tiles">
        {Array.from({ length: capacity }).map((_, i) => {
          const isFilled = i < count_filled;
          return (
            <div
              key={i}
              className={`pattern-tile ${isFilled ? 'pattern-tile--filled' : 'pattern-tile--empty'}`}
              style={isFilled && color ? { backgroundColor: getTileColor(color) } : undefined}
              title={isFilled && color ? color : 'Empty'}
            />
          );
        })}
      </div>
    </div>
  );
}
