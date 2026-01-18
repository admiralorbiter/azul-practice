use crate::TileColor;

/// Get the wall column index for a given row and tile color
///
/// The Azul wall has a fixed color pattern where each row is rotated by 1 position.
/// This function returns which column a tile of the given color would be placed in
/// for the given row.
///
/// # Panics
///
/// Panics if row is >= 5
///
/// # Example
///
/// ```
/// use engine::{TileColor, get_wall_column_for_color};
///
/// // Blue goes in column 0 for row 0
/// assert_eq!(get_wall_column_for_color(0, TileColor::Blue), 0);
///
/// // Blue goes in column 1 for row 1 (rotated)
/// assert_eq!(get_wall_column_for_color(1, TileColor::Blue), 1);
/// ```
pub fn get_wall_column_for_color(row: usize, color: TileColor) -> usize {
    match (row, color) {
        // Row 0: Blue, Yellow, Red, Black, White
        (0, TileColor::Blue)   => 0,
        (0, TileColor::Yellow) => 1,
        (0, TileColor::Red)    => 2,
        (0, TileColor::Black)  => 3,
        (0, TileColor::White)  => 4,
        
        // Row 1: White, Blue, Yellow, Red, Black
        (1, TileColor::White)  => 0,
        (1, TileColor::Blue)   => 1,
        (1, TileColor::Yellow) => 2,
        (1, TileColor::Red)    => 3,
        (1, TileColor::Black)  => 4,
        
        // Row 2: Black, White, Blue, Yellow, Red
        (2, TileColor::Black)  => 0,
        (2, TileColor::White)  => 1,
        (2, TileColor::Blue)   => 2,
        (2, TileColor::Yellow) => 3,
        (2, TileColor::Red)    => 4,
        
        // Row 3: Red, Black, White, Blue, Yellow
        (3, TileColor::Red)    => 0,
        (3, TileColor::Black)  => 1,
        (3, TileColor::White)  => 2,
        (3, TileColor::Blue)   => 3,
        (3, TileColor::Yellow) => 4,
        
        // Row 4: Yellow, Red, Black, White, Blue
        (4, TileColor::Yellow) => 0,
        (4, TileColor::Red)    => 1,
        (4, TileColor::Black)  => 2,
        (4, TileColor::White)  => 3,
        (4, TileColor::Blue)   => 4,
        
        _ => panic!("Invalid row: {} (must be 0-4)", row),
    }
}

/// Get the tile color at a specific wall position
///
/// Returns which color belongs at the given wall position according to
/// the standard Azul wall pattern.
///
/// # Panics
///
/// Panics if row or col is >= 5
///
/// # Example
///
/// ```
/// use engine::{TileColor, get_wall_color};
///
/// // Position [0][0] is Blue
/// assert_eq!(get_wall_color(0, 0), TileColor::Blue);
///
/// // Position [1][0] is White (rotated pattern)
/// assert_eq!(get_wall_color(1, 0), TileColor::White);
/// ```
pub fn get_wall_color(row: usize, col: usize) -> TileColor {
    match (row, col) {
        // Row 0: Blue, Yellow, Red, Black, White
        (0, 0) => TileColor::Blue,
        (0, 1) => TileColor::Yellow,
        (0, 2) => TileColor::Red,
        (0, 3) => TileColor::Black,
        (0, 4) => TileColor::White,
        
        // Row 1: White, Blue, Yellow, Red, Black
        (1, 0) => TileColor::White,
        (1, 1) => TileColor::Blue,
        (1, 2) => TileColor::Yellow,
        (1, 3) => TileColor::Red,
        (1, 4) => TileColor::Black,
        
        // Row 2: Black, White, Blue, Yellow, Red
        (2, 0) => TileColor::Black,
        (2, 1) => TileColor::White,
        (2, 2) => TileColor::Blue,
        (2, 3) => TileColor::Yellow,
        (2, 4) => TileColor::Red,
        
        // Row 3: Red, Black, White, Blue, Yellow
        (3, 0) => TileColor::Red,
        (3, 1) => TileColor::Black,
        (3, 2) => TileColor::White,
        (3, 3) => TileColor::Blue,
        (3, 4) => TileColor::Yellow,
        
        // Row 4: Yellow, Red, Black, White, Blue
        (4, 0) => TileColor::Yellow,
        (4, 1) => TileColor::Red,
        (4, 2) => TileColor::Black,
        (4, 3) => TileColor::White,
        (4, 4) => TileColor::Blue,
        
        _ => panic!("Invalid wall position: [{}, {}] (must be 0-4, 0-4)", row, col),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wall_pattern_consistency() {
        // Verify that get_wall_color and get_wall_column_for_color are inverses
        for row in 0..5 {
            for col in 0..5 {
                let color = get_wall_color(row, col);
                let computed_col = get_wall_column_for_color(row, color);
                assert_eq!(
                    computed_col, col,
                    "Color {:?} at [{}, {}] should map back to column {}",
                    color, row, col, col
                );
            }
        }
    }

    #[test]
    fn test_each_color_once_per_row() {
        // Verify each color appears exactly once in each row
        for row in 0..5 {
            let mut colors_seen = std::collections::HashSet::new();
            for col in 0..5 {
                let color = get_wall_color(row, col);
                assert!(
                    colors_seen.insert(color),
                    "Color {:?} appears twice in row {}",
                    color, row
                );
            }
            assert_eq!(colors_seen.len(), 5);
        }
    }
}
