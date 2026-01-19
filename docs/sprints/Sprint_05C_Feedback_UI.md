# Sprint 05C — Feedback System + UI Integration

**Status:** 📋 **PLANNED**  
**Prerequisites:** Sprint 05B (Evaluator Core) complete  
**Dependencies:** Evaluation API, feature delta tracking, UI components  
**Complexity:** Medium

---

## Goal

Add rich feedback generation with feature-based explanations, implement grading system, and complete UI integration with "Think Longer" controls and results visualization.

## Outcomes

- ✓ Feature delta tracking during rollouts (floor penalty, adjacency, completion, waste)
- ✓ Template-based feedback bullet generation
- ✓ Grading system mapping delta EV to user-friendly labels
- ✓ Complete evaluation UI in PracticeScreen
- ✓ "Think Longer" time budget controls
- ✓ Results panel with grade, EV comparison, and feedback
- ✓ Best move visualization overlay
- ✓ Dev panel diagnostics for evaluation
- ✓ End-to-end practice loop functional

---

## Feature Delta Tracking

To generate meaningful feedback, track actionable statistics during rollout evaluation.

### ActionFeatures Type

Statistics collected for an action across all its rollouts.

```rust
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
```

### Collection During Rollouts

Extend evaluator to track features:

```rust
// In evaluate_best_move, for each candidate action:
let mut feature_accumulator = FeatureAccumulator::new();

for rollout in rollouts:
    result = simulate_rollout(...)
    
    // Collect features from this rollout
    floor_penalty = calculate_floor_penalty_for_player(
        &result.final_state.players[player_id]
    );
    
    completions = count_pattern_lines_completed(
        &initial_state.players[player_id],
        &result.final_state.players[player_id]
    );
    
    adjacency = result.final_state.players[player_id].score - 
                initial_score - floor_penalty;
    
    tiles_to_floor = result.final_state.players[player_id]
        .floor_line.tiles.len();
    
    feature_accumulator.add_sample(floor_penalty, completions, adjacency, tiles_to_floor);

// Compute averages
let features = feature_accumulator.finalize();
```

### Helper Functions

```rust
fn count_pattern_lines_completed(before: &PlayerBoard, after: &PlayerBoard) -> u8 {
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

fn calculate_floor_penalty_for_player(player: &PlayerBoard) -> i32 {
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
```

---

## Feedback Generation

Generate 1-3 human-readable explanation bullets comparing user action to best action.

### FeedbackBullet Type

```rust
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FeedbackCategory {
    FloorPenalty,
    LineCompletion,
    WastedTiles,
    Adjacency,
    FirstPlayerToken,
}
```

### Feedback Templates

```rust
fn generate_feedback_bullets(
    user_features: &ActionFeatures,
    best_features: &ActionFeatures,
) -> Vec<FeedbackBullet> {
    let mut bullets = Vec::new();
    
    // 1. Floor penalty difference
    let floor_delta = user_features.expected_floor_penalty - best_features.expected_floor_penalty;
    if floor_delta.abs() > 0.5 {
        let text = if floor_delta < 0.0 {
            format!(
                "Your move reduces floor penalty by ~{:.1} points compared to the best move.",
                floor_delta.abs()
            )
        } else {
            format!(
                "Best move reduces floor penalty by ~{:.1} points more than your move.",
                floor_delta
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
            delta: 1.0, // Fixed importance
        });
    }
    
    // Sort by delta (importance) and take top 3
    bullets.sort_by(|a, b| b.delta.partial_cmp(&a.delta).unwrap());
    bullets.truncate(3);
    
    bullets
}
```

### Enhanced EvaluationResult

Add feedback to result type (extends Sprint 5B):

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EvaluationResult {
    // ... existing fields from Sprint 5B ...
    
    /// Feature deltas for user action (if provided)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_features: Option<ActionFeatures>,
    
    /// Feature deltas for best action
    pub best_features: ActionFeatures,
    
    /// Feedback bullets explaining the difference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback: Option<Vec<FeedbackBullet>>,
}
```

---

## Grading System

Map delta EV to user-friendly grade labels.

### Grade Type

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Grade {
    Excellent,  // Near-optimal
    Good,       // Solid move
    Okay,       // Acceptable but suboptimal
    Miss,       // Poor choice
}
```

### Grading Thresholds

```rust
pub const GRADE_THRESHOLDS: GradeThresholds = GradeThresholds {
    excellent_max: 0.25,
    good_max: 1.0,
    okay_max: 2.5,
    // miss: > 2.5
};

#[derive(Debug, Clone, Copy)]
pub struct GradeThresholds {
    pub excellent_max: f64,
    pub good_max: f64,
    pub okay_max: f64,
}

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
```

### Grade Display Text

```rust
impl Grade {
    pub fn display_text(&self) -> &'static str {
        match self {
            Grade::Excellent => "Excellent! You found the best move.",
            Grade::Good => "Good move! Close to optimal.",
            Grade::Okay => "Okay move, but there's a better option.",
            Grade::Miss => "Missed opportunity. Review the best move.",
        }
    }
    
    pub fn color_class(&self) -> &'static str {
        match self {
            Grade::Excellent => "grade-excellent",
            Grade::Good => "grade-good",
            Grade::Okay => "grade-okay",
            Grade::Miss => "grade-miss",
        }
    }
}
```

---

## UI Components

### Component 1: ThinkLongerControl

Dropdown selector for evaluation time budget.

**Location:** `web/src/components/ThinkLongerControl.tsx`

```typescript
import React from 'react';
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
```

**Styles:** `web/src/components/ThinkLongerControl.css`

```css
.think-longer-control {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 12px;
  background: var(--surface);
  border-radius: 4px;
}

.think-longer-control label {
  font-weight: 600;
  font-size: 14px;
  color: var(--text-primary);
}

.think-longer-control select {
  padding: 6px 12px;
  border: 1px solid var(--border);
  border-radius: 4px;
  background: white;
  font-size: 14px;
  cursor: pointer;
}

.think-longer-control select:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}
```

---

### Component 2: EvaluationResult

Display evaluation results with grade, EV comparison, and feedback.

**Location:** `web/src/components/EvaluationResult.tsx`

```typescript
import React from 'react';
import { EvaluationResult as EvalResult, Grade } from '../wasm/evaluator';
import './EvaluationResult.css';

interface EvaluationResultProps {
  result: EvalResult;
  onNextScenario: () => void;
}

export function EvaluationResult({ result, onNextScenario }: EvaluationResultProps) {
  const grade = result.deltaEv !== undefined ? computeGrade(-result.deltaEv) : null;
  
  return (
    <div className="evaluation-result">
      {/* Grade Badge */}
      {grade && (
        <div className={`grade-badge ${getGradeClass(grade)}`}>
          <div className="grade-label">{grade}</div>
          <div className="grade-text">{getGradeText(grade)}</div>
        </div>
      )}
      
      {/* EV Comparison */}
      <div className="ev-comparison">
        <div className="ev-row">
          <span className="ev-label">Your Move:</span>
          <span className="ev-value">
            {result.userActionEv !== undefined 
              ? result.userActionEv.toFixed(2) 
              : '—'}
          </span>
        </div>
        <div className="ev-row">
          <span className="ev-label">Best Move:</span>
          <span className="ev-value">{result.bestActionEv.toFixed(2)}</span>
        </div>
        {result.deltaEv !== undefined && (
          <div className="ev-row delta">
            <span className="ev-label">Difference:</span>
            <span className={`ev-value ${result.deltaEv < 0 ? 'negative' : 'positive'}`}>
              {result.deltaEv >= 0 ? '+' : ''}{result.deltaEv.toFixed(2)}
            </span>
          </div>
        )}
      </div>
      
      {/* Feedback Bullets */}
      {result.feedback && result.feedback.length > 0 && (
        <div className="feedback-section">
          <h3>Analysis</h3>
          <ul className="feedback-list">
            {result.feedback.map((bullet, idx) => (
              <li key={idx} className={`feedback-bullet ${bullet.category}`}>
                {bullet.text}
              </li>
            ))}
          </ul>
        </div>
      )}
      
      {/* Best Move Display */}
      <div className="best-move-section">
        <h3>Best Move</h3>
        <div className="best-move-description">
          {describeAction(result.bestAction)}
        </div>
      </div>
      
      {/* Actions */}
      <div className="result-actions">
        <button className="btn-primary" onClick={onNextScenario}>
          Next Scenario
        </button>
      </div>
      
      {/* Diagnostics (collapsible) */}
      <details className="diagnostics">
        <summary>Evaluation Details</summary>
        <div className="diagnostics-content">
          <div className="diagnostic-row">
            <span>Time:</span>
            <span>{result.metadata.elapsedMs}ms</span>
          </div>
          <div className="diagnostic-row">
            <span>Rollouts:</span>
            <span>{result.metadata.rolloutsRun}</span>
          </div>
          <div className="diagnostic-row">
            <span>Candidates:</span>
            <span>
              {result.metadata.candidatesEvaluated} of {result.metadata.totalLegalActions}
            </span>
          </div>
          <div className="diagnostic-row">
            <span>Seed:</span>
            <span className="monospace">{result.metadata.seed}</span>
          </div>
        </div>
      </details>
    </div>
  );
}

function computeGrade(deltaEv: number): string {
  const abs = Math.abs(deltaEv);
  if (abs <= 0.25) return 'EXCELLENT';
  if (abs <= 1.0) return 'GOOD';
  if (abs <= 2.5) return 'OKAY';
  return 'MISS';
}

function getGradeClass(grade: string): string {
  return `grade-${grade.toLowerCase()}`;
}

function getGradeText(grade: string): string {
  switch (grade) {
    case 'EXCELLENT': return 'Excellent! You found the best move.';
    case 'GOOD': return 'Good move! Close to optimal.';
    case 'OKAY': return 'Okay move, but there\'s a better option.';
    case 'MISS': return 'Missed opportunity. Review the best move.';
    default: return '';
  }
}

function describeAction(action: any): string {
  const source = action.source.Factory !== undefined 
    ? `Factory ${action.source.Factory + 1}`
    : 'Center';
  
  const dest = action.destination.PatternLine !== undefined
    ? `Pattern Line ${action.destination.PatternLine + 1}`
    : 'Floor';
  
  return `Take ${action.color} from ${source} → ${dest}`;
}
```

**Styles:** `web/src/components/EvaluationResult.css`

```css
.evaluation-result {
  padding: 20px;
  background: var(--surface);
  border-radius: 8px;
  display: flex;
  flex-direction: column;
  gap: 20px;
}

/* Grade Badge */
.grade-badge {
  padding: 16px;
  border-radius: 8px;
  text-align: center;
}

.grade-badge.grade-excellent {
  background: #d4edda;
  border: 2px solid #28a745;
}

.grade-badge.grade-good {
  background: #d1ecf1;
  border: 2px solid #17a2b8;
}

.grade-badge.grade-okay {
  background: #fff3cd;
  border: 2px solid #ffc107;
}

.grade-badge.grade-miss {
  background: #f8d7da;
  border: 2px solid #dc3545;
}

.grade-label {
  font-size: 24px;
  font-weight: 700;
  margin-bottom: 4px;
}

.grade-text {
  font-size: 14px;
}

/* EV Comparison */
.ev-comparison {
  display: flex;
  flex-direction: column;
  gap: 8px;
  padding: 12px;
  background: var(--background);
  border-radius: 4px;
}

.ev-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  font-size: 16px;
}

.ev-row.delta {
  margin-top: 8px;
  padding-top: 8px;
  border-top: 1px solid var(--border);
  font-weight: 600;
}

.ev-value.negative {
  color: #dc3545;
}

.ev-value.positive {
  color: #28a745;
}

/* Feedback */
.feedback-section h3 {
  margin: 0 0 12px 0;
  font-size: 18px;
}

.feedback-list {
  list-style: none;
  padding: 0;
  margin: 0;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.feedback-bullet {
  padding: 10px 12px;
  background: var(--background);
  border-left: 3px solid var(--accent);
  border-radius: 4px;
  font-size: 14px;
  line-height: 1.5;
}

/* Best Move */
.best-move-section h3 {
  margin: 0 0 8px 0;
  font-size: 18px;
}

.best-move-description {
  padding: 12px;
  background: var(--background);
  border-radius: 4px;
  font-family: 'Courier New', monospace;
  font-size: 14px;
}

/* Actions */
.result-actions {
  display: flex;
  gap: 12px;
  justify-content: center;
}

.btn-primary {
  padding: 10px 24px;
  background: var(--primary);
  color: white;
  border: none;
  border-radius: 4px;
  font-size: 16px;
  font-weight: 600;
  cursor: pointer;
  transition: background 0.2s;
}

.btn-primary:hover {
  background: var(--primary-dark);
}

/* Diagnostics */
.diagnostics {
  border-top: 1px solid var(--border);
  padding-top: 12px;
}

.diagnostics summary {
  cursor: pointer;
  font-weight: 600;
  font-size: 14px;
  color: var(--text-secondary);
}

.diagnostics-content {
  margin-top: 8px;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.diagnostic-row {
  display: flex;
  justify-content: space-between;
  font-size: 13px;
  color: var(--text-secondary);
}

.monospace {
  font-family: 'Courier New', monospace;
}
```

---

### Component 3: PracticeScreen Integration

Update PracticeScreen to include evaluation flow.

**Updates to:** `web/src/components/PracticeScreen.tsx`

```typescript
// Add state for evaluation
const [timeBudget, setTimeBudget] = useState<TimeBudget>(250);
const [isEvaluating, setIsEvaluating] = useState(false);
const [evaluationResult, setEvaluationResult] = useState<EvaluationResult | null>(null);
const [userAction, setUserAction] = useState<DraftAction | null>(null);

// Modify apply action to store user's action
const handleApplyAction = useCallback(async () => {
  if (!selection || !engine || !gameState) return;
  
  try {
    const action = buildDraftAction(selection);
    setUserAction(action); // Store for evaluation
    
    // Apply action...
    const newState = await applyAction(engine, gameState, action);
    setGameState(newState);
    setSelection(null);
  } catch (err) {
    // Error handling...
  }
}, [selection, engine, gameState]);

// Add evaluate handler
const handleEvaluate = useCallback(async () => {
  if (!engine || !gameState || !userAction) return;
  
  setIsEvaluating(true);
  try {
    const params: EvaluatorParams = {
      timeBudgetMs: timeBudget,
      rolloutsPerAction: timeBudget / 25, // ~10 for 250ms, ~30 for 750ms, ~60 for 1500ms
      evaluatorSeed: Date.now(),
      shortlistSize: 20,
    };
    
    const result = await gradeUserAction(engine, gameState, 0, userAction, params);
    setEvaluationResult(result);
  } catch (err) {
    console.error('Evaluation failed:', err);
    // Show error toast...
  } finally {
    setIsEvaluating(false);
  }
}, [engine, gameState, userAction, timeBudget]);

// Add JSX for evaluation UI
return (
  <div className="practice-screen">
    {/* Existing board rendering... */}
    
    {/* Evaluation Controls */}
    {userAction && !evaluationResult && (
      <div className="evaluation-controls">
        <ThinkLongerControl 
          value={timeBudget}
          onChange={setTimeBudget}
          disabled={isEvaluating}
        />
        <button 
          className="btn-evaluate"
          onClick={handleEvaluate}
          disabled={isEvaluating}
        >
          {isEvaluating ? 'Evaluating...' : 'Evaluate Move'}
        </button>
      </div>
    )}
    
    {/* Evaluation Result */}
    {evaluationResult && (
      <EvaluationResult 
        result={evaluationResult}
        onNextScenario={() => {
          setEvaluationResult(null);
          setUserAction(null);
          handleGenerateScenario();
        }}
      />
    )}
  </div>
);
```

---

### Component 4: DevPanel Enhancement

Add evaluation diagnostics to dev panel.

**Updates to:** `web/src/components/dev/DevPanel.tsx`

```typescript
// Add section for evaluation diagnostics
{evaluationResult && (
  <div className="dev-section">
    <h3>Evaluation Diagnostics</h3>
    
    <div className="dev-info">
      <span>Elapsed Time:</span>
      <span>{evaluationResult.metadata.elapsedMs}ms</span>
    </div>
    
    <div className="dev-info">
      <span>Rollouts Executed:</span>
      <span>{evaluationResult.metadata.rolloutsRun}</span>
    </div>
    
    <div className="dev-info">
      <span>Actions Evaluated:</span>
      <span>
        {evaluationResult.metadata.candidatesEvaluated} / 
        {evaluationResult.metadata.totalLegalActions}
      </span>
    </div>
    
    <div className="dev-info">
      <span>Within Budget:</span>
      <span>{evaluationResult.metadata.completedWithinBudget ? 'Yes' : 'No'}</span>
    </div>
    
    <div className="dev-info">
      <span>Evaluator Seed:</span>
      <span className="monospace">{evaluationResult.metadata.seed}</span>
    </div>
    
    {evaluationResult.candidates && (
      <details>
        <summary>All Candidates ({evaluationResult.candidates.length})</summary>
        <div className="candidates-list">
          {evaluationResult.candidates.map((candidate, idx) => (
            <div key={idx} className="candidate-item">
              <span className="rank">#{idx + 1}</span>
              <span className="action">{describeAction(candidate.action)}</span>
              <span className="ev">{candidate.ev.toFixed(2)}</span>
            </div>
          ))}
        </div>
      </details>
    )}
    
    <button 
      className="dev-button"
      onClick={() => {
        const json = JSON.stringify(evaluationResult, null, 2);
        navigator.clipboard.writeText(json);
      }}
    >
      Copy Evaluation JSON
    </button>
  </div>
)}
```

---

## Test Requirements

### Unit Tests (Rust)

**Test 1: Feature tracking collects statistics**
```rust
#[test]
fn test_feature_tracking() {
    let state = create_test_state();
    let params = EvaluatorParams {
        time_budget_ms: 250,
        rollouts_per_action: 10,
        evaluator_seed: 12345,
        shortlist_size: 20,
        rollout_config: RolloutPolicyConfig::default(),
    };
    
    let result = evaluate_best_move(&state, 0, &params).unwrap();
    
    // Should have feature data for best action
    assert!(result.best_features.expected_floor_penalty <= 0.0);
    assert!(result.best_features.expected_completions >= 0.0);
    assert!(result.best_features.tiles_acquired > 0);
}
```

**Test 2: Feedback generation produces bullets**
```rust
#[test]
fn test_feedback_generation() {
    let user_features = ActionFeatures {
        expected_floor_penalty: -3.0,
        expected_completions: 0.2,
        expected_adjacency_points: 2.0,
        expected_tiles_to_floor: 2.5,
        takes_first_player_token: true,
        tiles_acquired: 3,
    };
    
    let best_features = ActionFeatures {
        expected_floor_penalty: -1.0,
        expected_completions: 0.8,
        expected_adjacency_points: 4.0,
        expected_tiles_to_floor: 0.5,
        takes_first_player_token: false,
        tiles_acquired: 4,
    };
    
    let feedback = generate_feedback_bullets(&user_features, &best_features);
    
    // Should generate at least one bullet
    assert!(!feedback.is_empty());
    
    // Should be sorted by importance
    for i in 0..feedback.len() - 1 {
        assert!(feedback[i].delta >= feedback[i + 1].delta);
    }
    
    // Should not exceed 3 bullets
    assert!(feedback.len() <= 3);
}
```

**Test 3: Grade computation**
```rust
#[test]
fn test_grade_computation() {
    assert_eq!(compute_grade(0.1), Grade::Excellent);
    assert_eq!(compute_grade(0.5), Grade::Good);
    assert_eq!(compute_grade(1.5), Grade::Okay);
    assert_eq!(compute_grade(3.0), Grade::Miss);
    
    // Negative deltas (user better than "best" - shouldn't happen but handle it)
    assert_eq!(compute_grade(-0.1), Grade::Excellent);
}
```

### Integration Tests (UI)

**Test 1: Evaluation flow**
- Generate scenario
- Make move
- Click "Evaluate"
- Verify results display
- Verify grade shown
- Verify feedback bullets present

**Test 2: Think Longer control**
- Change time budget
- Verify evaluations use correct budget
- Verify longer budgets produce more rollouts

**Test 3: Next scenario flow**
- Complete evaluation
- Click "Next Scenario"
- Verify new scenario loads
- Verify evaluation result cleared

---

## Acceptance Criteria

- [ ] Feature delta tracking implemented in evaluator
- [ ] Feedback bullet generation produces 1-3 explanations
- [ ] Grading system maps delta EV to grades correctly
- [ ] ThinkLongerControl component functional
- [ ] EvaluationResult component displays all information clearly
- [ ] PracticeScreen integrated with evaluation flow
- [ ] DevPanel shows evaluation diagnostics
- [ ] End-to-end practice loop works: generate → play → evaluate → repeat
- [ ] All unit tests pass
- [ ] UI components render correctly
- [ ] Responsive during evaluation (consider Web Worker if needed)

---

## Files to Create/Modify

### New Files (Rust)

```
rust/engine/src/
└── rules/
    └── feedback.rs                (~200 lines)
        ├── ActionFeatures
        ├── FeedbackBullet
        ├── FeedbackCategory
        ├── Grade
        ├── generate_feedback_bullets()
        ├── compute_grade()
        └── helper functions
```

### Modified Files (Rust)

```
rust/engine/src/
└── rules/
    ├── evaluator.rs               (~100 lines added)
    │   └── Integrate feature tracking
    │
    └── mod.rs                     (~2 lines added)
        └── pub mod feedback;
        └── pub use feedback::*;
```

### New Files (TypeScript/React)

```
web/src/
└── components/
    ├── ThinkLongerControl.tsx     (~50 lines)
    ├── ThinkLongerControl.css     (~30 lines)
    ├── EvaluationResult.tsx       (~150 lines)
    └── EvaluationResult.css       (~150 lines)
```

### Modified Files (TypeScript/React)

```
web/src/
├── components/
│   ├── PracticeScreen.tsx         (~150 lines modified)
│   ├── PracticeScreen.css         (~50 lines added)
│   └── dev/
│       └── DevPanel.tsx           (~80 lines added)
│
└── wasm/
    └── evaluator.ts               (~20 lines added)
        └── Type definitions for new features
```

---

## Performance Considerations

### Web Worker for Long Evaluations

If evaluation blocks the UI (>1500ms budgets):

```typescript
// web/src/workers/evaluator.worker.ts
self.onmessage = async (e) => {
  const { state, playerId, userAction, params } = e.data;
  
  try {
    const result = await gradeUserAction(engine, state, playerId, userAction, params);
    self.postMessage({ success: true, result });
  } catch (err) {
    self.postMessage({ success: false, error: err.message });
  }
};
```

**Decision:** Implement in Sprint 5C only if 1500ms budget causes noticeable UI lag.

---

## Design Decisions

### Why Template-Based Feedback?

**Decision:** Use predefined templates rather than generative/LLM feedback

**Rationale:**
- Deterministic and controllable
- Fast (no API calls)
- Accurate (based on measured features)
- Easier to test and validate
- MVP-appropriate scope

**Future:** Could enhance with more nuanced templates or hybrid approach

---

### Why Limit to 3 Feedback Bullets?

**Decision:** Show top 3 most important feedback points

**Rationale:**
- Focus on key insights
- Avoid overwhelming user
- Mobile-friendly (less scrolling)
- Research recommendation: "1-3 bullets"

---

### Why Feature Deltas Over Absolute Features?

**Decision:** Compare user action features to best action features

**Rationale:**
- Relative comparisons more actionable ("2 points better" vs "scored 5 points")
- Highlights what user missed
- Aligns with teaching/learning goals

---

## Related Documentation

- [Sprint 05A: Rollout Simulation](Sprint_05A_Rollout_Simulation.md)
- [Sprint 05B: Evaluator Core](Sprint_05B_Evaluator_Core.md)
- [Spec: Best Move Evaluation & Feedback](../specs/08_best_move_evaluation_and_feedback.md)
- [Research Synthesis](../engineering/azul_best_move_algorithm_research_synthesis.md) - Section 6: Explainable feedback

---

## Next Steps

After completing Sprint 05C:
- **Sprint 05 is complete!** Full evaluation and feedback system functional
- Consider Sprint 06 for additional polish (drag-drop, animations, accessibility)
- Or move to Sprint 07 for calibration and advanced features
- The MVP core loop (generate → play → evaluate → feedback) is fully functional
