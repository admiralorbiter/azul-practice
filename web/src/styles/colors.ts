export const TILE_COLORS = {
  Blue: '#4A90E2',
  Yellow: '#F5D76E',
  Red: '#E94B3C',
  Black: '#2D3436',
  White: '#FFFFFF',
} as const;

export function getTileColor(color: string): string {
  return TILE_COLORS[color as keyof typeof TILE_COLORS] || '#999';
}

export function getWallColor(row: number, col: number): string {
  const pattern = [
    ['Blue', 'Yellow', 'Red', 'Black', 'White'],
    ['White', 'Blue', 'Yellow', 'Red', 'Black'],
    ['Black', 'White', 'Blue', 'Yellow', 'Red'],
    ['Red', 'Black', 'White', 'Blue', 'Yellow'],
    ['Yellow', 'Red', 'Black', 'White', 'Blue']
  ];
  return pattern[row][col];
}
