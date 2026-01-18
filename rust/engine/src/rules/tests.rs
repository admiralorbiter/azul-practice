#[cfg(test)]
mod tests {
    use crate::{State, TileColor, PatternLine, ActionSource, Destination, DraftAction};
    use crate::rules::{list_legal_actions, get_wall_column_for_color, apply_action, check_tile_conservation};

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
}
