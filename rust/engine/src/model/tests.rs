#[cfg(test)]
mod tests {
    use crate::model::*;

    #[test]
    fn test_tile_color_serialization() {
        let color = TileColor::Blue;
        let json = serde_json::to_string(&color).unwrap();
        assert_eq!(json, r#""Blue""#);
        
        let restored: TileColor = serde_json::from_str(&json).unwrap();
        assert_eq!(restored, color);
    }

    #[test]
    fn test_all_tile_colors_serialize() {
        let colors = vec![
            TileColor::Blue,
            TileColor::Yellow,
            TileColor::Red,
            TileColor::Black,
            TileColor::White,
        ];
        
        for color in colors {
            let json = serde_json::to_string(&color).unwrap();
            let restored: TileColor = serde_json::from_str(&json).unwrap();
            assert_eq!(restored, color);
        }
    }

    #[test]
    fn test_draft_phase_serialization() {
        let phase = DraftPhase::Mid;
        let json = serde_json::to_string(&phase).unwrap();
        assert_eq!(json, r#""MID""#);
        
        let restored: DraftPhase = serde_json::from_str(&json).unwrap();
        assert_eq!(restored, phase);
    }

    #[test]
    fn test_all_draft_phases_serialize() {
        // Test Early
        let phase = DraftPhase::Early;
        let json = serde_json::to_string(&phase).unwrap();
        assert_eq!(json, r#""EARLY""#);
        
        // Test Mid
        let phase = DraftPhase::Mid;
        let json = serde_json::to_string(&phase).unwrap();
        assert_eq!(json, r#""MID""#);
        
        // Test Late
        let phase = DraftPhase::Late;
        let json = serde_json::to_string(&phase).unwrap();
        assert_eq!(json, r#""LATE""#);
    }

    #[test]
    fn test_pattern_line_roundtrip() {
        let pattern_line = PatternLine {
            capacity: 3,
            color: Some(TileColor::Red),
            count_filled: 2,
        };
        
        let json = serde_json::to_string(&pattern_line).unwrap();
        let restored: PatternLine = serde_json::from_str(&json).unwrap();
        
        assert_eq!(restored, pattern_line);
    }

    #[test]
    fn test_pattern_line_empty() {
        let pattern_line = PatternLine {
            capacity: 5,
            color: None,
            count_filled: 0,
        };
        
        let json = serde_json::to_string(&pattern_line).unwrap();
        let restored: PatternLine = serde_json::from_str(&json).unwrap();
        
        assert_eq!(restored, pattern_line);
        assert_eq!(restored.color, None);
    }

    #[test]
    fn test_pattern_line_new() {
        for row in 0..5 {
            let pl = PatternLine::new(row);
            assert_eq!(pl.capacity, (row + 1) as u8);
            assert_eq!(pl.color, None);
            assert_eq!(pl.count_filled, 0);
        }
    }

    #[test]
    fn test_action_source_factory_serialization() {
        let source = ActionSource::Factory(2);
        let json = serde_json::to_string(&source).unwrap();
        assert_eq!(json, r#"{"Factory":2}"#);
        
        let restored: ActionSource = serde_json::from_str(&json).unwrap();
        assert_eq!(restored, source);
    }

    #[test]
    fn test_action_source_center_serialization() {
        let source = ActionSource::Center;
        let json = serde_json::to_string(&source).unwrap();
        assert_eq!(json, r#""Center""#);
        
        let restored: ActionSource = serde_json::from_str(&json).unwrap();
        assert_eq!(restored, source);
    }

    #[test]
    fn test_destination_pattern_line_serialization() {
        let dest = Destination::PatternLine(3);
        let json = serde_json::to_string(&dest).unwrap();
        assert_eq!(json, r#"{"PatternLine":3}"#);
        
        let restored: Destination = serde_json::from_str(&json).unwrap();
        assert_eq!(restored, dest);
    }

    #[test]
    fn test_destination_floor_serialization() {
        let dest = Destination::Floor;
        let json = serde_json::to_string(&dest).unwrap();
        assert_eq!(json, r#""Floor""#);
        
        let restored: Destination = serde_json::from_str(&json).unwrap();
        assert_eq!(restored, dest);
    }

    #[test]
    fn test_draft_action_roundtrip() {
        let action = DraftAction {
            source: ActionSource::Factory(0),
            color: TileColor::Blue,
            destination: Destination::PatternLine(2),
        };
        
        let json = serde_json::to_string(&action).unwrap();
        let restored: DraftAction = serde_json::from_str(&json).unwrap();
        
        assert_eq!(restored, action);
    }

    #[test]
    fn test_draft_action_to_floor() {
        let action = DraftAction {
            source: ActionSource::Center,
            color: TileColor::Red,
            destination: Destination::Floor,
        };
        
        let json = serde_json::to_string(&action).unwrap();
        let restored: DraftAction = serde_json::from_str(&json).unwrap();
        
        assert_eq!(restored, action);
    }

    #[test]
    fn test_player_board_new() {
        let board = PlayerBoard::new();
        assert_eq!(board.score, 0);
        assert_eq!(board.pattern_lines.len(), 5);
        assert_eq!(board.wall.len(), 5);
        assert_eq!(board.floor_line.tiles.len(), 0);
        assert!(!board.floor_line.has_first_player_token);
    }

    #[test]
    fn test_state_roundtrip() {
        let mut state = State::new_test_state();
        
        // Add some data to make it realistic
        state.bag.insert(TileColor::Blue, 10);
        state.bag.insert(TileColor::Red, 8);
        state.factories[0].insert(TileColor::Yellow, 2);
        state.center.tiles.insert(TileColor::Black, 3);
        
        state.players[0].score = 15;
        state.players[0].pattern_lines[2].color = Some(TileColor::Red);
        state.players[0].pattern_lines[2].count_filled = 2;
        state.players[0].wall[0][0] = true;
        
        let json = serde_json::to_string_pretty(&state).unwrap();
        let restored: State = serde_json::from_str(&json).unwrap();
        
        assert_eq!(restored, state);
    }

    #[test]
    fn test_state_without_scenario_seed() {
        let state = State::new_test_state();
        let json = serde_json::to_string(&state).unwrap();
        
        // Should not contain scenario_seed field when None
        assert!(!json.contains("scenario_seed"));
        
        let restored: State = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.scenario_seed, None);
    }

    #[test]
    fn test_state_with_scenario_seed() {
        let mut state = State::new_test_state();
        state.scenario_seed = Some("test_seed_123".to_string());
        
        let json = serde_json::to_string(&state).unwrap();
        
        // Should contain scenario_seed field when Some
        assert!(json.contains("scenario_seed"));
        assert!(json.contains("test_seed_123"));
        
        let restored: State = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.scenario_seed, Some("test_seed_123".to_string()));
    }

    #[test]
    fn test_json_field_names_snake_case() {
        let state = State::new_test_state();
        let json = serde_json::to_string(&state).unwrap();
        
        // Verify snake_case fields exist
        assert!(json.contains("\"active_player_id\""));
        assert!(json.contains("\"state_version\""));
        assert!(json.contains("\"draft_phase_progress\""));
        assert!(json.contains("\"has_first_player_token\""));
        
        // Should not contain camelCase
        assert!(!json.contains("activePlayerId"));
        assert!(!json.contains("stateVersion"));
    }

    #[test]
    fn test_version_fields_present() {
        let state = State::new_test_state();
        let json_value: serde_json::Value = serde_json::to_value(&state).unwrap();
        
        assert_eq!(json_value["state_version"], 1);
        assert_eq!(json_value["ruleset_id"], "azul_v1_2p");
    }

    #[test]
    fn test_tile_multiset_serialization() {
        let mut multiset = TileMultiset::new();
        multiset.insert(TileColor::Blue, 3);
        multiset.insert(TileColor::Red, 2);
        
        let json = serde_json::to_string(&multiset).unwrap();
        let restored: TileMultiset = serde_json::from_str(&json).unwrap();
        
        assert_eq!(restored.get(&TileColor::Blue), Some(&3));
        assert_eq!(restored.get(&TileColor::Red), Some(&2));
        assert_eq!(restored.get(&TileColor::Yellow), None);
    }

    #[test]
    fn test_floor_line_serialization() {
        let floor_line = FloorLine {
            tiles: vec![TileColor::Blue, TileColor::Red, TileColor::Black],
            has_first_player_token: true,
        };
        
        let json = serde_json::to_string(&floor_line).unwrap();
        let restored: FloorLine = serde_json::from_str(&json).unwrap();
        
        assert_eq!(restored, floor_line);
        assert_eq!(restored.tiles.len(), 3);
        assert!(restored.has_first_player_token);
    }

    #[test]
    fn test_center_area_serialization() {
        let mut tiles = TileMultiset::new();
        tiles.insert(TileColor::White, 2);
        tiles.insert(TileColor::Yellow, 1);
        
        let center = CenterArea {
            tiles,
            has_first_player_token: false,
        };
        
        let json = serde_json::to_string(&center).unwrap();
        let restored: CenterArea = serde_json::from_str(&json).unwrap();
        
        assert_eq!(restored, center);
        assert_eq!(restored.tiles.get(&TileColor::White), Some(&2));
        assert!(!restored.has_first_player_token);
    }

    #[test]
    fn test_state_new_test_state() {
        let state = State::new_test_state();
        
        assert_eq!(state.state_version, 1);
        assert_eq!(state.ruleset_id, "azul_v1_2p");
        assert_eq!(state.scenario_seed, None);
        assert_eq!(state.active_player_id, 0);
        assert_eq!(state.round_number, 1);
        assert_eq!(state.draft_phase_progress, DraftPhase::Early);
        assert_eq!(state.factories.len(), 5);
        assert!(state.center.has_first_player_token);
        assert_eq!(state.players.len(), 2);
    }

    #[test]
    fn test_wall_serialization() {
        let mut wall: Wall = [[false; 5]; 5];
        wall[0][0] = true;
        wall[1][2] = true;
        wall[4][4] = true;
        
        let json = serde_json::to_string(&wall).unwrap();
        let restored: Wall = serde_json::from_str(&json).unwrap();
        
        assert_eq!(restored, wall);
        assert!(restored[0][0]);
        assert!(restored[1][2]);
        assert!(restored[4][4]);
        assert!(!restored[2][2]);
    }
}
