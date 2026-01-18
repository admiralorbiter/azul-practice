import { getTileColor, getWallColor } from '../../styles/colors';
import './WallGrid.css';

interface WallGridProps {
  wall: boolean[][];
}

export function WallGrid({ wall }: WallGridProps) {
  return (
    <div className="wall-grid">
      {wall.map((row, rowIndex) => (
        <div key={rowIndex} className="wall-row">
          {row.map((isFilled, colIndex) => {
            const color = getWallColor(rowIndex, colIndex);
            return (
              <div
                key={colIndex}
                className={`wall-tile ${isFilled ? 'wall-tile--filled' : 'wall-tile--empty'}`}
                style={{
                  backgroundColor: isFilled ? getTileColor(color) : 'transparent',
                  borderColor: getTileColor(color),
                }}
                title={`${color} (${rowIndex},${colIndex})`}
              />
            );
          })}
        </div>
      ))}
    </div>
  );
}
