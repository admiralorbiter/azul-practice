import { FloorLine as FloorLineType } from '../../wasm/engine';
import { getTileColor } from '../../styles/colors';
import './FloorLine.css';

const FLOOR_PENALTIES = [-1, -1, -2, -2, -2, -3, -3];

interface FloorLineProps {
  floorLine: FloorLineType;
  isHighlighted: boolean;
  isDestination: boolean;
  onSelect?: () => void;
}

export function FloorLine({ floorLine, isHighlighted, isDestination, onSelect }: FloorLineProps) {
  const { tiles, has_first_player_token } = floorLine;
  const isClickable = isDestination && onSelect;

  const handleClick = () => {
    if (isClickable) {
      onSelect();
    }
  };

  // Combine first player token and tiles for display
  const displayItems: Array<{ type: 'token' | 'tile'; color?: string }> = [];
  if (has_first_player_token) {
    displayItems.push({ type: 'token' });
  }
  tiles.forEach(color => {
    displayItems.push({ type: 'tile', color });
  });

  return (
    <div
      className={`floor-line ${isHighlighted ? 'floor-line--highlighted' : ''} ${isClickable ? 'floor-line--clickable' : ''}`}
      onClick={handleClick}
    >
      <div className="floor-line-label">Floor</div>
      <div className="floor-line-slots">
        {FLOOR_PENALTIES.map((penalty, i) => {
          const item = displayItems[i];
          return (
            <div key={i} className="floor-slot">
              <div className="floor-slot-content">
                {item?.type === 'token' ? (
                  <div className="floor-token" title="First Player Token">1</div>
                ) : item?.type === 'tile' && item.color ? (
                  <div
                    className="floor-tile"
                    style={{ backgroundColor: getTileColor(item.color) }}
                    title={item.color}
                  />
                ) : (
                  <div className="floor-empty" />
                )}
              </div>
              <div className="floor-penalty">{penalty}</div>
            </div>
          );
        })}
        {displayItems.length > 7 && (
          <div className="floor-overflow">
            +{displayItems.length - 7} more
          </div>
        )}
      </div>
    </div>
  );
}
