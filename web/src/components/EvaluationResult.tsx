import { useState, useEffect } from 'react';
import { EvaluationResult as EvalResult, Grade } from '../wasm/evaluator';
import { describeAction } from '../wasm/engine';
import { BestMoveOverlay } from './ui/BestMoveOverlay';
import './EvaluationResult.css';

interface EvaluationResultProps {
  result: EvalResult;
  onNextScenario: () => void;
}

export function EvaluationResult({ result, onNextScenario }: EvaluationResultProps) {
  const [isRevealing, setIsRevealing] = useState(true);
  const [showBestMoveOverlay, setShowBestMoveOverlay] = useState(false);

  useEffect(() => {
    setIsRevealing(true);
    const timer = setTimeout(() => setIsRevealing(false), 500);
    return () => clearTimeout(timer);
  }, [result]);

  return (
    <div className="evaluation-result">
      {/* Best Move Overlay */}
      {showBestMoveOverlay && (
        <BestMoveOverlay
          bestAction={result.best_action}
          onDismiss={() => setShowBestMoveOverlay(false)}
        />
      )}
      
      {/* Grade Badge */}
      {result.grade && (
        <div className={`grade-badge grade-${result.grade.toLowerCase()} ${isRevealing ? 'revealing' : ''}`}>
          <div className="grade-label">{result.grade}</div>
          <div className="grade-text">{getGradeText(result.grade)}</div>
        </div>
      )}
      
      {/* EV Comparison */}
      <div className="ev-comparison">
        <div className="ev-row">
          <span className="ev-label">Your Move EV:</span>
          <span className="ev-value">
            {result.user_action_ev !== undefined 
              ? result.user_action_ev.toFixed(2) 
              : '—'}
          </span>
        </div>
        <div className="ev-row">
          <span className="ev-label">Best Move EV:</span>
          <span className="ev-value">{result.best_action_ev.toFixed(2)}</span>
        </div>
        {result.delta_ev !== undefined && (
          <div className="ev-row delta">
            <span className="ev-label">Difference:</span>
            <span className={`ev-value ${result.delta_ev < 0 ? 'negative' : 'positive'}`}>
              {result.delta_ev >= 0 ? '+' : ''}{result.delta_ev.toFixed(2)}
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
              <li key={idx} className={`feedback-bullet category-${bullet.category}`}>
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
          {describeAction(result.best_action)}
        </div>
        <button 
          className="btn-show-best-move"
          onClick={() => setShowBestMoveOverlay(true)}
          aria-label="Show best move on board"
        >
          Show Best Move
        </button>
      </div>
      
      {/* Actions */}
      <div className="result-actions">
        <button className="btn-primary" onClick={onNextScenario}>
          Next Scenario
        </button>
      </div>
      
      {/* Diagnostics */}
      <details className="diagnostics">
        <summary>Evaluation Details</summary>
        <div className="diagnostics-content">
          <div className="diagnostic-row">
            <span>Rollouts:</span>
            <span>{result.metadata.rollouts_run}</span>
          </div>
          <div className="diagnostic-row">
            <span>Candidates:</span>
            <span>{result.metadata.candidates_evaluated} of {result.metadata.total_legal_actions}</span>
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

function getGradeText(grade: Grade): string {
  switch (grade) {
    case 'EXCELLENT': return 'Excellent! You found the best move.';
    case 'GOOD': return 'Good move! Close to optimal.';
    case 'OKAY': return 'Okay move, but there is a better option.';
    case 'MISS': return 'Missed opportunity. Review the best move.';
  }
}
