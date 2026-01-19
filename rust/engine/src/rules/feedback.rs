use crate::model::{PlayerBoard, DraftAction, ActionSource, State};
use crate::rules::constants::FLOOR_PENALTIES;
use serde::{Deserialize, Serialize};
use std::cmp::min;

/// Statistics collected for an action across rollouts
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ActionFeatures {
    /// Expected floor penalty (average across rollouts)
    pub expected_floor_penalty: f64,
    /// Expected pattern lines completed this round
    pub expected_completions: f64,
    /// Expected adjacency points scored this round
    pub expected_adjacency_points: f64,
    /// Expected tiles sent to floor (waste)
    pub expected_tiles_to_floor: f64,
    /// Whether action takes first player token
    pub takes_first_player_token: bool,
    /// Number of tiles acquired in the action
    pub tiles_acquired: u8,
}

impl Default for ActionFeatures {
    fn default() -> Self {
        Self {
            expected_floor_penalty: 0.0,
            expected_completions: 0.0,
            expected_adjacency_points: 0.0,
            expected_tiles_to_floor: 0.0,
            takes_first_player_token: false,
            tiles_acquired: 0,
        }
    }
}

/// Category of feedback
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FeedbackCategory {
    FloorPenalty,
    LineCompletion,
    WastedTiles,
    Adjacency,
    FirstPlayerToken,
}

/// Human-readable feedback bullet
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FeedbackBullet {
    /// Category of feedback
    pub category: FeedbackCategory,
    /// Human-readable explanation text
    pub text: String,
    /// Numeric delta (for sorting by importance)
    pub delta: f64,
}

/// Grade for user's move
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Grade {
    Excellent,
    Good,
    Okay,
    Miss,
}

/// Thresholds for grade computation
pub struct GradeThresholds {
    pub excellent_max: f64,
    pub good_max: f64,
    pub okay_max: f64,
}

pub const GRADE_THRESHOLDS: GradeThresholds = GradeThresholds {
    excellent_max: 0.25,
    good_max: 1.0,
    okay_max: 2.5,
};

/// Compute grade from delta EV
pub fn compute_grade(delta_ev: f64) -> Grade {
    let abs_delta = delta_ev.abs();
    
    if abs_delta <= GRADE_THRESHOLDS.excellent_max {
        Grade::Excellent
    } else if abs_delta <= GRADE_THRESHOLDS.good_max {
        Grade::Good
    } else if abs_delta <= GRADE_THRESHOLDS.okay_max {
        Grade::Okay
    } else {
        Grade::Miss
    }
}

/// Count pattern lines that were completed in this round
pub fn count_pattern_lines_completed(before: &PlayerBoard, after: &PlayerBoard) -> u8 {
    let mut completed = 0;
    for row in 0..5 {
        let before_line = &before.pattern_lines[row];
        let after_line = &after.pattern_lines[row];
        
        // Line was complete before resolution, now empty
        if before_line.count_filled == before_line.capacity 
            && after_line.count_filled == 0 {
            completed += 1;
        }
    }
    completed
}

/// Calculate floor penalty for a player's floor line
pub fn calculate_floor_penalty_for_player(player: &PlayerBoard) -> i32 {
    let floor_count = player.floor_line.tiles.len();
    let has_token = player.floor_line.has_first_player_token;
    
    let total_slots = if has_token {
        min(floor_count + 1, 7)
    } else {
        min(floor_count, 7)
    };
    
    let mut penalty = 0;
    for i in 0..total_slots {
        penalty += FLOOR_PENALTIES[i];
    }
    penalty
}

/// Count tiles in an action source
pub fn count_tiles_in_action(state: &State, action: &DraftAction) -> u8 {
    match &action.source {
        ActionSource::Factory(idx) => {
            state.factories.get(*idx)
                .and_then(|f| f.get(&action.color))
                .copied()
                .unwrap_or(0)
        }
        ActionSource::Center => {
            state.center.tiles.get(&action.color).copied().unwrap_or(0)
        }
    }
}

/// Generate 1-3 feedback bullets comparing user to best action
pub fn generate_feedback_bullets(
    user_features: &ActionFeatures,
    best_features: &ActionFeatures,
) -> Vec<FeedbackBullet> {
    let mut bullets = Vec::new();
    
    // 1. Floor penalty difference
    let floor_delta = user_features.expected_floor_penalty - best_features.expected_floor_penalty;
    if floor_delta.abs() > 0.5 {
        let text = if floor_delta > 0.0 {
            format!(
                "Best move reduces floor penalty by ~{:.1} points more than your move.",
                floor_delta
            )
        } else {
            format!(
                "Your move reduces floor penalty by ~{:.1} points compared to the best move.",
                floor_delta.abs()
            )
        };
        bullets.push(FeedbackBullet {
            category: FeedbackCategory::FloorPenalty,
            text,
            delta: floor_delta.abs(),
        });
    }
    
    // 2. Line completion difference
    let completion_delta = best_features.expected_completions - user_features.expected_completions;
    if completion_delta > 0.1 {
        let text = format!(
            "Best move is more likely to complete a pattern line this round ({:.0}% vs {:.0}%).",
            best_features.expected_completions * 100.0,
            user_features.expected_completions * 100.0
        );
        bullets.push(FeedbackBullet {
            category: FeedbackCategory::LineCompletion,
            text,
            delta: completion_delta,
        });
    }
    
    // 3. Wasted tiles difference
    let waste_delta = user_features.expected_tiles_to_floor - best_features.expected_tiles_to_floor;
    if waste_delta > 0.5 {
        let text = format!(
            "Your move sends ~{:.1} more tiles to the floor than the best move.",
            waste_delta
        );
        bullets.push(FeedbackBullet {
            category: FeedbackCategory::WastedTiles,
            text,
            delta: waste_delta,
        });
    }
    
    // 4. Adjacency difference
    let adjacency_delta = best_features.expected_adjacency_points - user_features.expected_adjacency_points;
    if adjacency_delta > 0.5 {
        let text = format!(
            "Best move creates better wall adjacency, scoring ~{:.1} more points.",
            adjacency_delta
        );
        bullets.push(FeedbackBullet {
            category: FeedbackCategory::Adjacency,
            text,
            delta: adjacency_delta,
        });
    }
    
    // 5. First player token consideration
    if user_features.takes_first_player_token != best_features.takes_first_player_token {
        let text = if user_features.takes_first_player_token {
            "Your move takes the first player token, which has a tempo cost.".to_string()
        } else {
            "Best move takes the first player token, trading tempo for tile value.".to_string()
        };
        bullets.push(FeedbackBullet {
            category: FeedbackCategory::FirstPlayerToken,
            text,
            delta: 1.0,
        });
    }
    
    // Sort by importance and take top 3
    bullets.sort_by(|a, b| b.delta.partial_cmp(&a.delta).unwrap_or(std::cmp::Ordering::Equal));
    bullets.truncate(3);
    
    bullets
}
