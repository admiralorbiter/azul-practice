import { useState } from 'react';
import { GameState, DraftAction, ActionSource, Destination } from '../wasm/engine';

type SelectionState =
  | { stage: 'idle' }
  | { stage: 'source-selected'; source: ActionSource; color: string }
  | { stage: 'action-ready'; action: DraftAction };

interface UseActionSelectionOptions {
  gameState: GameState | null;
  legalActions: DraftAction[] | null;
}

interface UseActionSelectionReturn {
  selectionState: SelectionState;
  selectSource: (source: ActionSource, color: string) => void;
  selectDestination: (destination: Destination) => void;
  cancelSelection: () => void;
  getHighlightedDestinations: () => Set<number>;
  isSourceSelectable: (source: ActionSource) => boolean;
  isDestinationSelectable: (destination: Destination) => boolean;
}

function sourceEquals(a: ActionSource, b: ActionSource): boolean {
  if (a === 'Center' && b === 'Center') return true;
  if (typeof a !== 'string' && typeof b !== 'string') {
    return a.Factory === b.Factory;
  }
  return false;
}

function destinationToNumber(dest: Destination): number {
  if (dest === 'Floor') return -1;
  if (typeof dest !== 'string' && 'PatternLine' in dest) {
    return dest.PatternLine;
  }
  return -1;
}

export function useActionSelection({
  gameState,
  legalActions,
}: UseActionSelectionOptions): UseActionSelectionReturn {
  const [selectionState, setSelectionState] = useState<SelectionState>({ stage: 'idle' });

  const selectSource = (source: ActionSource, color: string) => {
    setSelectionState({ stage: 'source-selected', source, color });
  };

  const selectDestination = (destination: Destination) => {
    if (selectionState.stage === 'source-selected') {
      const action: DraftAction = {
        source: selectionState.source,
        color: selectionState.color,
        destination,
      };
      setSelectionState({ stage: 'action-ready', action });
    }
  };

  const cancelSelection = () => {
    setSelectionState({ stage: 'idle' });
  };

  const getHighlightedDestinations = (): Set<number> => {
    const highlighted = new Set<number>();
    
    if (selectionState.stage !== 'source-selected' || !legalActions) {
      return highlighted;
    }

    const { source, color } = selectionState;

    // Find all legal actions that match the current source and color
    const matchingActions = legalActions.filter(
      action => sourceEquals(action.source, source) && action.color === color
    );

    // Add all valid destinations to the set
    matchingActions.forEach(action => {
      highlighted.add(destinationToNumber(action.destination));
    });

    return highlighted;
  };

  const isSourceSelectable = (source: ActionSource): boolean => {
    if (!legalActions || !gameState) return false;
    return legalActions.some(action => sourceEquals(action.source, source));
  };

  const isDestinationSelectable = (destination: Destination): boolean => {
    if (selectionState.stage !== 'source-selected' || !legalActions) {
      return false;
    }

    const { source, color } = selectionState;
    return legalActions.some(
      action =>
        sourceEquals(action.source, source) &&
        action.color === color &&
        destinationToNumber(action.destination) === destinationToNumber(destination)
    );
  };

  return {
    selectionState,
    selectSource,
    selectDestination,
    cancelSelection,
    getHighlightedDestinations,
    isSourceSelectable,
    isDestinationSelectable,
  };
}
