import './ThinkLongerControl.css';

export type TimeBudget = 250 | 750 | 1500;

interface ThinkLongerControlProps {
  value: TimeBudget;
  onChange: (budget: TimeBudget) => void;
  disabled?: boolean;
}

export function ThinkLongerControl({ value, onChange, disabled }: ThinkLongerControlProps) {
  return (
    <div className="think-longer-control">
      <label htmlFor="time-budget">Thinking Time:</label>
      <select
        id="time-budget"
        value={value}
        onChange={(e) => onChange(Number(e.target.value) as TimeBudget)}
        disabled={disabled}
      >
        <option value={250}>Fast (250ms)</option>
        <option value={750}>Medium (750ms)</option>
        <option value={1500}>Deep (1500ms)</option>
      </select>
    </div>
  );
}
