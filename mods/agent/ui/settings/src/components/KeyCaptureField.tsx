import { useCallback, useEffect, useRef, useState } from "react";
import type { PttKey } from "../hooks/useAgentSettings";

interface KeyCaptureFieldProps {
  label: string;
  description?: string;
  pttKey: PttKey | null;
  onChange: (key: PttKey | null) => void;
}

export function KeyCaptureField({
  label,
  description,
  pttKey,
  onChange,
}: KeyCaptureFieldProps) {
  const [capturing, setCapturing] = useState(false);
  const [displayName, setDisplayName] = useState<string>(
    pttKey !== null ? formatPttKeyName(pttKey) : "None",
  );
  const containerRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    setDisplayName(pttKey !== null ? formatPttKeyName(pttKey) : "None");
  }, [pttKey]);

  const startCapture = useCallback(() => {
    setCapturing(true);
    containerRef.current?.focus();
  }, []);

  const handleKeyDown = useCallback(
    (e: React.KeyboardEvent<HTMLDivElement>) => {
      if (!capturing) return;
      e.preventDefault();
      if (isModifierCode(e.code)) return;
      onChange({ code: e.code, modifiers: collectModifiers(e) });
      setCapturing(false);
    },
    [capturing, onChange],
  );

  const handleKeyUp = useCallback(
    (e: React.KeyboardEvent<HTMLDivElement>) => {
      if (!capturing) return;
      if (!isModifierCode(e.code)) return;
      if (hasActiveModifiers(e)) return;
      onChange({ code: e.code, modifiers: [] });
      setCapturing(false);
    },
    [capturing, onChange],
  );

  const handleBlur = useCallback(() => {
    setCapturing(false);
  }, []);

  const showWarning = pttKey !== null && isModifierCode(pttKey.code) && pttKey.modifiers.length === 0;

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
        onKeyUp={handleKeyUp}
        onBlur={handleBlur}
        role="button"
        aria-label={capturing ? "Press a key to capture" : `Current key: ${displayName}. Click to change.`}
      >
        <span className="agent-key-badge">{displayName}</span>
        <span className="agent-key-hint">
          {capturing ? "Press any key..." : "Click to capture"}
        </span>
      </div>
      {showWarning && (
        <span className="agent-key-warning">
          Modifier keys alone may trigger during normal typing
        </span>
      )}
    </div>
  );
}

const MODIFIER_CODES = new Set([
  "ControlLeft", "ControlRight",
  "ShiftLeft", "ShiftRight",
  "AltLeft", "AltRight",
  "MetaLeft", "MetaRight",
]);

function isModifierCode(code: string): boolean {
  return MODIFIER_CODES.has(code);
}

function hasActiveModifiers(e: React.KeyboardEvent): boolean {
  return e.ctrlKey || e.shiftKey || e.altKey || e.metaKey;
}

function collectModifiers(e: React.KeyboardEvent): string[] {
  const mods: string[] = [];
  if (e.ctrlKey) mods.push("ctrl");
  if (e.shiftKey) mods.push("shift");
  if (e.altKey) mods.push("alt");
  if (e.metaKey) mods.push("meta");
  return mods;
}

const MODIFIER_DISPLAY: Record<string, string> = {
  ctrl: "Ctrl",
  shift: "Shift",
  alt: "Alt",
  meta: "Cmd",
};

/** Formats a PttKey for display, e.g. "Ctrl + Space". */
function formatPttKeyName(key: PttKey): string {
  const modLabels = key.modifiers.map((m) => MODIFIER_DISPLAY[m] ?? m);
  const keyLabel = formatKeyCode(key.code);
  return [...modLabels, keyLabel].join(" + ");
}

/** Converts a browser KeyboardEvent.code to a user-friendly display name. */
function formatKeyCode(code: string): string {
  if (code.startsWith("Key")) return code.slice(3);
  if (code.startsWith("Digit")) return code.slice(5);
  return code.replace(/(Left|Right)$/, " ($1)");
}
