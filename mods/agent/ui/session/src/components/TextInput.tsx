import { useState, useCallback, useRef, type KeyboardEvent, type ChangeEvent, type CompositionEvent } from "react";
import { TextareaAutosize } from "@hmcs/ui";

interface TextInputProps {
  onSend: (text: string) => Promise<void>;
}

export function TextInput({ onSend }: TextInputProps) {
  const [value, setValue] = useState("");
  const [sending, setSending] = useState(false);
  const valueRef = useRef("");
  const isComposingRef = useRef(false);

  const syncValue = (v: string) => {
    valueRef.current = v;
    setValue(v);
  };

  const handleChange = useCallback((e: ChangeEvent<HTMLTextAreaElement>) => {
    syncValue(e.target.value);
  }, []);

  const handleCompositionStart = useCallback(() => {
    isComposingRef.current = true;
  }, []);

  const handleCompositionEnd = useCallback((e: CompositionEvent<HTMLTextAreaElement>) => {
    isComposingRef.current = false;
    syncValue(e.currentTarget.value);
  }, []);

  const handleSend = useCallback(async () => {
    const text = valueRef.current.trim();
    if (!text || sending) return;
    setSending(true);
    try {
      await onSend(text);
      syncValue("");
    } finally {
      setSending(false);
    }
  }, [sending, onSend]);

  const handleKeyDown = useCallback((e: KeyboardEvent<HTMLTextAreaElement>) => {
    if (e.key === "Enter" && !e.shiftKey && !isComposingRef.current) {
      e.preventDefault();
      handleSend();
    }
  }, [handleSend]);

  return (
    <div className="hud-text-input">
      <TextareaAutosize
        className="hud-text-input-field min-h-0 bg-transparent border-0 shadow-none ring-0 backdrop-blur-none p-0 focus-visible:ring-0 focus-visible:shadow-none"
        minRows={1}
        maxRows={5}
        placeholder="Type a message..."
        value={value}
        onChange={handleChange}
        onKeyDown={handleKeyDown}
        onCompositionStart={handleCompositionStart}
        onCompositionEnd={handleCompositionEnd}
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
