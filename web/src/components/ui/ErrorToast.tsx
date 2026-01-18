import { useEffect } from 'react';
import './ErrorToast.css';

interface ErrorToastProps {
  message: string;
  code?: string;
  onDismiss: () => void;
  autoHideDuration?: number;
}

export function ErrorToast({ message, code, onDismiss, autoHideDuration = 5000 }: ErrorToastProps) {
  useEffect(() => {
    if (autoHideDuration > 0) {
      const timer = setTimeout(onDismiss, autoHideDuration);
      return () => clearTimeout(timer);
    }
  }, [autoHideDuration, onDismiss]);

  return (
    <div className="error-toast">
      <div className="error-toast-content">
        <div className="error-toast-icon">⚠️</div>
        <div className="error-toast-text">
          {code && <div className="error-toast-code">{code}</div>}
          <div className="error-toast-message">{message}</div>
        </div>
        <button className="error-toast-close" onClick={onDismiss} aria-label="Dismiss">
          ✕
        </button>
      </div>
    </div>
  );
}
