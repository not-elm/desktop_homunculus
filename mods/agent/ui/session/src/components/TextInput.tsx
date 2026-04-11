import { TextareaAutosize } from '@hmcs/ui';
import {
  type ChangeEvent,
  type CompositionEvent,
  type KeyboardEvent,
  useCallback,
  useRef,
  useState,
} from 'react';

interface TextInputProps {
  onSend: (text: string) => Promise<void>;
  isInterruptible?: boolean;
  onInterrupt?: () => void;
}

export function TextInput({ onSend, isInterruptible, onInterrupt }: TextInputProps) {
  const [value, setValue] = useState('');
  const [sending, setSending] = useState(false);
  const valueRef = useRef('');
  const isComposingRef = useRef(false);

  const syncValue = (v: string) => {
    valueRef.current = v;
    setValue(v);
  };

  const handleChange = useCallback(
    (e: ChangeEvent<HTMLTextAreaElement>) => {
      syncValue(e.target.value);
    },
    [syncValue],
  );

  const handleCompositionStart = useCallback(() => {
    isComposingRef.current = true;
  }, []);

  const handleCompositionEnd = useCallback(
    (e: CompositionEvent<HTMLTextAreaElement>) => {
      isComposingRef.current = false;
      syncValue(e.currentTarget.value);
    },
    [syncValue],
  );

  const handleSend = useCallback(async () => {
    const text = valueRef.current.trim();
    if (!text || sending) return;
    setSending(true);
    try {
      await onSend(text);
      syncValue('');
    } finally {
      setSending(false);
    }
  }, [sending, onSend, syncValue]);

  const handleKeyDown = useCallback(
    (e: KeyboardEvent<HTMLTextAreaElement>) => {
      if (e.key === 'Enter' && !e.shiftKey && !isComposingRef.current) {
        e.preventDefault();
        handleSend();
      }
    },
    [handleSend],
  );

  return (
    <div className="hud-text-input">
      <TextareaAutosize
        className="hud-text-input-field min-h-0 bg-transparent border-0 shadow-none ring-0 backdrop-blur-none p-0 focus-visible:ring-0 focus-visible:shadow-none focus-visible:animate-none"
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
      {isInterruptible ? (
        <button
          className="hud-text-input-send hud-text-input-send--interrupt"
          onClick={onInterrupt}
          title="Interrupt"
        >
          <InterruptIcon />
        </button>
      ) : (
        <button
          className="hud-text-input-send"
          onClick={handleSend}
          disabled={sending || !value.trim()}
          title="Send"
        >
          <SendIcon />
        </button>
      )}
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

function InterruptIcon() {
  return (
    <svg width="10" height="10" viewBox="0 0 10 10" fill="none">
      <rect x="2" y="2" width="6" height="6" rx="1" fill="currentColor" />
    </svg>
  );
}
