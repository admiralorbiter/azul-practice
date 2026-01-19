import { TileMultiset, ActionSource } from '../../wasm/engine';
import { getTileColor } from '../../styles/colors';
import './CenterArea.css';

interface CenterAreaProps {
  tiles: TileMultiset;
  hasFirstPlayerToken: boolean;
  isSelected: boolean;
  isSelectable: boolean;
  onSelect: () => void;
  getDragSourceProps?: (source: ActionSource, color: string, count: number) => any;
}

export function CenterArea({ tiles, hasFirstPlayerToken, isSelected, isSelectable, onSelect, getDragSourceProps }: CenterAreaProps) {
  const tileEntries = Object.entries(tiles).filter(([, count]) => count > 0);
  const isEmpty = tileEntries.length === 0 && !hasFirstPlayerToken;

  const handleClick = () => {
    if (isSelectable && !isEmpty) {
      onSelect();
    }
  };

  const source: ActionSource = 'Center';

  return (
    <div
      className={`center-area ${isSelected ? 'center-area--selected' : ''} ${isSelectable && !isEmpty ? 'center-area--selectable' : ''} ${isEmpty ? 'center-area--empty' : ''}`}
      onClick={handleClick}
      role="group"
      aria-label={`Center pool, ${hasFirstPlayerToken ? 'has first player token, ' : ''}contains ${tileEntries.map(([c, count]) => `${count} ${c}`).join(', ') || 'empty'}`}
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
            {tileEntries.map(([color, count]) => {
              const dragProps = getDragSourceProps?.(source, color, count) || {};
              return (
                <div
                  key={color}
                  className={`center-color-group ${dragProps.className || ''}`}
                  {...dragProps}
                  role="button"
                  tabIndex={0}
                  aria-label={`Drag ${count} ${color} tiles from center`}
                >
                  <div
                    className="tile tile--large"
                    style={{ backgroundColor: getTileColor(color) }}
                    title={color}
                  />
                  <span className="tile-count">{count}</span>
                </div>
              );
            })}
          </div>
        )}
      </div>
    </div>
  );
}
