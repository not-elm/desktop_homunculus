import { useState, useCallback, type KeyboardEvent } from "react";

interface TextInputProps {
  onSend: (text: string) => Promise<void>;
}

export function TextInput({ onSend }: TextInputProps) {
  const [value, setValue] = useState("");
  const [sending, setSending] = useState(false);

  const handleSend = useCallback(async () => {
    const text = value.trim();
    if (!text || sending) return;
    setSending(true);
    try {
      await onSend(text);
      setValue("");
    } finally {
      setSending(false);
    }
  }, [value, sending, onSend]);

  const handleKeyDown = useCallback((e: KeyboardEvent<HTMLInputElement>) => {
    if (e.key === "Enter" && !e.shiftKey) {
      e.preventDefault();
      handleSend();
    }
  }, [handleSend]);

  return (
    <div className="hud-text-input">
      <input
        type="text"
        className="hud-text-input-field"
        placeholder="Type a message..."
        value={value}
        onChange={(e) => setValue(e.target.value)}
        onKeyDown={handleKeyDown}
        disabled={sending}
      />
      <button
        className="hud-text-input-send"
        onClick={handleSend}
        disabled={sending || !value.trim()}
        title="Send"
      >
        <SendIcon />
      </button>
    </div>
  );
}

function SendIcon() {
  return (
    <svg width="12" height="12" viewBox="0 0 12 12" fill="none">
      <path d="M2 10L10 6L2 2V5L7 6L2 7V10Z" fill="currentColor" />
    </svg>
  );
}
