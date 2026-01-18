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
}
