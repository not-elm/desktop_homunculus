import { useCallback, useEffect, useRef, useState } from "react";

interface KeyCaptureFieldProps {
  label: string;
  description?: string;
  pttKey: string | null;
  onChange: (key: string | null) => void;
}

export function KeyCaptureField({
  label,
  description,
  pttKey,
  onChange,
}: KeyCaptureFieldProps) {
  const [capturing, setCapturing] = useState(false);
  const [displayName, setDisplayName] = useState<string>(
    pttKey !== null ? formatKeyName(pttKey) : "None",
  );
  const containerRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    setDisplayName(pttKey !== null ? formatKeyName(pttKey) : "None");
  }, [pttKey]);

  const startCapture = useCallback(() => {
    setCapturing(true);
    containerRef.current?.focus();
  }, []);

  const handleKeyDown = useCallback(
    (e: React.KeyboardEvent<HTMLDivElement>) => {
      if (!capturing) return;
      e.preventDefault();
      onChange(e.code);
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

/** Converts a browser `KeyboardEvent.code` to a user-friendly display name. */
function formatKeyName(code: string): string {
  if (code.startsWith("Key")) return code.slice(3);
  if (code.startsWith("Digit")) return code.slice(5);
  return code.replace(/(Left|Right)$/, " ($1)");
}
