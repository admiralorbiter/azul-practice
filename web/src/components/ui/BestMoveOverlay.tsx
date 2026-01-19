import { useState, useEffect } from 'react';
import { DraftAction } from '../../wasm/engine';
import './BestMoveOverlay.css';

interface BestMoveOverlayProps {
  bestAction: DraftAction;
  onDismiss: () => void;
}

export function BestMoveOverlay({ bestAction, onDismiss }: BestMoveOverlayProps) {
  const [isVisible, setIsVisible] = useState(false);

  useEffect(() => {
    // Fade in animation
    setIsVisible(true);
  }, []);

  const handleDismiss = () => {
    setIsVisible(false);
    setTimeout(onDismiss, 300); // Wait for fade out
  };

  // Helper to format source description
  const getSourceDescription = () => {
    if (bestAction.source === 'Center') {
      return 'Take from Center';
    } else if (typeof bestAction.source !== 'string' && 'Factory' in bestAction.source) {
      return `Take from Factory ${bestAction.source.Factory + 1}`;
    }
    return 'Take tiles';
  };

  // Helper to format destination description
  const getDestinationDescription = () => {
    if (bestAction.destination === 'Floor') {
      return 'Place to Floor';
    } else if (typeof bestAction.destination !== 'string' && 'PatternLine' in bestAction.destination) {
      return `Place to Pattern Line ${bestAction.destination.PatternLine + 1}`;
    }
    return 'Place tiles';
  };

  return (
    <div className={`best-move-overlay ${isVisible ? 'visible' : ''}`}>
      <div className="overlay-backdrop" onClick={handleDismiss} />
      
      <div className="overlay-content">
        <div className="overlay-header">
          <h3>Best Move</h3>
        </div>
        
        <div className="overlay-instructions">
          <div className="instruction-step">
            <div className="step-number">1</div>
            <div className="step-content">
              <div className="step-title">{getSourceDescription()}</div>
              <div className="step-detail">Color: {bestAction.color}</div>
            </div>
          </div>
          
          <div className="instruction-arrow">↓</div>
          
          <div className="instruction-step">
            <div className="step-number">2</div>
            <div className="step-content">
              <div className="step-title">{getDestinationDescription()}</div>
            </div>
          </div>
        </div>
        
        <button
          className="overlay-dismiss"
          onClick={handleDismiss}
          aria-label="Close best move overlay"
        >
          Got it!
        </button>
      </div>
    </div>
  );
}
