import { audio, Webview } from '@hmcs/sdk';
import { useCallback } from 'react';

export default function Toolbar() {
  const handleClose = useCallback(() => {
    audio.se.play('se:close');
    Webview.current()?.close();
  }, []);

  return (
    <div className="toolbar">
      <span className="toolbar-title">Persona</span>
      <button type="button" className="toolbar-close" onClick={handleClose} aria-label="Close">
        <svg
          aria-hidden="true"
          width="10"
          height="10"
          viewBox="0 0 10 10"
          fill="none"
          stroke="currentColor"
          strokeWidth="1.5"
        >
          <path d="M1 1l8 8M9 1L1 9" />
        </svg>
      </button>
    </div>
  );
}
