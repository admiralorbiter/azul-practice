#[cfg(test)]
mod tests {
    use crate::{State, TileColor, PatternLine, ActionSource, Destination, DraftAction};
    use crate::rules::{list_legal_actions, get_wall_column_for_color, apply_action, check_tile_conservation};
    use std::collections::HashMap;

    /// Helper to create a state with tiles in factories
    fn create_test_state_with_factories() -> State {
        let mut state = State::new_test_state();
        
        // Add some tiles to factories
        state.factories[0].insert(TileColor::Blue, 2);
        state.factories[0].insert(TileColor::Red, 1);
        state.factories[1].insert(TileColor::Yellow, 3);
        state.factories[2].insert(TileColor::Black, 2);
        
        // Add tiles to center
        state.center.tiles.insert(TileColor::White, 2);
        
        state
    }

    #[test]
    fn test_basic_legal_actions() {
        let state = create_test_state_with_factories();
        let actions = list_legal_actions(&state, 0);
        
        // Should have actions for colors in factories and center
        assert!(actions.len() > 0);
        
        // Should have floor actions for each color
        let floor_actions: Vec<_> = actions.iter()
            .filter(|a| a.destination == Destination::Floor)
            .collect();
        assert!(floor_actions.len() > 0);
    }

    #[test]
    fn test_color_consistency_in_pattern_line() {
        let mut state = create_test_state_with_factories();
        
        // Set up pattern line with 2 Red tiles
        state.players[0].pattern_lines[2] = PatternLine {
            capacity: 3,
            color: Some(TileColor::Red),
            count_filled: 2,
        };
        
        let actions = list_legal_actions(&state, 0);
        
        // Should allow Red into row 2, but not Blue
        let red_to_row2 = actions.iter().any(|a| 
            a.color == TileColor::Red && 
            a.destination == Destination::PatternLine(2)
        );
        let blue_to_row2 = actions.iter().any(|a| 
            a.color == TileColor::Blue && 
            a.destination == Destination::PatternLine(2)
        );
        
        assert!(red_to_row2, "Red should be legal in row 2");
        assert!(!blue_to_row2, "Blue should be illegal in row 2");
    }

    #[test]
    fn test_wall_conflict() {
        let mut state = create_test_state_with_factories();
        
        // Fill wall position where Yellow goes in row 1
        state.players[0].wall[1][2] = true;  // Yellow in row 1
        
        let actions = list_legal_actions(&state, 0);
        
        // Should not allow Yellow into row 1 pattern line
        let yellow_to_row1 = actions.iter().any(|a| 
            a.color == TileColor::Yellow && 
            a.destination == Destination::PatternLine(1)
        );
        
        assert!(!yellow_to_row1, "Yellow should be blocked by wall conflict in row 1");
        
        // But Yellow to floor should still be legal
        let yellow_to_floor = actions.iter().any(|a| 
            a.color == TileColor::Yellow && 
            a.destination == Destination::Floor
        );
        assert!(yellow_to_floor, "Yellow to floor should always be legal");
    }

    #[test]
    fn test_complete_pattern_line() {
        let mut state = create_test_state_with_factories();
        
        // Fill row 3 pattern line completely
        state.players[0].pattern_lines[3] = PatternLine {
            capacity: 4,
            color: Some(TileColor::Blue),
            count_filled: 4,
        };
        
        let actions = list_legal_actions(&state, 0);
        
        // Should not allow any color into row 3
        let any_to_row3 = actions.iter().any(|a| 
            a.destination == Destination::PatternLine(3)
        );
        
        assert!(!any_to_row3, "Complete pattern line should not accept tiles");
    }

    #[test]
    fn test_taking_from_center_with_token_is_legal() {
        let mut state = State::new_test_state();
        state.center.has_first_player_token = true;
        state.center.tiles.insert(TileColor::Blue, 3);
        
        let actions = list_legal_actions(&state, 0);
        
        // Should have actions from center (token doesn't block)
        let center_actions: Vec<_> = actions.iter()
            .filter(|a| a.source == ActionSource::Center)
            .collect();
        
        assert!(center_actions.len() > 0, "Token should not block center actions");
    }

    #[test]
    fn test_floor_always_available() {
        let mut state = State::new_test_state();
        
        // Block all pattern lines for Red by filling wall positions
        for row in 0..5 {
            let wall_col = get_wall_column_for_color(row, TileColor::Red);
            state.players[0].wall[row][wall_col] = true;
        }
        
        // Add Red tiles to a factory
        state.factories[0].insert(TileColor::Red, 2);
        
        let actions = list_legal_actions(&state, 0);
        
        // Should still have Red to Floor action
        let red_to_floor = actions.iter().any(|a| 
            a.color == TileColor::Red && 
            a.destination == Destination::Floor
        );
        
        assert!(red_to_floor, "Floor should always be available");
        
        // Should NOT have Red to any pattern line
        let red_to_pattern = actions.iter().any(|a| 
            a.color == TileColor::Red && 
            matches!(a.destination, Destination::PatternLine(_))
        );
        
        assert!(!red_to_pattern, "Red should be blocked from all pattern lines");
    }

    #[test]
    fn test_empty_factories_generate_no_actions() {
        let state = State::new_test_state();  // All factories empty
        let actions = list_legal_actions(&state, 0);
        
        assert_eq!(actions.len(), 0, "Empty state should have no actions");
    }

    #[test]
    fn test_action_count_with_full_factories() {
        let mut state = State::new_test_state();
        
        // Fill all factories with diverse tiles
        state.factories[0].insert(TileColor::Blue, 2);
        state.factories[0].insert(TileColor::Red, 2);
        state.factories[1].insert(TileColor::Yellow, 3);
        state.factories[1].insert(TileColor::Black, 1);
        state.factories[2].insert(TileColor::White, 2);
        state.factories[2].insert(TileColor::Blue, 2);
        
        let actions = list_legal_actions(&state, 0);
        
        // With empty board, each color should have 6 destinations (5 rows + floor)
        // We have 6 color instances across factories
        // Minimum: 6 colors × 6 destinations = 36 actions
        assert!(actions.len() >= 36, "Should have many actions with full factories");
    }

    #[test]
    fn test_partially_filled_pattern_line_blocks_other_colors() {
        let mut state = create_test_state_with_factories();
        
        // Add different color tiles
        state.factories[0].insert(TileColor::Blue, 2);
        state.factories[0].insert(TileColor::Yellow, 1);
        
        // Partially fill row 1 with Blue
        state.players[0].pattern_lines[1] = PatternLine {
            capacity: 2,
            color: Some(TileColor::Blue),
            count_filled: 1,
        };
        
        let actions = list_legal_actions(&state, 0);
        
        // Blue should be allowed into row 1
        let blue_to_row1 = actions.iter().any(|a| 
            a.color == TileColor::Blue && 
            a.destination == Destination::PatternLine(1)
        );
        assert!(blue_to_row1, "Blue should match existing color in row 1");
        
        // Yellow should NOT be allowed into row 1
        let yellow_to_row1 = actions.iter().any(|a| 
            a.color == TileColor::Yellow && 
            a.destination == Destination::PatternLine(1)
        );
        assert!(!yellow_to_row1, "Yellow should be blocked by Blue in row 1");
    }

    #[test]
    fn test_can_place_returns_false_for_complete_line() {
        use crate::PlayerBoard;
        use crate::rules::legality;
        
        let mut player = PlayerBoard::new();
        player.pattern_lines[2] = PatternLine {
            capacity: 3,
            color: Some(TileColor::Red),
            count_filled: 3,  // Complete!
        };
        
        let result = legality::can_place_in_pattern_line(&player, 2, TileColor::Red);
        assert!(!result, "Cannot place in complete pattern line");
    }

    #[test]
    fn test_can_place_checks_wall_conflict() {
        use crate::PlayerBoard;
        use crate::rules::legality;
        
        let mut player = PlayerBoard::new();
        
        // Fill wall position for Blue in row 0
        player.wall[0][0] = true;  // Blue is at [0][0]
        
        let result = legality::can_place_in_pattern_line(&player, 0, TileColor::Blue);
        assert!(!result, "Cannot place Blue in row 0 due to wall conflict");
        
        // But other colors should work
        let result = legality::can_place_in_pattern_line(&player, 0, TileColor::Yellow);
        assert!(result, "Yellow should be allowed in row 0");
    }

    // ============================================================
    // apply_action tests
    // ============================================================

    #[test]
    fn test_simple_action_no_overflow() {
        let mut state = State::new_test_state();
        state.factories[0].insert(TileColor::Blue, 2);
        // Add remaining tiles to bag for conservation
        state.bag.insert(TileColor::Blue, 18);
        state.bag.insert(TileColor::Yellow, 20);
        state.bag.insert(TileColor::Red, 20);
        state.bag.insert(TileColor::Black, 20);
        state.bag.insert(TileColor::White, 20);
        
        let action = DraftAction {
            source: ActionSource::Factory(0),
            color: TileColor::Blue,
            destination: Destination::PatternLine(2),
        };
        
        let new_state = apply_action(&state, &action).unwrap();
        
        // Pattern line should have 2 Blue tiles
        assert_eq!(new_state.players[0].pattern_lines[2].count_filled, 2);
        assert_eq!(new_state.players[0].pattern_lines[2].color, Some(TileColor::Blue));
        
        // No overflow to floor
        assert_eq!(new_state.players[0].floor_line.tiles.len(), 0);
        
        // Active player should toggle
        assert_eq!(new_state.active_player_id, 1);
    }

    #[test]
    fn test_action_with_overflow() {
        let mut state = State::new_test_state();
        state.players[0].pattern_lines[1] = PatternLine {
            capacity: 2,
            color: Some(TileColor::Red),
            count_filled: 1,
        };
        state.factories[0].insert(TileColor::Red, 3);
        // Add tiles for conservation: 1 in pattern + 3 in factory + 96 in bag = 100
        state.bag.insert(TileColor::Blue, 20);
        state.bag.insert(TileColor::Yellow, 20);
        state.bag.insert(TileColor::Red, 16);
        state.bag.insert(TileColor::Black, 20);
        state.bag.insert(TileColor::White, 20);
        
        let action = DraftAction {
            source: ActionSource::Factory(0),
            color: TileColor::Red,
            destination: Destination::PatternLine(1),
        };
        
        let new_state = apply_action(&state, &action).unwrap();
        
        // Pattern line should be complete (1 + 1 = 2)
        assert_eq!(new_state.players[0].pattern_lines[1].count_filled, 2);
        
        // 2 tiles should overflow to floor (3 taken - 1 placed)
        assert_eq!(new_state.players[0].floor_line.tiles.len(), 2);
        assert_eq!(new_state.players[0].floor_line.tiles[0], TileColor::Red);
        assert_eq!(new_state.players[0].floor_line.tiles[1], TileColor::Red);
    }

    #[test]
    fn test_factory_remnants_move_to_center() {
        let mut state = State::new_test_state();
        state.factories[0].insert(TileColor::Blue, 2);
        state.factories[0].insert(TileColor::Red, 1);
        state.factories[0].insert(TileColor::Yellow, 1);
        // Add tiles for conservation: 4 in factory + 96 in bag = 100
        state.bag.insert(TileColor::Blue, 18);
        state.bag.insert(TileColor::Yellow, 19);
        state.bag.insert(TileColor::Red, 19);
        state.bag.insert(TileColor::Black, 20);
        state.bag.insert(TileColor::White, 20);
        
        let action = DraftAction {
            source: ActionSource::Factory(0),
            color: TileColor::Blue,
            destination: Destination::Floor,
        };
        
        let new_state = apply_action(&state, &action).unwrap();
        
        // Factory should be cleared
        assert_eq!(new_state.factories[0].len(), 0);
        
        // Red and Yellow should be in center
        assert_eq!(*new_state.center.tiles.get(&TileColor::Red).unwrap_or(&0), 1);
        assert_eq!(*new_state.center.tiles.get(&TileColor::Yellow).unwrap_or(&0), 1);
        
        // Blue should NOT be in center (was taken)
        assert_eq!(*new_state.center.tiles.get(&TileColor::Blue).unwrap_or(&0), 0);
    }

    #[test]
    fn test_first_player_token_transfer() {
        let mut state = State::new_test_state();
        state.center.has_first_player_token = true;
        state.center.tiles.insert(TileColor::White, 2);
        // Add tiles for conservation: 2 in center + 98 in bag = 100
        state.bag.insert(TileColor::Blue, 20);
        state.bag.insert(TileColor::Yellow, 20);
        state.bag.insert(TileColor::Red, 20);
        state.bag.insert(TileColor::Black, 20);
        state.bag.insert(TileColor::White, 18);
        
        let action = DraftAction {
            source: ActionSource::Center,
            color: TileColor::White,
            destination: Destination::PatternLine(4),
        };
        
        let new_state = apply_action(&state, &action).unwrap();
        
        // Token should be removed from center
        assert!(!new_state.center.has_first_player_token);
        
        // Token should be on player's floor line
        assert!(new_state.players[0].floor_line.has_first_player_token);
        
        // Tiles should be placed correctly
        assert_eq!(new_state.players[0].pattern_lines[4].count_filled, 2);
    }

    #[test]
    fn test_all_tiles_to_floor() {
        let mut state = State::new_test_state();
        state.factories[0].insert(TileColor::Black, 3);
        // Add tiles for conservation: 3 in factory + 97 in bag = 100
        state.bag.insert(TileColor::Blue, 20);
        state.bag.insert(TileColor::Yellow, 20);
        state.bag.insert(TileColor::Red, 20);
        state.bag.insert(TileColor::Black, 17);
        state.bag.insert(TileColor::White, 20);
        
        let action = DraftAction {
            source: ActionSource::Factory(0),
            color: TileColor::Black,
            destination: Destination::Floor,
        };
        
        let new_state = apply_action(&state, &action).unwrap();
        
        // All 3 tiles should be in floor
        assert_eq!(new_state.players[0].floor_line.tiles.len(), 3);
        assert!(new_state.players[0].floor_line.tiles.iter().all(|c| *c == TileColor::Black));
        
        // Pattern lines should be unchanged
        for pattern_line in &new_state.players[0].pattern_lines {
            assert_eq!(pattern_line.count_filled, 0);
        }
    }

    #[test]
    fn test_active_player_toggles() {
        let mut state = State::new_test_state();
        state.factories[0].insert(TileColor::Blue, 2);
        // Add tiles for conservation: 2 in factory + 98 in bag = 100
        state.bag.insert(TileColor::Blue, 18);
        state.bag.insert(TileColor::Yellow, 20);
        state.bag.insert(TileColor::Red, 20);
        state.bag.insert(TileColor::Black, 20);
        state.bag.insert(TileColor::White, 20);
        
        // Initial state: player 0 is active
        assert_eq!(state.active_player_id, 0);
        
        let action = DraftAction {
            source: ActionSource::Factory(0),
            color: TileColor::Blue,
            destination: Destination::Floor,
        };
        
        let new_state = apply_action(&state, &action).unwrap();
        
        // After action, player 1 should be active
        assert_eq!(new_state.active_player_id, 1);
        
        // Apply another action as player 1
        let mut state2 = new_state;
        state2.factories[1].insert(TileColor::Red, 1);
        // Adjust bag to account for Red tile moving from bag
        *state2.bag.get_mut(&TileColor::Red).unwrap() -= 1;
        
        let action2 = DraftAction {
            source: ActionSource::Factory(1),
            color: TileColor::Red,
            destination: Destination::Floor,
        };
        
        let new_state2 = apply_action(&state2, &action2).unwrap();
        
        // Should toggle back to player 0
        assert_eq!(new_state2.active_player_id, 0);
    }

    #[test]
    fn test_tile_conservation_after_action() {
        let mut state = State::new_test_state();
        
        // Set up initial state with tiles in various locations
        state.factories[0].insert(TileColor::Blue, 2);
        state.factories[1].insert(TileColor::Red, 3);
        state.center.tiles.insert(TileColor::Yellow, 1);
        // Add tiles for conservation: 6 in play + 94 in bag = 100
        state.bag.insert(TileColor::Blue, 18);
        state.bag.insert(TileColor::Yellow, 19);
        state.bag.insert(TileColor::Red, 17);
        state.bag.insert(TileColor::Black, 20);
        state.bag.insert(TileColor::White, 20);
        
        // Verify initial conservation
        assert!(check_tile_conservation(&state).is_ok());
        
        // Apply action
        let action = DraftAction {
            source: ActionSource::Factory(0),
            color: TileColor::Blue,
            destination: Destination::PatternLine(1),
        };
        
        let new_state = apply_action(&state, &action).unwrap();
        
        // Verify conservation still holds
        assert!(check_tile_conservation(&new_state).is_ok());
    }

    #[test]
    fn test_error_invalid_source() {
        let state = State::new_test_state();
        
        let action = DraftAction {
            source: ActionSource::Factory(99),  // Out of bounds
            color: TileColor::Blue,
            destination: Destination::Floor,
        };
        
        let result = apply_action(&state, &action);
        
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.code, "INVALID_SOURCE");
    }

    #[test]
    fn test_error_source_empty() {
        let mut state = State::new_test_state();
        state.factories[0].insert(TileColor::Blue, 2);
        
        let action = DraftAction {
            source: ActionSource::Factory(0),
            color: TileColor::Red,  // Not in factory
            destination: Destination::Floor,
        };
        
        let result = apply_action(&state, &action);
        
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.code, "SOURCE_EMPTY");
    }

    #[test]
    fn test_error_color_mismatch() {
        let mut state = State::new_test_state();
        state.factories[0].insert(TileColor::Blue, 2);
        state.players[0].pattern_lines[2] = PatternLine {
            capacity: 3,
            color: Some(TileColor::Red),
            count_filled: 1,
        };
        
        let action = DraftAction {
            source: ActionSource::Factory(0),
            color: TileColor::Blue,
            destination: Destination::PatternLine(2),  // Has Red
        };
        
        let result = apply_action(&state, &action);
        
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.code, "COLOR_MISMATCH");
    }

    #[test]
    fn test_error_wall_conflict() {
        let mut state = State::new_test_state();
        state.factories[0].insert(TileColor::Yellow, 2);
        state.players[0].wall[1][2] = true;  // Yellow position in row 1
        
        let action = DraftAction {
            source: ActionSource::Factory(0),
            color: TileColor::Yellow,
            destination: Destination::PatternLine(1),
        };
        
        let result = apply_action(&state, &action);
        
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.code, "WALL_CONFLICT");
    }

    #[test]
    fn test_error_pattern_line_complete() {
        let mut state = State::new_test_state();
        state.factories[0].insert(TileColor::Blue, 2);
        state.players[0].pattern_lines[3] = PatternLine {
            capacity: 4,
            color: Some(TileColor::Blue),
            count_filled: 4,  // Complete
        };
        
        let action = DraftAction {
            source: ActionSource::Factory(0),
            color: TileColor::Blue,
            destination: Destination::PatternLine(3),
        };
        
        let result = apply_action(&state, &action);
        
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.code, "PATTERN_LINE_COMPLETE");
    }

    #[test]
    fn test_empty_pattern_line_with_overflow() {
        let mut state = State::new_test_state();
        state.factories[0].insert(TileColor::Yellow, 4);
        // Add tiles for conservation: 4 in factory + 96 in bag = 100
        state.bag.insert(TileColor::Blue, 20);
        state.bag.insert(TileColor::Yellow, 16);
        state.bag.insert(TileColor::Red, 20);
        state.bag.insert(TileColor::Black, 20);
        state.bag.insert(TileColor::White, 20);
        
        let action = DraftAction {
            source: ActionSource::Factory(0),
            color: TileColor::Yellow,
            destination: Destination::PatternLine(0),  // Capacity 1
        };
        
        let new_state = apply_action(&state, &action).unwrap();
        
        // 1 tile in pattern line
        assert_eq!(new_state.players[0].pattern_lines[0].count_filled, 1);
        assert_eq!(new_state.players[0].pattern_lines[0].color, Some(TileColor::Yellow));
        
        // 3 tiles overflow to floor
        assert_eq!(new_state.players[0].floor_line.tiles.len(), 3);
    }

    #[test]
    fn test_exact_fit_completes_pattern_line() {
        let mut state = State::new_test_state();
        state.players[0].pattern_lines[2] = PatternLine {
            capacity: 3,
            color: Some(TileColor::Blue),
            count_filled: 1,
        };
        state.factories[0].insert(TileColor::Blue, 2);
        // Add tiles for conservation: 1 in pattern + 2 in factory + 97 in bag = 100
        state.bag.insert(TileColor::Blue, 17);
        state.bag.insert(TileColor::Yellow, 20);
        state.bag.insert(TileColor::Red, 20);
        state.bag.insert(TileColor::Black, 20);
        state.bag.insert(TileColor::White, 20);
        
        let action = DraftAction {
            source: ActionSource::Factory(0),
            color: TileColor::Blue,
            destination: Destination::PatternLine(2),
        };
        
        let new_state = apply_action(&state, &action).unwrap();
        
        // Pattern line should be exactly complete
        assert_eq!(new_state.players[0].pattern_lines[2].count_filled, 3);
        assert_eq!(new_state.players[0].pattern_lines[2].capacity, 3);
        
        // No overflow
        assert_eq!(new_state.players[0].floor_line.tiles.len(), 0);
    }

    #[test]
    fn test_factory_remnants_add_to_existing_center_tiles() {
        let mut state = State::new_test_state();
        state.factories[0].insert(TileColor::Blue, 2);
        state.factories[0].insert(TileColor::Red, 1);
        state.center.tiles.insert(TileColor::Red, 3);  // Already has Red
        // Add tiles for conservation: 2 Blue + 4 Red in play + 94 in bag = 100
        state.bag.insert(TileColor::Blue, 18);
        state.bag.insert(TileColor::Yellow, 20);
        state.bag.insert(TileColor::Red, 16);
        state.bag.insert(TileColor::Black, 20);
        state.bag.insert(TileColor::White, 20);
        
        let action = DraftAction {
            source: ActionSource::Factory(0),
            color: TileColor::Blue,
            destination: Destination::Floor,
        };
        
        let new_state = apply_action(&state, &action).unwrap();
        
        // Red count should be 3 + 1 = 4
        assert_eq!(*new_state.center.tiles.get(&TileColor::Red).unwrap(), 4);
    }

    #[test]
    fn test_taking_from_center_without_token() {
        let mut state = State::new_test_state();
        state.center.has_first_player_token = false;
        state.center.tiles.insert(TileColor::Blue, 2);
        // Add tiles for conservation: 2 in center + 98 in bag = 100
        state.bag.insert(TileColor::Blue, 18);
        state.bag.insert(TileColor::Yellow, 20);
        state.bag.insert(TileColor::Red, 20);
        state.bag.insert(TileColor::Black, 20);
        state.bag.insert(TileColor::White, 20);
        
        let action = DraftAction {
            source: ActionSource::Center,
            color: TileColor::Blue,
            destination: Destination::Floor,
        };
        
        let new_state = apply_action(&state, &action).unwrap();
        
        // Token should remain false
        assert!(!new_state.center.has_first_player_token);
        assert!(!new_state.players[0].floor_line.has_first_player_token);
    }

    #[test]
    fn test_floor_line_not_capped_at_7() {
        let mut state = State::new_test_state();
        state.factories[0].insert(TileColor::Black, 10);
        // Add tiles for conservation: 10 in factory + 90 in bag = 100
        state.bag.insert(TileColor::Blue, 20);
        state.bag.insert(TileColor::Yellow, 20);
        state.bag.insert(TileColor::Red, 20);
        state.bag.insert(TileColor::Black, 10);
        state.bag.insert(TileColor::White, 20);
        
        let action = DraftAction {
            source: ActionSource::Factory(0),
            color: TileColor::Black,
            destination: Destination::Floor,
        };
        
        let new_state = apply_action(&state, &action).unwrap();
        
        // All 10 tiles should be in floor (not capped)
        assert_eq!(new_state.players[0].floor_line.tiles.len(), 10);
    }

    // ============================================================
    // Wall tile scoring golden tests (Sprint 03B)
    // ============================================================

    #[test]
    fn test_scoring_isolated_tile() {
        use crate::rules::scoring::calculate_wall_tile_score;
        use crate::Wall;
        
        let mut wall: Wall = [[false; 5]; 5];
        wall[2][2] = true; // Isolated tile
        
        let score = calculate_wall_tile_score(&wall, 2, 2);
        assert_eq!(score, 1, "Isolated tile should score 1 point");
    }

    #[test]
    fn test_scoring_horizontal_2() {
        use crate::rules::scoring::calculate_wall_tile_score;
        use crate::Wall;
        
        let mut wall: Wall = [[false; 5]; 5];
        wall[0][0] = true;
        wall[0][1] = true; // Extends to 2 tiles
        
        let score = calculate_wall_tile_score(&wall, 0, 1);
        assert_eq!(score, 2, "Horizontal chain of 2 should score 2 points");
    }

    #[test]
    fn test_scoring_horizontal_5() {
        use crate::rules::scoring::calculate_wall_tile_score;
        use crate::Wall;
        
        let mut wall: Wall = [[false; 5]; 5];
        wall[1][0] = true;
        wall[1][1] = true;
        wall[1][2] = true; // New tile
        wall[1][3] = true;
        wall[1][4] = true;
        
        let score = calculate_wall_tile_score(&wall, 1, 2);
        assert_eq!(score, 5, "Complete horizontal row should score 5 points");
    }

    #[test]
    fn test_scoring_vertical_3() {
        use crate::rules::scoring::calculate_wall_tile_score;
        use crate::Wall;
        
        let mut wall: Wall = [[false; 5]; 5];
        wall[0][2] = true;
        wall[1][2] = true;
        wall[2][2] = true; // New tile
        
        let score = calculate_wall_tile_score(&wall, 2, 2);
        assert_eq!(score, 3, "Vertical chain of 3 should score 3 points");
    }

    #[test]
    fn test_scoring_vertical_5() {
        use crate::rules::scoring::calculate_wall_tile_score;
        use crate::Wall;
        
        let mut wall: Wall = [[false; 5]; 5];
        wall[0][3] = true;
        wall[1][3] = true;
        wall[2][3] = true; // New tile
        wall[3][3] = true;
        wall[4][3] = true;
        
        let score = calculate_wall_tile_score(&wall, 2, 3);
        assert_eq!(score, 5, "Complete vertical column should score 5 points");
    }

    #[test]
    fn test_scoring_t_shape() {
        use crate::rules::scoring::calculate_wall_tile_score;
        use crate::Wall;
        
        // T-shape: horizontal chain of 3, vertical chain of 3
        let mut wall: Wall = [[false; 5]; 5];
        wall[0][2] = true;  // Top of T
        wall[1][1] = true;  // Left of T
        wall[1][2] = true;  // New tile (center of T)
        wall[1][3] = true;  // Right of T
        wall[2][2] = true;  // Bottom of T
        
        let score = calculate_wall_tile_score(&wall, 1, 2);
        assert_eq!(score, 6, "T-shape should score 3+3=6 points");
    }

    #[test]
    fn test_scoring_cross_maximum() {
        use crate::rules::scoring::calculate_wall_tile_score;
        use crate::Wall;
        
        // Cross: complete row (5) + complete column (5)
        let mut wall: Wall = [[false; 5]; 5];
        
        // Complete row 2
        wall[2][0] = true;
        wall[2][1] = true;
        wall[2][2] = true; // New tile
        wall[2][3] = true;
        wall[2][4] = true;
        
        // Complete column 2
        wall[0][2] = true;
        wall[1][2] = true;
        // [2][2] already set
        wall[3][2] = true;
        wall[4][2] = true;
        
        let score = calculate_wall_tile_score(&wall, 2, 2);
        assert_eq!(score, 10, "Full cross should score 5+5=10 points (maximum)");
    }

    #[test]
    fn test_scoring_l_shape() {
        use crate::rules::scoring::calculate_wall_tile_score;
        use crate::Wall;
        
        // L-shape: vertical 3, horizontal 2
        let mut wall: Wall = [[false; 5]; 5];
        wall[0][0] = true;  // Top of vertical
        wall[1][0] = true;  // Middle of vertical
        wall[2][0] = true;  // New tile (corner)
        wall[2][1] = true;  // Horizontal extension
        
        let score = calculate_wall_tile_score(&wall, 2, 0);
        assert_eq!(score, 5, "L-shape should score 3+2=5 points");
    }

    #[test]
    fn test_scoring_gap_stops_chain() {
        use crate::rules::scoring::calculate_wall_tile_score;
        use crate::Wall;
        
        // Gap in horizontal chain
        let mut wall: Wall = [[false; 5]; 5];
        wall[1][0] = true;
        wall[1][1] = true;  // New tile
        // wall[1][2] is false (gap)
        wall[1][3] = true;
        wall[1][4] = true;
        
        let score = calculate_wall_tile_score(&wall, 1, 1);
        assert_eq!(score, 2, "Gap should stop chain counting, only 2 tiles");
    }

    #[test]
    fn test_scoring_corner_extends_right() {
        use crate::rules::scoring::calculate_wall_tile_score;
        use crate::Wall;
        
        let mut wall: Wall = [[false; 5]; 5];
        wall[0][0] = true;  // New tile at corner
        wall[0][1] = true;
        wall[0][2] = true;
        
        let score = calculate_wall_tile_score(&wall, 0, 0);
        assert_eq!(score, 3, "Corner extending right should score 3");
    }

    // ============================================================
    // Floor penalty tests (Sprint 03B)
    // ============================================================

    #[test]
    fn test_floor_penalty_empty() {
        use crate::rules::scoring::calculate_floor_penalty;
        use crate::FloorLine;
        
        let floor = FloorLine {
            tiles: vec![],
            has_first_player_token: false,
        };
        
        let penalty = calculate_floor_penalty(&floor);
        assert_eq!(penalty, 0, "Empty floor should have no penalty");
    }

    #[test]
    fn test_floor_penalty_with_token_only() {
        use crate::rules::scoring::calculate_floor_penalty;
        use crate::FloorLine;
        
        let floor = FloorLine {
            tiles: vec![],
            has_first_player_token: true,
        };
        
        let penalty = calculate_floor_penalty(&floor);
        assert_eq!(penalty, -1, "Token only should be -1");
    }

    #[test]
    fn test_floor_penalty_3_tiles_no_token() {
        use crate::rules::scoring::calculate_floor_penalty;
        use crate::FloorLine;
        
        let floor = FloorLine {
            tiles: vec![TileColor::Blue, TileColor::Red, TileColor::Yellow],
            has_first_player_token: false,
        };
        
        let penalty = calculate_floor_penalty(&floor);
        assert_eq!(penalty, -4, "3 tiles without token: -1-1-2 = -4");
    }

    #[test]
    fn test_floor_penalty_3_tiles_with_token() {
        use crate::rules::scoring::calculate_floor_penalty;
        use crate::FloorLine;
        
        let floor = FloorLine {
            tiles: vec![TileColor::Blue, TileColor::Red, TileColor::Yellow],
            has_first_player_token: true,
        };
        
        let penalty = calculate_floor_penalty(&floor);
        assert_eq!(penalty, -6, "Token + 3 tiles: -1(token) -1-2-2 = -6");
    }

    #[test]
    fn test_floor_penalty_maximum() {
        use crate::rules::scoring::calculate_floor_penalty;
        use crate::FloorLine;
        
        let floor = FloorLine {
            tiles: vec![
                TileColor::Blue,
                TileColor::Red,
                TileColor::Yellow,
                TileColor::Black,
                TileColor::White,
                TileColor::Blue,
                TileColor::Red,
                TileColor::Black,  // Extra tiles don't count
                TileColor::Yellow,
                TileColor::White,
            ],
            has_first_player_token: true,
        };
        
        let penalty = calculate_floor_penalty(&floor);
        assert_eq!(penalty, -14, "Maximum penalty: -1-1-2-2-2-3-3 = -14");
    }

    #[test]
    fn test_floor_penalty_7_tiles_no_token() {
        use crate::rules::scoring::calculate_floor_penalty;
        use crate::FloorLine;
        
        let floor = FloorLine {
            tiles: vec![
                TileColor::Blue,
                TileColor::Red,
                TileColor::Yellow,
                TileColor::Black,
                TileColor::White,
                TileColor::Blue,
                TileColor::Red,
            ],
            has_first_player_token: false,
        };
        
        let penalty = calculate_floor_penalty(&floor);
        assert_eq!(penalty, -14, "7 tiles without token: -1-1-2-2-2-3-3 = -14");
    }

    // ============================================================
    // Score clamping tests (Sprint 03B)
    // ============================================================

    #[test]
    fn test_score_clamping_to_zero() {
        use crate::rules::scoring::apply_floor_penalties;
        use crate::FloorLine;
        
        let mut state = State::new_test_state();
        state.players[0].score = 3;
        state.players[0].floor_line = FloorLine {
            tiles: vec![
                TileColor::Blue,
                TileColor::Red,
                TileColor::Yellow,
                TileColor::Black,
                TileColor::White,
                TileColor::Blue,
            ],
            has_first_player_token: true,
        };
        
        apply_floor_penalties(&mut state);
        
        // Score: 3 + (-13) = -10, clamped to 0
        assert_eq!(state.players[0].score, 0, "Score should be clamped to 0");
    }

    #[test]
    fn test_score_clamping_positive_result() {
        use crate::rules::scoring::apply_floor_penalties;
        use crate::FloorLine;
        
        let mut state = State::new_test_state();
        state.players[0].score = 10;
        state.players[0].floor_line = FloorLine {
            tiles: vec![TileColor::Blue, TileColor::Red],
            has_first_player_token: false,
        };
        
        apply_floor_penalties(&mut state);
        
        // Score: 10 + (-2) = 8
        assert_eq!(state.players[0].score, 8, "Score should remain positive");
    }

    // ============================================================
    // Integration tests (Sprint 03A + 03B)
    // ============================================================

    #[test]
    fn test_pattern_line_resolution_with_scoring() {
        use crate::rules::resolution::resolve_pattern_lines;
        
        let mut state = create_test_state_with_tiles();
        
        // Remove tiles from bag
        *state.bag.get_mut(&TileColor::Blue).unwrap() -= 3;
        
        // Set initial score
        state.players[0].score = 10;
        
        // Set up complete pattern line
        state.players[0].pattern_lines[2] = PatternLine {
            capacity: 3,
            color: Some(TileColor::Blue),
            count_filled: 3,
        };
        
        resolve_pattern_lines(&mut state);
        
        // Score should increase by 1 (isolated tile)
        assert_eq!(state.players[0].score, 11);
    }

    #[test]
    fn test_multiple_tiles_score_independently() {
        use crate::rules::resolution::resolve_pattern_lines;
        
        let mut state = create_test_state_with_tiles();
        
        // Remove tiles
        *state.bag.get_mut(&TileColor::Blue).unwrap() -= 1;
        *state.bag.get_mut(&TileColor::Yellow).unwrap() -= 2;
        
        // Set initial score
        state.players[0].score = 5;
        
        // Set up 2 complete pattern lines
        state.players[0].pattern_lines[0] = PatternLine {
            capacity: 1,
            color: Some(TileColor::Blue),
            count_filled: 1,
        };
        state.players[0].pattern_lines[1] = PatternLine {
            capacity: 2,
            color: Some(TileColor::Yellow),
            count_filled: 2,
        };
        
        resolve_pattern_lines(&mut state);
        
        // Each tile is isolated: +1 +1 = +2
        assert_eq!(state.players[0].score, 7);
    }

    #[test]
    fn test_complete_scoring_flow() {
        use crate::rules::resolution::resolve_pattern_lines;
        use crate::rules::scoring::apply_floor_penalties;
        use crate::FloorLine;
        
        let mut state = create_test_state_with_tiles();
        
        // Remove tiles
        *state.bag.get_mut(&TileColor::Blue).unwrap() -= 1;
        
        // Set initial score
        state.players[0].score = 5;
        
        // Complete pattern line
        state.players[0].pattern_lines[0] = PatternLine {
            capacity: 1,
            color: Some(TileColor::Blue),
            count_filled: 1,
        };
        
        // Floor line with penalties
        state.players[0].floor_line = FloorLine {
            tiles: vec![TileColor::Red, TileColor::Yellow],
            has_first_player_token: true,
        };
        
        // Resolve pattern lines (adds wall tile score)
        resolve_pattern_lines(&mut state);
        // Score: 5 + 1 = 6
        
        // Apply floor penalties
        apply_floor_penalties(&mut state);
        // Score: 6 + (-1 token -1 red -2 yellow) = 6 - 4 = 2
        
        assert_eq!(state.players[0].score, 2);
    }

    // ============================================================
    // resolve_pattern_lines tests (Sprint 03A)
    // ============================================================

    /// Helper to create a test state with specific tile distribution
    fn create_test_state_with_tiles() -> State {
        let mut state = State::new_test_state();
        
        // Add 100 tiles to bag for valid conservation
        state.bag.insert(TileColor::Blue, 20);
        state.bag.insert(TileColor::Yellow, 20);
        state.bag.insert(TileColor::Red, 20);
        state.bag.insert(TileColor::Black, 20);
        state.bag.insert(TileColor::White, 20);
        
        state
    }

    #[test]
    fn test_resolve_single_complete_pattern_line() {
        use crate::rules::resolution::resolve_pattern_lines;
        
        let mut state = create_test_state_with_tiles();
        
        // Remove 3 tiles from bag to put in pattern line
        *state.bag.get_mut(&TileColor::Blue).unwrap() -= 3;
        
        // Set up one complete pattern line
        state.players[0].pattern_lines[2] = PatternLine {
            capacity: 3,
            color: Some(TileColor::Blue),
            count_filled: 3,
        };
        
        resolve_pattern_lines(&mut state);
        
        // Verify tile placed on wall (Blue at row 2, col 2)
        assert!(state.players[0].wall[2][2]);
        
        // Verify excess tiles in lid (3 - 1 = 2 tiles)
        assert_eq!(state.lid.get(&TileColor::Blue), Some(&2));
        
        // Verify pattern line reset
        assert_eq!(state.players[0].pattern_lines[2].count_filled, 0);
        assert_eq!(state.players[0].pattern_lines[2].color, None);
        
        // Verify tile conservation
        assert!(check_tile_conservation(&state).is_ok());
    }

    #[test]
    fn test_resolve_multiple_complete_pattern_lines() {
        use crate::rules::resolution::resolve_pattern_lines;
        
        let mut state = create_test_state_with_tiles();
        
        // Remove tiles from bag
        *state.bag.get_mut(&TileColor::Blue).unwrap() -= 1;
        *state.bag.get_mut(&TileColor::Red).unwrap() -= 5;
        
        // Set up multiple complete pattern lines
        state.players[0].pattern_lines[0] = PatternLine {
            capacity: 1,
            color: Some(TileColor::Blue),
            count_filled: 1,
        };
        state.players[0].pattern_lines[4] = PatternLine {
            capacity: 5,
            color: Some(TileColor::Red),
            count_filled: 5,
        };
        
        resolve_pattern_lines(&mut state);
        
        // Verify both tiles placed
        assert!(state.players[0].wall[0][0]); // Blue at row 0, col 0
        assert!(state.players[0].wall[4][1]); // Red at row 4, col 1
        
        // Verify lid contents
        assert_eq!(state.lid.get(&TileColor::Blue).copied().unwrap_or(0), 0); // 0 discarded from row 0
        assert_eq!(state.lid.get(&TileColor::Red), Some(&4));  // 4 discarded from row 4
        
        // Verify pattern lines reset
        assert_eq!(state.players[0].pattern_lines[0].count_filled, 0);
        assert_eq!(state.players[0].pattern_lines[4].count_filled, 0);
        
        // Verify tile conservation
        assert!(check_tile_conservation(&state).is_ok());
    }

    #[test]
    fn test_resolve_no_complete_pattern_lines() {
        use crate::rules::resolution::resolve_pattern_lines;
        
        let mut state = create_test_state_with_tiles();
        
        // Remove tiles from bag
        *state.bag.get_mut(&TileColor::Yellow).unwrap() -= 2;
        
        // Set up incomplete pattern line
        state.players[0].pattern_lines[2] = PatternLine {
            capacity: 3,
            color: Some(TileColor::Yellow),
            count_filled: 2, // Not complete
        };
        
        let wall_before = state.players[0].wall;
        let lid_before = state.lid.clone();
        let pattern_line_before = state.players[0].pattern_lines[2].clone();
        
        resolve_pattern_lines(&mut state);
        
        // Verify no changes
        assert_eq!(state.players[0].wall, wall_before);
        assert_eq!(state.lid, lid_before);
        assert_eq!(state.players[0].pattern_lines[2].count_filled, pattern_line_before.count_filled);
        assert_eq!(state.players[0].pattern_lines[2].color, pattern_line_before.color);
    }

    #[test]
    fn test_resolve_both_players() {
        use crate::rules::resolution::resolve_pattern_lines;
        
        let mut state = create_test_state_with_tiles();
        
        // Remove tiles from bag
        *state.bag.get_mut(&TileColor::White).unwrap() -= 2;
        *state.bag.get_mut(&TileColor::Black).unwrap() -= 4;
        
        // Player 0
        state.players[0].pattern_lines[1] = PatternLine {
            capacity: 2,
            color: Some(TileColor::White),
            count_filled: 2,
        };
        
        // Player 1
        state.players[1].pattern_lines[3] = PatternLine {
            capacity: 4,
            color: Some(TileColor::Black),
            count_filled: 4,
        };
        
        resolve_pattern_lines(&mut state);
        
        // Verify player 0
        assert!(state.players[0].wall[1][0]); // White at row 1, col 0
        
        // Verify player 1
        assert!(state.players[1].wall[3][1]); // Black at row 3, col 1
        
        // Verify lid
        assert_eq!(state.lid.get(&TileColor::White), Some(&1)); // 2 - 1 = 1
        assert_eq!(state.lid.get(&TileColor::Black), Some(&3)); // 4 - 1 = 3
        
        // Verify tile conservation
        assert!(check_tile_conservation(&state).is_ok());
    }

    #[test]
    fn test_tile_conservation_after_resolution() {
        use crate::rules::resolution::resolve_pattern_lines;
        
        let mut state = create_test_state_with_tiles();
        
        // Set up all 5 pattern lines complete with varying colors
        *state.bag.get_mut(&TileColor::Blue).unwrap() -= 1;
        *state.bag.get_mut(&TileColor::Yellow).unwrap() -= 2;
        *state.bag.get_mut(&TileColor::Red).unwrap() -= 3;
        *state.bag.get_mut(&TileColor::Black).unwrap() -= 4;
        *state.bag.get_mut(&TileColor::White).unwrap() -= 5;
        
        state.players[0].pattern_lines[0] = PatternLine {
            capacity: 1,
            color: Some(TileColor::Blue),
            count_filled: 1,
        };
        state.players[0].pattern_lines[1] = PatternLine {
            capacity: 2,
            color: Some(TileColor::Yellow),
            count_filled: 2,
        };
        state.players[0].pattern_lines[2] = PatternLine {
            capacity: 3,
            color: Some(TileColor::Red),
            count_filled: 3,
        };
        state.players[0].pattern_lines[3] = PatternLine {
            capacity: 4,
            color: Some(TileColor::Black),
            count_filled: 4,
        };
        state.players[0].pattern_lines[4] = PatternLine {
            capacity: 5,
            color: Some(TileColor::White),
            count_filled: 5,
        };
        
        // Count tiles before
        assert!(check_tile_conservation(&state).is_ok());
        
        resolve_pattern_lines(&mut state);
        
        // Count tiles after - should still be 100
        assert!(check_tile_conservation(&state).is_ok());
    }

    #[test]
    fn test_row_0_no_discard() {
        use crate::rules::resolution::resolve_pattern_lines;
        
        let mut state = create_test_state_with_tiles();
        
        // Remove 1 tile from bag
        *state.bag.get_mut(&TileColor::Yellow).unwrap() -= 1;
        
        state.players[0].pattern_lines[0] = PatternLine {
            capacity: 1,
            color: Some(TileColor::Yellow),
            count_filled: 1,
        };
        
        resolve_pattern_lines(&mut state);
        
        // Verify tile on wall
        assert!(state.players[0].wall[0][1]); // Yellow at row 0, col 1
        
        // Verify NO tiles in lid for Yellow (capacity - 1 = 0)
        assert_eq!(state.lid.get(&TileColor::Yellow).copied().unwrap_or(0), 0);
        
        // Verify tile conservation
        assert!(check_tile_conservation(&state).is_ok());
    }

    #[test]
    fn test_all_rows_different_colors() {
        use crate::rules::resolution::resolve_pattern_lines;
        
        let mut state = create_test_state_with_tiles();
        
        // Set up all colors in different rows for both players
        *state.bag.get_mut(&TileColor::Blue).unwrap() -= 4;
        *state.bag.get_mut(&TileColor::Yellow).unwrap() -= 2;
        *state.bag.get_mut(&TileColor::Red).unwrap() -= 3;
        *state.bag.get_mut(&TileColor::Black).unwrap() -= 5;
        *state.bag.get_mut(&TileColor::White).unwrap() -= 1;
        
        // Player 0 - different colors
        state.players[0].pattern_lines[0] = PatternLine {
            capacity: 1,
            color: Some(TileColor::Red),
            count_filled: 1,
        };
        state.players[0].pattern_lines[2] = PatternLine {
            capacity: 3,
            color: Some(TileColor::Yellow),
            count_filled: 3,
        };
        
        // Player 1 - different colors
        state.players[1].pattern_lines[1] = PatternLine {
            capacity: 2,
            color: Some(TileColor::Blue),
            count_filled: 2,
        };
        state.players[1].pattern_lines[3] = PatternLine {
            capacity: 4,
            color: Some(TileColor::White),
            count_filled: 4,
        };
        state.players[1].pattern_lines[4] = PatternLine {
            capacity: 5,
            color: Some(TileColor::Black),
            count_filled: 5,
        };
        
        resolve_pattern_lines(&mut state);
        
        // Verify all walls updated correctly
        assert!(state.players[0].wall[0][2]); // Red at row 0, col 2
        assert!(state.players[0].wall[2][3]); // Yellow at row 2, col 3
        assert!(state.players[1].wall[1][1]); // Blue at row 1, col 1
        assert!(state.players[1].wall[3][2]); // White at row 3, col 2
        assert!(state.players[1].wall[4][2]); // Black at row 4, col 2
        
        // Verify lid
        assert_eq!(state.lid.get(&TileColor::Red).copied().unwrap_or(0), 0); // 1-1=0
        assert_eq!(state.lid.get(&TileColor::Yellow), Some(&2)); // 3-1=2
        assert_eq!(state.lid.get(&TileColor::Blue), Some(&1)); // 2-1=1
        assert_eq!(state.lid.get(&TileColor::White), Some(&3)); // 4-1=3
        assert_eq!(state.lid.get(&TileColor::Black), Some(&4)); // 5-1=4
        
        // Total in lid: 0+2+1+3+4 = 10 tiles
        let lid_total: u8 = state.lid.values().sum();
        assert_eq!(lid_total, 10);
        
        // Verify tile conservation
        assert!(check_tile_conservation(&state).is_ok());
    }

    // ============================================================
    // End-of-round integration tests (Sprint 03C)
    // ============================================================

    #[test]
    fn test_complete_end_of_round_flow() {
        use crate::rules::end_of_round::resolve_end_of_round;
        
        let mut state = create_test_state_with_tiles();
        state.round_number = 2;
        
        // Setup: Player 0 has complete pattern line and floor tiles
        *state.bag.get_mut(&TileColor::Blue).unwrap() -= 3;
        state.players[0].pattern_lines[2] = PatternLine {
            capacity: 3,
            color: Some(TileColor::Blue),
            count_filled: 3,
        };
        state.players[0].floor_line.tiles.push(TileColor::Red);
        state.players[0].floor_line.has_first_player_token = true;
        
        let result = resolve_end_of_round(&state).unwrap();
        
        // Verify round incremented
        assert_eq!(result.round_number, 3);
        
        // Verify pattern line resolved
        assert_eq!(result.players[0].pattern_lines[2].count_filled, 0);
        assert!(result.players[0].wall[2][2]);
        
        // Verify floor cleared
        assert_eq!(result.players[0].floor_line.tiles.len(), 0);
        assert!(!result.players[0].floor_line.has_first_player_token);
        
        // Verify token in center
        assert!(result.center.has_first_player_token);
        
        // Verify first player set correctly
        assert_eq!(result.active_player_id, 0);
        
        // Verify factories refilled
        let factory_count: u8 = result.factories.iter()
            .map(|f| f.values().sum::<u8>())
            .sum();
        assert!(factory_count > 0, "Factories should be refilled");
    }

    #[test]
    fn test_bag_refill_from_lid() {
        use crate::rules::end_of_round::resolve_end_of_round;
        
        let mut state = State::new_test_state();
        
        // Setup: Bag has only 8 tiles, lid has 18
        state.bag.insert(TileColor::Blue, 5);
        state.bag.insert(TileColor::Red, 3);
        state.lid.insert(TileColor::Yellow, 10);
        state.lid.insert(TileColor::Black, 8);
        
        let result = resolve_end_of_round(&state).unwrap();
        
        // Verify lid was emptied (transferred to bag)
        let lid_count: u8 = result.lid.values().sum();
        assert_eq!(lid_count, 0, "Lid should be empty after refill");
        
        // Verify factories filled (should have 20 tiles total)
        let factory_count: u8 = result.factories.iter()
            .map(|f| f.values().sum::<u8>())
            .sum();
        assert_eq!(factory_count, 20, "Factories should have 20 tiles");
    }

    #[test]
    fn test_game_end_detection() {
        use crate::rules::end_of_round::{resolve_end_of_round, check_game_end};
        
        let mut state = create_test_state_with_tiles();
        
        // Setup: Player 0 has complete horizontal row
        state.players[0].wall[2] = [true, true, true, true, true];
        
        // Verify check_game_end detects it
        assert!(check_game_end(&state));
        
        let result = resolve_end_of_round(&state).unwrap();
        
        // Game should end, no factory refill
        assert!(check_game_end(&result));
        let factory_count: u8 = result.factories.iter()
            .map(|f| f.values().sum::<u8>())
            .sum();
        assert_eq!(factory_count, 0, "Factories should not refill after game end");
    }

    #[test]
    fn test_partial_factory_fill_late_game() {
        use crate::rules::refill::refill_factories;
        
        let mut state = State::new_test_state();
        
        // Setup: Only 6 tiles available (bag + lid)
        state.bag.insert(TileColor::Blue, 3);
        state.bag.insert(TileColor::Red, 2);
        state.lid.insert(TileColor::Yellow, 1);
        
        refill_factories(&mut state);
        
        // Verify only 6 tiles in factories
        let factory_count: u8 = state.factories.iter()
            .map(|f| f.values().sum::<u8>())
            .sum();
        assert_eq!(factory_count, 6, "Partial fill with 6 tiles");
        
        // Verify bag and lid both empty
        assert_eq!(state.bag.values().sum::<u8>(), 0);
        assert_eq!(state.lid.values().sum::<u8>(), 0);
    }

    #[test]
    fn test_first_player_determination() {
        use crate::rules::end_of_round::resolve_end_of_round;
        
        let mut state = create_test_state_with_tiles();
        
        // Player 1 has token
        state.players[1].floor_line.has_first_player_token = true;
        state.active_player_id = 0;
        
        let result = resolve_end_of_round(&state).unwrap();
        
        // Player 1 should be first player next round
        assert_eq!(result.active_player_id, 1);
        
        // Token should be in center
        assert!(result.center.has_first_player_token);
        assert!(!result.players[0].floor_line.has_first_player_token);
        assert!(!result.players[1].floor_line.has_first_player_token);
    }

    #[test]
    fn test_tile_conservation_through_end_of_round() {
        use crate::rules::end_of_round::resolve_end_of_round;
        
        let state = create_test_state_with_tiles();
        
        // Verify conservation before
        assert!(check_tile_conservation(&state).is_ok());
        
        let result = resolve_end_of_round(&state).unwrap();
        
        // Verify conservation after
        assert!(check_tile_conservation(&result).is_ok());
    }

    #[test]
    fn test_floor_tiles_discarded_to_lid() {
        use crate::rules::end_of_round::resolve_end_of_round;
        
        let mut state = create_test_state_with_tiles();
        
        // Setup: Floor has tiles
        state.players[0].floor_line.tiles = vec![
            TileColor::Blue,
            TileColor::Red,
            TileColor::Yellow,
        ];
        
        let lid_before: u8 = state.lid.values().sum();
        
        let result = resolve_end_of_round(&state).unwrap();
        
        // Verify floor cleared
        assert_eq!(result.players[0].floor_line.tiles.len(), 0);
        
        // Verify tiles in lid (3 more than before)
        let lid_after: u8 = result.lid.values().sum();
        assert_eq!(lid_after, lid_before + 3);
    }

    #[test]
    fn test_check_game_end_no_complete_row() {
        use crate::rules::end_of_round::check_game_end;
        
        let mut state = State::new_test_state();
        
        // Almost complete row (missing one tile)
        state.players[0].wall[1] = [true, true, true, true, false];
        
        assert!(!check_game_end(&state));
    }

    // =====================================================================
    // Shared Test Helpers (Sprint 5A+)
    // =====================================================================

    /// Helper to count total tiles in state
    fn count_total_tiles(state: &State) -> u32 {
            let mut total = 0u32;
            
            // Bag and lid
            for count in state.bag.values() {
                total += *count as u32;
            }
            for count in state.lid.values() {
                total += *count as u32;
            }
            
            // Factories and center
            for factory in &state.factories {
                for count in factory.values() {
                    total += *count as u32;
                }
            }
            for count in state.center.tiles.values() {
                total += *count as u32;
            }
            
            // Player boards
            for player in &state.players {
                for pattern_line in &player.pattern_lines {
                    total += pattern_line.count_filled as u32;
                }
                for row in &player.wall {
                    for &filled in row {
                        if filled {
                            total += 1;
                        }
                    }
                }
                total += player.floor_line.tiles.len() as u32;
            }
            
            total
        }

        /// Create start-of-round state with full factories
        fn create_start_of_round_state() -> State {
            let mut state = State::new_test_state();
            
            // Add tiles to factories (20 total)
            state.factories[0].insert(TileColor::Blue, 2);
            state.factories[0].insert(TileColor::Red, 2);
            state.factories[1].insert(TileColor::Yellow, 2);
            state.factories[1].insert(TileColor::Black, 2);
            state.factories[2].insert(TileColor::White, 2);
            state.factories[2].insert(TileColor::Blue, 2);
            state.factories[3].insert(TileColor::Red, 2);
            state.factories[3].insert(TileColor::Yellow, 2);
            state.factories[4].insert(TileColor::Black, 2);
            state.factories[4].insert(TileColor::White, 2);
            
            // Bag: 80 tiles
            state.bag.insert(TileColor::Blue, 16);
            state.bag.insert(TileColor::Yellow, 16);
            state.bag.insert(TileColor::Red, 16);
            state.bag.insert(TileColor::Black, 16);
            state.bag.insert(TileColor::White, 16);
            
            state.center.has_first_player_token = true;
            
            state
        }
        
        /// Create mid-round state (some factories empty)
        fn create_mid_round_state() -> State {
            let mut state = create_start_of_round_state();
            
            // Move tiles from emptied factories to center (simulating mid-round)
            let tiles_from_f0: Vec<_> = state.factories[0].clone().into_iter().collect();
            let tiles_from_f1: Vec<_> = state.factories[1].clone().into_iter().collect();
            
            for (color, count) in tiles_from_f0 {
                *state.center.tiles.entry(color).or_insert(0) += count;
            }
            for (color, count) in tiles_from_f1 {
                *state.center.tiles.entry(color).or_insert(0) += count;
            }
            
            // Empty the factories
            state.factories[0] = HashMap::new();
            state.factories[1] = HashMap::new();
            
            state
        }
        
        /// Create nearly complete round (only center tiles left)
        fn create_nearly_complete_round() -> State {
            let mut state = State::new_test_state();
            // Empty all factories
            for factory in &mut state.factories {
                *factory = HashMap::new();
            }
            // Leave only 2 tiles in center
            state.center.tiles.clear();
            state.center.tiles.insert(TileColor::Blue, 2);
            state.center.has_first_player_token = true;
            
            // Rest in bag
            state.bag.insert(TileColor::Blue, 18);
            state.bag.insert(TileColor::Yellow, 20);
            state.bag.insert(TileColor::Red, 20);
            state.bag.insert(TileColor::Black, 20);
            state.bag.insert(TileColor::White, 20);
            
            state
        }

    /// Create fully populated test state for conservation testing
    fn create_fully_populated_test_state() -> State {
            let mut state = State::new_test_state();
            
            // Distribute tiles across all locations to total 100
            // Bag: 50 tiles
            state.bag.insert(TileColor::Blue, 10);
            state.bag.insert(TileColor::Yellow, 10);
            state.bag.insert(TileColor::Red, 10);
            state.bag.insert(TileColor::Black, 10);
            state.bag.insert(TileColor::White, 10);
            
            // Factories: 20 tiles (4 per factory)
            for i in 0..5 {
                state.factories[i].insert(TileColor::Blue, 2);
                state.factories[i].insert(TileColor::Red, 2);
            }
            
            // Center: 10 tiles
            state.center.tiles.insert(TileColor::Yellow, 5);
            state.center.tiles.insert(TileColor::Black, 5);
            
            // Player 0: 10 tiles
            state.players[0].pattern_lines[0].color = Some(TileColor::Blue);
            state.players[0].pattern_lines[0].count_filled = 1;
            state.players[0].pattern_lines[1].color = Some(TileColor::Red);
            state.players[0].pattern_lines[1].count_filled = 2;
            state.players[0].pattern_lines[2].color = Some(TileColor::Yellow);
            state.players[0].pattern_lines[2].count_filled = 3;
            state.players[0].floor_line.tiles.push(TileColor::Black);
            state.players[0].floor_line.tiles.push(TileColor::White);
            state.players[0].floor_line.tiles.push(TileColor::Blue);
            state.players[0].wall[3][0] = true; // 1 tile on wall
            
            // Player 1: 10 tiles
            state.players[1].pattern_lines[3].color = Some(TileColor::White);
            state.players[1].pattern_lines[3].count_filled = 4;
            state.players[1].pattern_lines[4].color = Some(TileColor::Black);
            state.players[1].pattern_lines[4].count_filled = 3;
            state.players[1].floor_line.tiles.push(TileColor::Red);
            state.players[1].floor_line.tiles.push(TileColor::Yellow);
            state.players[1].wall[0][0] = true; // 1 tile on wall
            
            state
    }

    // =====================================================================
    // Rollout Tests (Sprint 5A)
    // =====================================================================

    mod rollout_tests {
        use super::*;
        use crate::rules::{simulate_rollout, RolloutConfig, RolloutError, PolicyMix};

        #[test]
        fn test_rollout_completes_from_round_start() {
            let state = create_start_of_round_state();
            let config = RolloutConfig {
                active_player_policy: PolicyMix::AllGreedy,
                opponent_policy: PolicyMix::AllGreedy,
                seed: 12345,
                max_actions: 100,
            };
            
            let result = simulate_rollout(&state, &config).unwrap();
            
            // Round should complete
            assert!(result.completed_normally);
            
            // Should have taken some actions
            assert!(result.actions_simulated > 0);
            assert!(result.actions_simulated < 30); // Typical round is 10-20 actions
            
            // Scores should be non-negative
            assert!(result.player_0_score >= 0);
            assert!(result.player_1_score >= 0);
        }

        #[test]
        fn test_rollout_completes_from_mid_round() {
            let state = create_mid_round_state();
            let config = RolloutConfig {
                active_player_policy: PolicyMix::AllRandom,
                opponent_policy: PolicyMix::AllRandom,
                seed: 67890,
                max_actions: 100,
            };
            
            let result = simulate_rollout(&state, &config).unwrap();
            
            // Should complete with fewer actions than from start
            assert!(result.completed_normally);
            assert!(result.actions_simulated > 0);
            assert!(result.actions_simulated < 15);
        }

        #[test]
        fn test_deterministic_rollouts() {
            let state = create_nearly_complete_round();
            let config = RolloutConfig {
                active_player_policy: PolicyMix::AllRandom,
                opponent_policy: PolicyMix::AllRandom,
                seed: 42,
                max_actions: 100,
            };
            
            // Run rollout twice with same seed
            let result1 = simulate_rollout(&state, &config).unwrap();
            let result2 = simulate_rollout(&state, &config).unwrap();
            
            // Core results should be identical for the drafting simulation
            assert_eq!(result1.actions_simulated, result2.actions_simulated,
                "Same seed should produce same number of actions");
            assert_eq!(result1.player_0_score, result2.player_0_score,
                "Same seed should produce same P0 score");
            assert_eq!(result1.player_1_score, result2.player_1_score,
                "Same seed should produce same P1 score");
            assert_eq!(result1.completed_normally, result2.completed_normally);
            
            // Verify walls are identical (deterministic scoring)
            for player_idx in 0..2 {
                let p1 = &result1.final_state.players[player_idx];
                let p2 = &result2.final_state.players[player_idx];
                
                assert_eq!(p1.wall, p2.wall, 
                    "Player {} walls should be identical", player_idx);
            }
            
            // Note: Factory refill at end-of-round uses thread_rng(), so refilled
            // factories may differ. This is acceptable - the core drafting simulation
            // is deterministic, which is what matters for move evaluation.
        }

        #[test]
        fn test_different_seeds_different_results() {
            let state = create_start_of_round_state();
            let config1 = RolloutConfig {
                active_player_policy: PolicyMix::AllRandom,
                opponent_policy: PolicyMix::AllRandom,
                seed: 111,
                max_actions: 100,
            };
            let config2 = RolloutConfig {
                seed: 222,
                ..config1.clone()
            };
            
            let result1 = simulate_rollout(&state, &config1).unwrap();
            let result2 = simulate_rollout(&state, &config2).unwrap();
            
            // Results should differ (with very high probability)
            assert_ne!(result1.actions_simulated, result2.actions_simulated);
        }

        #[test]
        fn test_tile_conservation_throughout_rollout() {
            let state = create_fully_populated_test_state();
            
            // Verify initial state has 100 tiles
            assert_eq!(count_total_tiles(&state), 100);
            
            let config = RolloutConfig {
                active_player_policy: PolicyMix::AllGreedy,
                opponent_policy: PolicyMix::AllGreedy,
                seed: 999,
                max_actions: 100,
            };
            
            let result = simulate_rollout(&state, &config).unwrap();
            
            // Verify final state still has 100 tiles
            assert_eq!(count_total_tiles(&result.final_state), 100);
        }

        #[test]
        fn test_max_actions_exceeded() {
            let state = create_start_of_round_state();
            let config = RolloutConfig {
                active_player_policy: PolicyMix::AllRandom,
                opponent_policy: PolicyMix::AllRandom,
                seed: 123,
                max_actions: 3, // Artificially low limit
            };
            
            let result = simulate_rollout(&state, &config);
            
            // Should hit max actions before completing
            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), RolloutError::MaxActionsExceeded);
        }

        #[test]
        fn test_greedy_vs_random_policies() {
            let state = create_start_of_round_state();
            
            // Test with all greedy
            let config_greedy = RolloutConfig {
                active_player_policy: PolicyMix::AllGreedy,
                opponent_policy: PolicyMix::AllGreedy,
                seed: 555,
                max_actions: 100,
            };
            let result_greedy = simulate_rollout(&state, &config_greedy).unwrap();
            assert!(result_greedy.completed_normally);
            
            // Test with all random
            let config_random = RolloutConfig {
                active_player_policy: PolicyMix::AllRandom,
                opponent_policy: PolicyMix::AllRandom,
                seed: 555,
                max_actions: 100,
            };
            let result_random = simulate_rollout(&state, &config_random).unwrap();
            assert!(result_random.completed_normally);
            
            // Test with mixed policy
            let config_mixed = RolloutConfig {
                active_player_policy: PolicyMix::Mixed { greedy_ratio: 0.7 },
                opponent_policy: PolicyMix::Mixed { greedy_ratio: 0.7 },
                seed: 555,
                max_actions: 100,
            };
            let result_mixed = simulate_rollout(&state, &config_mixed).unwrap();
            assert!(result_mixed.completed_normally);
        }

        #[test]
        fn test_end_of_round_resolution_applied() {
            let mut state = create_mid_round_state();
            
            // Set up complete pattern line (accounting for tile conservation)
            // Remove 1 tile from bag to add to pattern line
            if let Some(count) = state.bag.get_mut(&TileColor::Blue) {
                *count -= 1;
            }
            state.players[0].pattern_lines[0] = PatternLine {
                capacity: 1,
                color: Some(TileColor::Blue),
                count_filled: 1,
            };
            
            let config = RolloutConfig {
                active_player_policy: PolicyMix::AllGreedy,
                opponent_policy: PolicyMix::AllGreedy,
                seed: 777,
                max_actions: 100,
            };
            
            let result = simulate_rollout(&state, &config).unwrap();
            
            // After resolution:
            // 1. Pattern line should be cleared
            assert_eq!(result.final_state.players[0].pattern_lines[0].count_filled, 0);
            assert_eq!(result.final_state.players[0].pattern_lines[0].color, None);
            
            // 2. Wall should have tile placed
            assert!(result.final_state.players[0].wall[0][0]); // Blue at row 0, col 0
            
            // 3. Score should be updated (at least 1 point)
            assert!(result.final_state.players[0].score > 0);
        }

        #[test]
        fn test_rollout_from_nearly_complete_round() {
            let state = create_nearly_complete_round();
            
            let config = RolloutConfig {
                active_player_policy: PolicyMix::AllGreedy,
                opponent_policy: PolicyMix::AllGreedy,
                seed: 888,
                max_actions: 100,
            };
            
            let result = simulate_rollout(&state, &config).unwrap();
            
            // Should complete with very few actions
            assert!(result.actions_simulated <= 2);
            assert!(result.completed_normally);
        }
    }

    // =====================================================================
    // Evaluator Tests (Sprint 5B)
    // =====================================================================

    mod evaluator_tests {
        use super::*;
        use crate::rules::{
            evaluate_best_move, grade_user_action, EvaluatorParams, RolloutPolicyConfig
        };
        use std::time::Instant;

        #[test]
        fn test_evaluation_within_time_budget() {
            let state = create_start_of_round_state();
            let params = EvaluatorParams {
                time_budget_ms: 500,
                rollouts_per_action: 5,
                evaluator_seed: 12345,
                shortlist_size: 10,
                rollout_config: RolloutPolicyConfig::default(),
            };
            
            let start = Instant::now();
            let result = evaluate_best_move(&state, 0, &params).unwrap();
            let elapsed = start.elapsed().as_millis();
            
            // Should complete within reasonable time
            assert!(elapsed < 700, "Elapsed time {} exceeds 700ms", elapsed);
            
            // Should have evaluated at least one action
            assert!(result.metadata.candidates_evaluated > 0);
        }

        #[test]
        fn test_action_shortlisting() {
            let state = create_start_of_round_state();
            let params = EvaluatorParams {
                time_budget_ms: 1000,
                rollouts_per_action: 5,
                evaluator_seed: 67890,
                shortlist_size: 10,
                rollout_config: RolloutPolicyConfig::default(),
            };
            
            let result = evaluate_best_move(&state, 0, &params).unwrap();
            
            // Should have many legal actions initially
            assert!(result.metadata.total_legal_actions > 10);
            
            // Should evaluate only shortlist size
            assert!(result.metadata.candidates_evaluated <= 10);
        }

        #[test]
        fn test_deterministic_evaluation() {
            let state = create_start_of_round_state();
            let params = EvaluatorParams {
                time_budget_ms: 250,
                rollouts_per_action: 10,
                evaluator_seed: 42,
                shortlist_size: 0, // Disable shortlisting for full determinism
                rollout_config: RolloutPolicyConfig::default(),
            };
            
            let result1 = evaluate_best_move(&state, 0, &params).unwrap();
            let result2 = evaluate_best_move(&state, 0, &params).unwrap();
            
            // Same seed should produce identical results
            assert_eq!(
                serde_json::to_string(&result1.best_action).unwrap(),
                serde_json::to_string(&result2.best_action).unwrap(),
                "Best actions should be identical"
            );
            assert!(
                (result1.best_action_ev - result2.best_action_ev).abs() < 0.001,
                "EVs should be nearly identical"
            );
        }

        #[test]
        fn test_different_seeds_different_evaluations() {
            let state = create_start_of_round_state();
            let params1 = EvaluatorParams {
                time_budget_ms: 250,
                rollouts_per_action: 10,
                evaluator_seed: 111,
                shortlist_size: 20,
                rollout_config: RolloutPolicyConfig::default(),
            };
            let params2 = EvaluatorParams {
                evaluator_seed: 222,
                ..params1.clone()
            };
            
            let result1 = evaluate_best_move(&state, 0, &params1).unwrap();
            let result2 = evaluate_best_move(&state, 0, &params2).unwrap();
            
            // Different seeds should likely produce different EVs (probabilistic)
            // Note: Actions might be same if clearly dominant
            assert_ne!(
                (result1.best_action_ev * 100.0) as i32,
                (result2.best_action_ev * 100.0) as i32,
                "Different seeds should produce different EVs"
            );
        }

        #[test]
        fn test_grade_user_action() {
            let state = create_start_of_round_state();
            let params = EvaluatorParams {
                time_budget_ms: 250,
                rollouts_per_action: 10,
                evaluator_seed: 555,
                shortlist_size: 20,
                rollout_config: RolloutPolicyConfig::default(),
            };
            
            // Evaluate best move
            let best_result = evaluate_best_move(&state, 0, &params).unwrap();
            
            // Get a different legal action (not the best)
            let legal_actions = list_legal_actions(&state, 0);
            let user_action = legal_actions.iter()
                .find(|a| *a != &best_result.best_action)
                .expect("Should have at least 2 legal actions")
                .clone();
            
            // Grade user action
            let graded = grade_user_action(&state, 0, &user_action, &params, &best_result).unwrap();
            
            // Should have user EV and delta
            assert!(graded.user_action_ev.is_some());
            assert!(graded.delta_ev.is_some());
            
            // Delta should be negative or zero (user action not better than best)
            let delta = graded.delta_ev.unwrap();
            assert!(delta <= 0.0);
        }

        #[test]
        fn test_best_action_is_legal() {
            let state = create_start_of_round_state();
            let params = EvaluatorParams {
                time_budget_ms: 250,
                rollouts_per_action: 10,
                evaluator_seed: 777,
                shortlist_size: 20,
                rollout_config: RolloutPolicyConfig::default(),
            };
            
            let result = evaluate_best_move(&state, 0, &params).unwrap();
            let legal_actions = list_legal_actions(&state, 0);
            
            // Best action must be in legal action set
            assert!(legal_actions.contains(&result.best_action));
        }

        #[test]
        fn test_no_shortlisting_when_few_actions() {
            let state = create_nearly_complete_round();
            let params = EvaluatorParams {
                time_budget_ms: 250,
                rollouts_per_action: 10,
                evaluator_seed: 888,
                shortlist_size: 20, // Larger than available
                rollout_config: RolloutPolicyConfig::default(),
            };
            
            let result = evaluate_best_move(&state, 0, &params).unwrap();
            
            // Should evaluate all actions when fewer than shortlist size
            assert_eq!(
                result.metadata.candidates_evaluated, 
                result.metadata.total_legal_actions,
                "Should evaluate all actions when less than shortlist size"
            );
        }

        #[test]
        fn test_time_budget_cutoff() {
            let state = create_start_of_round_state();
            let params = EvaluatorParams {
                time_budget_ms: 50, // Very short budget
                rollouts_per_action: 50, // Many rollouts per action
                evaluator_seed: 999,
                shortlist_size: 20,
                rollout_config: RolloutPolicyConfig::default(),
            };
            
            let start = Instant::now();
            let result = evaluate_best_move(&state, 0, &params).unwrap();
            let elapsed = start.elapsed().as_millis();
            
            // Should respect time budget (with some tolerance)
            assert!(elapsed < 200, "Time budget not respected: {}ms", elapsed);
            
            // Should still return a valid best action
            assert!(result.best_action_ev.is_finite());
            
            // Likely didn't evaluate all shortlist candidates
            assert!(result.metadata.candidates_evaluated < 20);
        }
    }

    // =====================================================================
    // Feedback Tests (Sprint 5C)
    // =====================================================================

    mod feedback_tests {
        use super::*;
        use crate::rules::{
            compute_grade, generate_feedback_bullets, ActionFeatures, Grade,
            count_pattern_lines_completed, calculate_floor_penalty_for_player
        };

        #[test]
        fn test_grade_computation() {
            assert_eq!(compute_grade(0.1), Grade::Excellent);
            assert_eq!(compute_grade(0.25), Grade::Excellent);
            assert_eq!(compute_grade(0.5), Grade::Good);
            assert_eq!(compute_grade(1.0), Grade::Good);
            assert_eq!(compute_grade(1.5), Grade::Okay);
            assert_eq!(compute_grade(2.5), Grade::Okay);
            assert_eq!(compute_grade(3.0), Grade::Miss);
            
            // Negative deltas (absolute value used)
            assert_eq!(compute_grade(-0.1), Grade::Excellent);
            assert_eq!(compute_grade(-2.0), Grade::Okay);
        }

        #[test]
        fn test_feedback_generation_floor_penalty() {
            let user_features = ActionFeatures {
                expected_floor_penalty: -3.0,
                expected_completions: 0.5,
                expected_adjacency_points: 2.0,
                expected_tiles_to_floor: 2.0,
                takes_first_player_token: false,
                tiles_acquired: 3,
            };
            
            let best_features = ActionFeatures {
                expected_floor_penalty: -1.0,
                expected_completions: 0.5,
                expected_adjacency_points: 2.0,
                expected_tiles_to_floor: 2.0,
                takes_first_player_token: false,
                tiles_acquired: 4,
            };
            
            let feedback = generate_feedback_bullets(&user_features, &best_features);
            
            // Should generate floor penalty feedback
            assert!(!feedback.is_empty());
            assert!(feedback.iter().any(|b| matches!(b.category, crate::rules::FeedbackCategory::FloorPenalty)));
        }

        #[test]
        fn test_feedback_generation_completions() {
            let user_features = ActionFeatures {
                expected_floor_penalty: -1.0,
                expected_completions: 0.2,
                expected_adjacency_points: 2.0,
                expected_tiles_to_floor: 1.0,
                takes_first_player_token: false,
                tiles_acquired: 3,
            };
            
            let best_features = ActionFeatures {
                expected_floor_penalty: -1.0,
                expected_completions: 0.8,
                expected_adjacency_points: 2.0,
                expected_tiles_to_floor: 1.0,
                takes_first_player_token: false,
                tiles_acquired: 4,
            };
            
            let feedback = generate_feedback_bullets(&user_features, &best_features);
            
            // Should generate completion feedback
            assert!(!feedback.is_empty());
            assert!(feedback.iter().any(|b| matches!(b.category, crate::rules::FeedbackCategory::LineCompletion)));
        }

        #[test]
        fn test_feedback_sorting_and_limit() {
            let user_features = ActionFeatures {
                expected_floor_penalty: -5.0,
                expected_completions: 0.1,
                expected_adjacency_points: 1.0,
                expected_tiles_to_floor: 3.0,
                takes_first_player_token: true,
                tiles_acquired: 2,
            };
            
            let best_features = ActionFeatures {
                expected_floor_penalty: -1.0,
                expected_completions: 0.9,
                expected_adjacency_points: 4.0,
                expected_tiles_to_floor: 0.5,
                takes_first_player_token: false,
                tiles_acquired: 4,
            };
            
            let feedback = generate_feedback_bullets(&user_features, &best_features);
            
            // Should not exceed 3 bullets
            assert!(feedback.len() <= 3);
            
            // Should be sorted by importance (delta descending)
            for i in 0..feedback.len().saturating_sub(1) {
                assert!(feedback[i].delta >= feedback[i + 1].delta,
                    "Feedback not sorted: bullet {} has delta {}, bullet {} has delta {}",
                    i, feedback[i].delta, i + 1, feedback[i + 1].delta);
            }
        }

        #[test]
        fn test_count_pattern_lines_completed() {
            let mut before = crate::model::PlayerBoard::new();
            let mut after = crate::model::PlayerBoard::new();
            
            // Set up a completed line
            before.pattern_lines[0].capacity = 1;
            before.pattern_lines[0].color = Some(TileColor::Blue);
            before.pattern_lines[0].count_filled = 1;
            
            // After resolution, line is cleared
            after.pattern_lines[0].capacity = 1;
            after.pattern_lines[0].color = None;
            after.pattern_lines[0].count_filled = 0;
            after.wall[0][0] = true; // Tile placed on wall
            
            let completions = count_pattern_lines_completed(&before, &after);
            assert_eq!(completions, 1);
        }

        #[test]
        fn test_calculate_floor_penalty() {
            let mut player = crate::model::PlayerBoard::new();
            
            // Empty floor
            let penalty = calculate_floor_penalty_for_player(&player);
            assert_eq!(penalty, 0);
            
            // 3 tiles
            player.floor_line.tiles.push(TileColor::Blue);
            player.floor_line.tiles.push(TileColor::Red);
            player.floor_line.tiles.push(TileColor::Yellow);
            let penalty = calculate_floor_penalty_for_player(&player);
            assert_eq!(penalty, -4); // -1, -1, -2
            
            // With first player token
            player.floor_line.has_first_player_token = true;
            let penalty = calculate_floor_penalty_for_player(&player);
            assert_eq!(penalty, -6); // -1, -1, -2, -2
        }

        #[test]
        fn test_evaluation_includes_features() {
            let state = create_start_of_round_state();
            let params = crate::rules::EvaluatorParams {
                time_budget_ms: 250,
                rollouts_per_action: 10,
                evaluator_seed: 12345,
                shortlist_size: 20,
                rollout_config: crate::rules::RolloutPolicyConfig::default(),
            };
            
            let result = crate::rules::evaluate_best_move(&state, 0, &params).unwrap();
            
            // Should have best_features populated
            assert!(result.best_features.tiles_acquired > 0);
            assert!(result.best_features.expected_floor_penalty <= 0.0);
            assert!(result.best_features.expected_completions >= 0.0);
        }

        #[test]
        fn test_ev_consistency_when_user_picks_evaluated_action() {
            let state = create_start_of_round_state();
            let params = crate::rules::EvaluatorParams {
                time_budget_ms: 250,
                rollouts_per_action: 10,
                evaluator_seed: 12345,
                shortlist_size: 20,
                rollout_config: crate::rules::RolloutPolicyConfig::default(),
            };
            
            // Evaluate best move
            let best_result = crate::rules::evaluate_best_move(&state, 0, &params).unwrap();
            
            // User picks the best action
            let user_action = best_result.best_action.clone();
            
            // Grade user action
            let graded_result = crate::rules::grade_user_action(
                &state, 
                0, 
                &user_action, 
                &params, 
                &best_result
            ).unwrap();
            
            // User EV should equal best EV (same action)
            assert_eq!(graded_result.user_action_ev, Some(best_result.best_action_ev));
            
            // Delta should be 0.0
            assert_eq!(graded_result.delta_ev, Some(0.0));
            
            // Grade should be Excellent
            assert_eq!(graded_result.grade, Some(crate::rules::Grade::Excellent));
        }

        #[test]
        fn test_ev_consistency_for_candidate_action() {
            let state = create_start_of_round_state();
            let params = crate::rules::EvaluatorParams {
                time_budget_ms: 250,
                rollouts_per_action: 10,
                evaluator_seed: 12345,
                shortlist_size: 20,
                rollout_config: crate::rules::RolloutPolicyConfig::default(),
            };
            
            // Evaluate best move
            let best_result = crate::rules::evaluate_best_move(&state, 0, &params).unwrap();
            
            // Pick a different candidate (not the best)
            let candidates = best_result.candidates.as_ref().unwrap();
            assert!(candidates.len() > 1, "Need multiple candidates for this test");
            
            let user_action = candidates[1].action.clone();
            let expected_ev = candidates[1].ev;
            
            // Grade user action
            let graded_result = crate::rules::grade_user_action(
                &state, 
                0, 
                &user_action, 
                &params, 
                &best_result
            ).unwrap();
            
            // User EV should match the EV from candidates list (consistency)
            assert_eq!(graded_result.user_action_ev, Some(expected_ev));
            
            // Delta should be consistent with that EV
            let expected_delta = expected_ev - best_result.best_action_ev;
            assert_eq!(graded_result.delta_ev, Some(expected_delta));
        }
    }
}
