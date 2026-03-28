import { useEffect, useRef, useState } from "react";
import type { AgentSettings } from "./useAgentSettings";

export interface SelectOption {
  value: string;
  label: string;
}

export function useModelOptions(
  executor: AgentSettings["executor"],
  apiKey: string,
): { options: SelectOption[]; loading: boolean } {
  const [options, setOptions] = useState<SelectOption[]>(() => fallbackOptions(executor));
  const [loading, setLoading] = useState(false);
  const cacheRef = useRef(new Map<string, SelectOption[]>());

  useEffect(() => {
    if (executor === "codex") {
      setOptions([]);
      setLoading(false);
      return;
    }

    if (!apiKey) {
      setOptions(CLAUDE_FALLBACK);
      setLoading(false);
      return;
    }

    const cacheKey = `claude:${apiKey}`;
    const cached = cacheRef.current.get(cacheKey);
    if (cached) {
      setOptions(cached);
      setLoading(false);
      return;
    }

    let cancelled = false;
    setLoading(true);

    fetchAnthropicModels(apiKey).then((result) => {
      if (cancelled) return;
      cacheRef.current.set(cacheKey, result);
      setOptions(result);
      setLoading(false);
    }).catch(() => {
      if (cancelled) return;
      setOptions(CLAUDE_FALLBACK);
      setLoading(false);
    });

    return () => { cancelled = true; };
  }, [executor, apiKey]);

  return { options, loading };
}

async function fetchAnthropicModels(apiKey: string): Promise<SelectOption[]> {
  const res = await fetch("https://api.anthropic.com/v1/models?limit=100", {
    headers: {
      "x-api-key": apiKey,
      "anthropic-version": "2023-06-01",
    },
  });
  if (!res.ok) throw new Error(`API error: ${res.status}`);

  const body = await res.json() as {
    data: { id: string; display_name: string }[];
  };

  const models = body.data
    .map((m) => ({ value: m.id, label: m.display_name || m.id }))
    .sort((a, b) => a.label.localeCompare(b.label));

  return [{ value: "default", label: "Default" }, ...models];
}

function fallbackOptions(executor: AgentSettings["executor"]): SelectOption[] {
  return executor === "codex" ? [] : CLAUDE_FALLBACK;
}

const CLAUDE_FALLBACK: SelectOption[] = [
  { value: "default", label: "Default" },
  { value: "claude-sonnet-4-6", label: "Claude Sonnet 4.6" },
  { value: "claude-opus-4-6", label: "Claude Opus 4.6" },
  { value: "claude-haiku-4-5", label: "Claude Haiku 4.5" },
];
