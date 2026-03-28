import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@hmcs/ui";
import type { AgentSettings, PttKey } from "../hooks/useAgentSettings";
import { KeyCaptureField } from "./KeyCaptureField";

interface InlineSettingsBarProps {
  settings: AgentSettings;
  onSettingsChange: (settings: AgentSettings) => void;
}

export function InlineSettingsBar({ settings, onSettingsChange }: InlineSettingsBarProps) {
  function updatePttKey(key: PttKey | null) {
    onSettingsChange({ ...settings, pttKey: key });
  }

  function updateExecutor(value: string) {
    onSettingsChange({ ...settings, executor: value as AgentSettings["executor"] });
  }

  function updateModel(value: string) {
    const key = settings.executor === "codex" ? "codexModel" : "claudeModel";
    onSettingsChange({ ...settings, [key]: value === "default" ? "" : value });
  }

  const currentModel = settings.executor === "codex" ? settings.codexModel : settings.claudeModel;

  return (
    <div className="hud-inline-settings">
      <CompactKeyCapture pttKey={settings.pttKey} onChange={updatePttKey} />
      <span className="hud-inline-sep" />
      <CompactSelect value={settings.executor} onChange={updateExecutor} options={EXECUTOR_OPTIONS} />
      <span className="hud-inline-sep" />
      <CompactSelect value={currentModel || "default"} onChange={updateModel} options={MODEL_OPTIONS} />
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

interface SelectOption { value: string; label: string }

function CompactSelect({ value, onChange, options }: { value: string; onChange: (v: string) => void; options: SelectOption[] }) {
  return (
    <div className="hud-inline-select">
      <Select value={value} onValueChange={onChange}>
        <SelectTrigger className="hud-inline-select-trigger">
          <SelectValue />
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
];

const MODEL_OPTIONS: SelectOption[] = [
  { value: "default", label: "Default" },
  { value: "claude-sonnet-4-6", label: "Sonnet 4.6" },
  { value: "claude-opus-4-6", label: "Opus 4.6" },
  { value: "claude-haiku-4-5", label: "Haiku 4.5" },
];
