import { TileMultiset, ActionSource } from '../../wasm/engine';
import { getTileColor } from '../../styles/colors';
import './Factory.css';

interface FactoryProps {
  factoryIndex: number;
  tiles: TileMultiset;
  isSelected: boolean;
  isSelectable: boolean;
  onSelect: (factoryIndex: number) => void;
  getDragSourceProps?: (source: ActionSource, color: string, count: number) => any;
}

export function Factory({ factoryIndex, tiles, isSelected, isSelectable, onSelect, getDragSourceProps }: FactoryProps) {
  const tileEntries = Object.entries(tiles).filter(([, count]) => count > 0);
  const isEmpty = tileEntries.length === 0;

  const handleClick = () => {
    if (isSelectable && !isEmpty) {
      onSelect(factoryIndex);
    }
  };

  const source: ActionSource = { Factory: factoryIndex };

  return (
    <div
      className={`factory ${isSelected ? 'factory--selected' : ''} ${isSelectable && !isEmpty ? 'factory--selectable' : ''} ${isEmpty ? 'factory--empty' : ''}`}
      onClick={handleClick}
      role="group"
      aria-label={`Factory ${factoryIndex + 1}, contains ${tileEntries.map(([c, count]) => `${count} ${c}`).join(', ') || 'empty'}`}
    >
      <div className="factory-label">F{factoryIndex + 1}</div>
      <div className="factory-tiles">
        {isEmpty ? (
          <div className="factory-empty-text">Empty</div>
        ) : (
          tileEntries.map(([color, count]) => {
            const dragProps = getDragSourceProps?.(source, color, count) || {};
            return (
              <div
                key={color}
                className={`factory-tile-group ${dragProps.className || ''}`}
                {...dragProps}
                role="button"
                tabIndex={0}
                aria-label={`Drag ${count} ${color} tiles from factory ${factoryIndex + 1}`}
              >
                {Array.from({ length: count }).map((_, i) => (
                  <div
                    key={i}
                    className="tile"
                    style={{ backgroundColor: getTileColor(color) }}
                    title={color}
                  />
                ))}
              </div>
            );
          })
        )}
      </div>
    </div>
  );
}
