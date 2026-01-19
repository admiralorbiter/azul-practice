import { useState, useCallback, useRef } from 'react';
import { ActionSource, Destination, DraftAction, GameState } from '../wasm/engine';

interface DraggedTile {
  source: ActionSource;
  color: string;
  count: number;
}

interface DragAndDropHook {
  // Drag source props
  getDragSourceProps: (source: ActionSource, color: string, count: number) => {
    draggable: boolean;
    onDragStart: (e: React.DragEvent) => void;
    onDragEnd: (e: React.DragEvent) => void;
    className?: string;
  };
  
  // Drop target props
  getDropTargetProps: (destination: Destination) => {
    onDragOver: (e: React.DragEvent) => void;
    onDragEnter: (e: React.DragEvent) => void;
    onDragLeave: (e: React.DragEvent) => void;
    onDrop: (e: React.DragEvent) => void;
    className?: string;
  };
  
  // State
  isDragging: boolean;
  draggedItem: DraggedTile | null;
  hoveredTarget: string | null;
  snapBack: boolean;
  
  // Actions
  isValidTarget: (destination: Destination) => boolean;
  onDropAction: (action: DraftAction) => void;
  onInvalidDrop: (reason: string) => void;
}

interface UseDragAndDropOptions {
  gameState: GameState | null;
  legalActions: DraftAction[] | null;
  onApplyAction: (action: DraftAction) => void;
  onError: (message: string) => void;
}

function sourceEquals(a: ActionSource, b: ActionSource): boolean {
  if (a === 'Center' && b === 'Center') return true;
  if (typeof a !== 'string' && typeof b !== 'string') {
    return a.Factory === b.Factory;
  }
  return false;
}

function destinationEquals(a: Destination, b: Destination): boolean {
  if (a === 'Floor' && b === 'Floor') return true;
  if (typeof a !== 'string' && typeof b !== 'string') {
    return a.PatternLine === b.PatternLine;
  }
  return false;
}

function destinationToId(dest: Destination): string {
  if (dest === 'Floor') return 'floor';
  if (typeof dest !== 'string' && 'PatternLine' in dest) {
    return `pattern-${dest.PatternLine}`;
  }
  return 'unknown';
}

export function useDragAndDrop({
  legalActions,
  onApplyAction,
  onError,
}: UseDragAndDropOptions): DragAndDropHook {
  const [isDragging, setIsDragging] = useState(false);
  const [draggedItem, setDraggedItem] = useState<DraggedTile | null>(null);
  const [hoveredTarget, setHoveredTarget] = useState<string | null>(null);
  const [snapBack, setSnapBack] = useState(false);
  const dragCounter = useRef(0);

  const isValidTarget = useCallback((destination: Destination): boolean => {
    if (!draggedItem || !legalActions) return false;
    
    return legalActions.some(
      action =>
        sourceEquals(action.source, draggedItem.source) &&
        action.color === draggedItem.color &&
        destinationEquals(action.destination, destination)
    );
  }, [draggedItem, legalActions]);

  const getDragSourceProps = useCallback((source: ActionSource, color: string, count: number) => {
    return {
      draggable: true,
      onDragStart: (_e: React.DragEvent) => {
        const e = _e;
        // Store drag data
        e.dataTransfer.effectAllowed = 'move';
        e.dataTransfer.setData('text/plain', JSON.stringify({ source, color, count }));
        
        // Create custom ghost image
        const ghost = document.createElement('div');
        ghost.className = 'drag-ghost';
        ghost.textContent = `${count}× ${color}`;
        ghost.style.cssText = `
          position: absolute;
          top: -1000px;
          background: var(--tile-${color.toLowerCase()}, #666);
          color: white;
          padding: 8px 16px;
          border-radius: 4px;
          opacity: 0.8;
          pointer-events: none;
          font-weight: 600;
        `;
        document.body.appendChild(ghost);
        e.dataTransfer.setDragImage(ghost, 40, 20);
        setTimeout(() => ghost.remove(), 0);
        
        setIsDragging(true);
        setDraggedItem({ source, color, count });
      },
      onDragEnd: (_e: React.DragEvent) => {
        setIsDragging(false);
        setDraggedItem(null);
        setHoveredTarget(null);
      },
      className: isDragging && draggedItem?.source === source && draggedItem?.color === color ? 'dragging' : '',
    };
  }, [isDragging, draggedItem]);

  const getDropTargetProps = useCallback((destination: Destination) => {
    const targetId = destinationToId(destination);
    const isValid = isValidTarget(destination);
    
    return {
      onDragOver: (_e: React.DragEvent) => {
        const e = _e;
        if (isValid) {
          e.preventDefault();
          e.dataTransfer.dropEffect = 'move';
        }
      },
      onDragEnter: (_e: React.DragEvent) => {
        if (isValid) {
          dragCounter.current++;
          if (dragCounter.current === 1) {
            setHoveredTarget(targetId);
          }
        }
      },
      onDragLeave: (_e: React.DragEvent) => {
        if (isValid) {
          dragCounter.current--;
          if (dragCounter.current === 0) {
            setHoveredTarget(null);
          }
        }
      },
      onDrop: (e: React.DragEvent) => {
        e.preventDefault();
        dragCounter.current = 0;
        setHoveredTarget(null);
        
        if (!draggedItem) return;
        
        if (isValid) {
          const action: DraftAction = {
            source: draggedItem.source,
            color: draggedItem.color,
            destination,
          };
          onApplyAction(action);
        } else {
          // Trigger snap-back animation
          setSnapBack(true);
          setTimeout(() => setSnapBack(false), 300);
          onError('Cannot drop here - invalid destination');
        }
      },
      className: [
        isValid && isDragging && 'valid-target',
        hoveredTarget === targetId && 'hovered'
      ].filter(Boolean).join(' '),
    };
  }, [isDragging, draggedItem, hoveredTarget, isValidTarget, onApplyAction, onError]);

  const onDropAction = useCallback((action: DraftAction) => {
    onApplyAction(action);
  }, [onApplyAction]);

  const onInvalidDrop = useCallback((reason: string) => {
    setSnapBack(true);
    setTimeout(() => setSnapBack(false), 300);
    onError(reason);
  }, [onError]);

  return {
    getDragSourceProps,
    getDropTargetProps,
    isDragging,
    draggedItem,
    hoveredTarget,
    snapBack,
    isValidTarget,
    onDropAction,
    onInvalidDrop,
  };
}
