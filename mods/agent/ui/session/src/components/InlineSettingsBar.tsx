import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@hmcs/ui";
import type { AgentSettings, PttKey } from "../hooks/useAgentSettings";
import { useModelOptions } from "../hooks/useModelOptions";
import type { SelectOption } from "../hooks/useModelOptions";
import { KeyCaptureField } from "./KeyCaptureField";

interface InlineSettingsBarProps {
  settings: AgentSettings;
  onSettingsChange: (settings: AgentSettings) => void;
  apiKey: string;
}

export function InlineSettingsBar({ settings, onSettingsChange, apiKey }: InlineSettingsBarProps) {
  const { options: modelOptions, loading: modelsLoading } = useModelOptions(settings.executor, apiKey);
  const showModelSelector = settings.executor !== "codex" && settings.executor !== "codex-appserver";

  function updatePttKey(key: PttKey | null) {
    onSettingsChange({ ...settings, pttKey: key });
  }

  function updateExecutor(value: string) {
    onSettingsChange({ ...settings, executor: value as AgentSettings["executor"] });
  }

  function updateModel(value: string) {
    onSettingsChange({ ...settings, claudeModel: value === "default" ? "" : value });
  }

  return (
    <div className="hud-inline-settings">
      <CompactKeyCapture pttKey={settings.pttKey} onChange={updatePttKey} />
      <span className="hud-inline-sep" />
      <CompactSelect value={settings.executor} onChange={updateExecutor} options={EXECUTOR_OPTIONS} />
      {showModelSelector && (
        <>
          <span className="hud-inline-sep" />
          <CompactSelect
            key={settings.executor}
            value={settings.claudeModel || "default"}
            onChange={updateModel}
            options={modelOptions}
            loading={modelsLoading}
          />
        </>
      )}
    </div>
  );
}

function CompactKeyCapture({ pttKey, onChange }: { pttKey: PttKey | null; onChange: (k: PttKey | null) => void }) {
  return (
    <div className="hud-inline-ptt">
      <KeyCaptureField
        label=""
        pttKey={pttKey}
        onChange={onChange}
      />
    </div>
  );
}

function CompactSelect({ value, onChange, options, loading }: {
  value: string;
  onChange: (v: string) => void;
  options: SelectOption[];
  loading?: boolean;
}) {
  return (
    <div className="hud-inline-select">
      <Select value={value} onValueChange={onChange} disabled={loading}>
        <SelectTrigger className="hud-inline-select-trigger">
          {loading ? <span className="hud-inline-loading">...</span> : <SelectValue />}
        </SelectTrigger>
        <SelectContent>
          {options.map((o) => (
            <SelectItem key={o.value} value={o.value}>{o.label}</SelectItem>
          ))}
        </SelectContent>
      </Select>
    </div>
  );
}

const EXECUTOR_OPTIONS: SelectOption[] = [
  { value: "sdk", label: "Claude SDK" },
  { value: "cli", label: "Claude CLI" },
  { value: "codex", label: "Codex" },
  { value: "codex-appserver", label: "Codex AppServer" },
];
