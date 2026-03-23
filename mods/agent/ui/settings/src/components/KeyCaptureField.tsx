import { useCallback, useEffect, useRef, useState } from "react";

interface KeyCaptureFieldProps {
  label: string;
  description?: string;
  keyCode: number | null;
  onChange: (keyCode: number | null) => void;
}

export function KeyCaptureField({
  label,
  description,
  keyCode,
  onChange,
}: KeyCaptureFieldProps) {
  const [capturing, setCapturing] = useState(false);
  const [displayName, setDisplayName] = useState<string>(
    keyCode !== null ? `Code ${keyCode}` : "None",
  );
  const containerRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    setDisplayName(keyCode !== null ? `Code ${keyCode}` : "None");
  }, [keyCode]);

  const startCapture = useCallback(() => {
    setCapturing(true);
    containerRef.current?.focus();
  }, []);

  const handleKeyDown = useCallback(
    (e: React.KeyboardEvent<HTMLDivElement>) => {
      if (!capturing) return;
      e.preventDefault();
      const code = resolveKeyCode(e.code);
      setDisplayName(e.code);
      onChange(code);
      setCapturing(false);
    },
    [capturing, onChange],
  );

  const handleBlur = useCallback(() => {
    setCapturing(false);
  }, []);

  return (
    <div className="settings-label">
      {label}
      {description && (
        <span className="settings-label-desc">{description}</span>
      )}
      <div
        ref={containerRef}
        className={`agent-key-capture${capturing ? " agent-key-capture--capturing" : ""}`}
        tabIndex={0}
        onClick={startCapture}
        onKeyDown={handleKeyDown}
        onBlur={handleBlur}
        role="button"
        aria-label={capturing ? "Press a key to capture" : `Current key: ${displayName}. Click to change.`}
      >
        <span className="agent-key-badge">{displayName}</span>
        <span className="agent-key-hint">
          {capturing ? "Press any key..." : "Click to capture"}
        </span>
      </div>
    </div>
  );
}

/**
 * Maps a browser KeyboardEvent.code string to a numeric keycode.
 * Uses the key code value directly encoded as a hash of the string for
 * cross-platform consistency. The service layer (using uiohook) will
 * interpret these values independently.
 */
function resolveKeyCode(code: string): number {
  let hash = 0;
  for (let i = 0; i < code.length; i++) {
    hash = (hash * 31 + code.charCodeAt(i)) >>> 0;
  }
  return hash;
}
