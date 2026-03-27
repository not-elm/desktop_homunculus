import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@hmcs/ui";
import type { AgentSettings } from "../hooks/useAgentSettings";

interface CodexTabProps {
  settings: AgentSettings;
  onSettingsChange: (settings: AgentSettings) => void;
}

export function CodexTab({ settings, onSettingsChange }: CodexTabProps) {
  return (
    <div className="settings-section">
      <label className="settings-label">
        Model
        <span className="settings-label-desc">
          Codex model for the agent to use
        </span>
        <Select
          value={settings.codexModel || "default"}
          onValueChange={(v) =>
            onSettingsChange({
              ...settings,
              codexModel: v === "default" ? "" : v,
            })
          }
        >
          <SelectTrigger>
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="default">Default (gpt-5.3-codex)</SelectItem>
          </SelectContent>
        </Select>
      </label>
    </div>
  );
}
