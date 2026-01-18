import { TileMultiset } from '../../wasm/engine';
import { getTileColor } from '../../styles/colors';
import './CenterArea.css';

interface CenterAreaProps {
  tiles: TileMultiset;
  hasFirstPlayerToken: boolean;
  isSelected: boolean;
  isSelectable: boolean;
  onSelect: () => void;
}

export function CenterArea({ tiles, hasFirstPlayerToken, isSelected, isSelectable, onSelect }: CenterAreaProps) {
  const tileEntries = Object.entries(tiles).filter(([, count]) => count > 0);
  const isEmpty = tileEntries.length === 0 && !hasFirstPlayerToken;

  const handleClick = () => {
    if (isSelectable && !isEmpty) {
      onSelect();
    }
  };

  return (
    <div
      className={`center-area ${isSelected ? 'center-area--selected' : ''} ${isSelectable && !isEmpty ? 'center-area--selectable' : ''} ${isEmpty ? 'center-area--empty' : ''}`}
      onClick={handleClick}
    >
      <div className="center-label">Center</div>
      <div className="center-content">
        {hasFirstPlayerToken && (
          <div className="first-player-token" title="First Player Token">
            1
          </div>
        )}
        {tileEntries.length === 0 && !hasFirstPlayerToken ? (
          <div className="center-empty-text">Empty</div>
        ) : (
          <div className="center-tiles">
            {tileEntries.map(([color, count]) => (
              <div key={color} className="center-color-group">
                <div
                  className="tile tile--large"
                  style={{ backgroundColor: getTileColor(color) }}
                  title={color}
                />
                <span className="tile-count">{count}</span>
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
}
