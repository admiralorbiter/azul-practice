import { PatternLine as PatternLineType, Destination } from '../../wasm/engine';
import { getTileColor } from '../../styles/colors';
import './PatternLine.css';

interface PatternLineProps {
  row: number;
  patternLine: PatternLineType;
  isHighlighted: boolean;
  isDestination: boolean;
  onSelect?: () => void;
  getDropTargetProps?: (destination: Destination) => any;
  isDragging?: boolean;
}

export function PatternLine({ row, patternLine, isHighlighted, isDestination, onSelect, getDropTargetProps, isDragging: _isDragging }: PatternLineProps) {
  const { capacity, color, count_filled } = patternLine;
  const isClickable = isDestination && onSelect;

  const handleClick = () => {
    if (isClickable) {
      onSelect();
    }
  };

  const destination: Destination = { PatternLine: row };
  const dropProps = getDropTargetProps?.(destination) || {};
  const { className: dropClassName, ...otherDropProps } = dropProps;

  return (
    <div
      {...otherDropProps}
      className={`pattern-line ${isHighlighted ? 'pattern-line--highlighted' : ''} ${isClickable ? 'pattern-line--clickable' : ''} ${dropClassName || ''}`}
      onClick={handleClick}
      role="button"
      aria-label={`Pattern line ${row + 1}, capacity ${capacity}, currently ${count_filled} of ${capacity} ${color || 'empty'}`}
      aria-dropeffect={dropClassName?.includes('valid-target') ? 'move' : 'none'}
      tabIndex={isClickable ? 0 : -1}
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
