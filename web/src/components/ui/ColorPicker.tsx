import { getTileColor } from '../../styles/colors';
import './ColorPicker.css';

interface ColorPickerProps {
  colors: string[];
  onSelect: (color: string) => void;
  onCancel: () => void;
}

export function ColorPicker({ colors, onSelect, onCancel }: ColorPickerProps) {
  return (
    <div className="color-picker-overlay" onClick={onCancel}>
      <div className="color-picker-modal" onClick={(e) => e.stopPropagation()}>
        <div className="color-picker-header">
          <h3>Select Color</h3>
          <button className="color-picker-close" onClick={onCancel} aria-label="Close">
            ✕
          </button>
        </div>
        <div className="color-picker-options">
          {colors.map((color) => (
            <button
              key={color}
              className="color-option"
              onClick={() => onSelect(color)}
              style={{ backgroundColor: getTileColor(color) }}
              title={color}
            >
              <span className="color-name">{color}</span>
            </button>
          ))}
        </div>
      </div>
    </div>
  );
}
