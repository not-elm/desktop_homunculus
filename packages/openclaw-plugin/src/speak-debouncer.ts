import type { PluginLogger } from './deps.js';
import { errorMessage } from './util/error.js';

export interface SpeakPayload {
  agentId: string;
  text: string;
}

export interface SpeakDebouncer {
  push(key: string, agentId: string, text: string): void;
  forceFlush(key: string): Promise<void>;
}

export interface SpeakDebouncerOptions {
  /** Idle delay in milliseconds before the buffer is spoken. Default 300. */
  delayMs?: number;
  /** Emits the accumulated text. Any rejection is logged, never rethrown. */
  speak: (payload: SpeakPayload) => Promise<void>;
  logger: PluginLogger;
}

interface Entry {
  agentId: string;
  text: string;
  timer: ReturnType<typeof setTimeout>;
}

const DEFAULT_DELAY_MS = 300;

export function createSpeakDebouncer(opts: SpeakDebouncerOptions): SpeakDebouncer {
  const delayMs = opts.delayMs ?? DEFAULT_DELAY_MS;
  const buffers = new Map<string, Entry>();

  function scheduleFlush(key: string): ReturnType<typeof setTimeout> {
    return setTimeout(() => {
      void runFlush(key);
    }, delayMs);
  }

  async function runFlush(key: string): Promise<void> {
    const entry = buffers.get(key);
    if (!entry) return;
    buffers.delete(key);
    try {
      await opts.speak({ agentId: entry.agentId, text: entry.text });
    } catch (err) {
      opts.logger.warn(`[speak-debouncer] speak failed key=${key}: ${errorMessage(err)}`);
    }
  }

  return {
    push(key, agentId, text) {
      const existing = buffers.get(key);
      if (existing && existing.agentId === agentId) {
        clearTimeout(existing.timer);
        existing.text += text;
        existing.timer = scheduleFlush(key);
        return;
      }
      if (existing) clearTimeout(existing.timer);
      buffers.set(key, { agentId, text, timer: scheduleFlush(key) });
    },
    async forceFlush(key) {
      const entry = buffers.get(key);
      if (!entry) return;
      clearTimeout(entry.timer);
      await runFlush(key);
    },
  };
}
