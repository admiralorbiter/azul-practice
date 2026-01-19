import { TileMultiset } from '../../wasm/engine';
import { getTileColor } from '../../styles/colors';
import './Factory.css';

interface FactoryProps {
  factoryIndex: number;
  tiles: TileMultiset;
  isSelected: boolean;
  isSelectable: boolean;
  onSelect: (factoryIndex: number) => void;
}

export function Factory({ factoryIndex, tiles, isSelected, isSelectable, onSelect }: FactoryProps) {
  const tileEntries = Object.entries(tiles).filter(([, count]) => count > 0);
  const isEmpty = tileEntries.length === 0;

  const handleClick = () => {
    if (isSelectable && !isEmpty) {
      onSelect(factoryIndex);
    }
  };

  return (
    <div
      className={`factory ${isSelected ? 'factory--selected' : ''} ${isSelectable && !isEmpty ? 'factory--selectable' : ''} ${isEmpty ? 'factory--empty' : ''}`}
      onClick={handleClick}
    >
      <div className="factory-label">F{factoryIndex + 1}</div>
      <div className="factory-tiles">
        {isEmpty ? (
          <div className="factory-empty-text">Empty</div>
        ) : (
          tileEntries.map(([color, count]) => (
            <div key={color} className="factory-tile-group">
              {Array.from({ length: count }).map((_, i) => (
                <div
                  key={i}
                  className="tile"
                  style={{ backgroundColor: getTileColor(color) }}
                  title={color}
                />
              ))}
            </div>
          ))
        )}
      </div>
    </div>
  );
}
