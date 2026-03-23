import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@hmcs/ui";
import type { AgentSettings } from "../hooks/useAgentSettings";
import { PhraseListField } from "./PhraseListField";
import { DirectoryListField } from "./DirectoryListField";
import { KeyCaptureField } from "./KeyCaptureField";

interface GeneralTabProps {
  settings: AgentSettings;
  onSettingsChange: (settings: AgentSettings) => void;
}

export function GeneralTab({ settings, onSettingsChange }: GeneralTabProps) {
  function update<K extends keyof AgentSettings>(
    key: K,
    value: AgentSettings[K],
  ) {
    onSettingsChange({ ...settings, [key]: value });
  }

  function addPhrase(key: keyof Pick<AgentSettings, "wakeWords" | "shutdownWords" | "greetingPhrases" | "completionPhrases" | "errorPhrases">, phrase: string) {
    update(key, [...settings[key], phrase]);
  }

  function removePhrase(key: keyof Pick<AgentSettings, "wakeWords" | "shutdownWords" | "greetingPhrases" | "completionPhrases" | "errorPhrases">, index: number) {
    update(key, settings[key].filter((_, i) => i !== index));
  }

  function addDirectory(path: string) {
    const { paths, default: def } = settings.workingDirectories;
    update("workingDirectories", { paths: [...paths, path], default: def });
  }

  function removeDirectory(index: number) {
    const { paths, default: def } = settings.workingDirectories;
    const newPaths = paths.filter((_, i) => i !== index);
    const newDefault = def >= newPaths.length ? Math.max(0, newPaths.length - 1) : def;
    update("workingDirectories", { paths: newPaths, default: newDefault });
  }

  function setDefaultDirectory(index: number) {
    update("workingDirectories", {
      ...settings.workingDirectories,
      default: index,
    });
  }

  return (
    <div className="settings-section">
      <PhraseListField
        label="Wake Words"
        description="Phrases that activate the agent"
        phrases={settings.wakeWords}
        onAdd={(p) => addPhrase("wakeWords", p)}
        onRemove={(i) => removePhrase("wakeWords", i)}
      />

      <PhraseListField
        label="Shutdown Words"
        description="Phrases that deactivate the agent"
        phrases={settings.shutdownWords}
        onAdd={(p) => addPhrase("shutdownWords", p)}
        onRemove={(i) => removePhrase("shutdownWords", i)}
      />

      <div className="agent-divider" />

      <PhraseListField
        label="Greeting Phrases"
        description="Said when the agent activates"
        phrases={settings.greetingPhrases}
        onAdd={(p) => addPhrase("greetingPhrases", p)}
        onRemove={(i) => removePhrase("greetingPhrases", i)}
      />

      <PhraseListField
        label="Completion Phrases"
        description="Said when a task finishes"
        phrases={settings.completionPhrases}
        onAdd={(p) => addPhrase("completionPhrases", p)}
        onRemove={(i) => removePhrase("completionPhrases", i)}
      />

      <PhraseListField
        label="Error Phrases"
        description="Said when an error occurs"
        phrases={settings.errorPhrases}
        onAdd={(p) => addPhrase("errorPhrases", p)}
        onRemove={(i) => removePhrase("errorPhrases", i)}
      />

      <div className="agent-divider" />

      <DirectoryListField
        label="Working Directories"
        description="Directories available to the agent. Select default with radio button."
        paths={settings.workingDirectories.paths}
        defaultIndex={settings.workingDirectories.default}
        onAdd={addDirectory}
        onRemove={removeDirectory}
        onSetDefault={setDefaultDirectory}
      />

      <div className="agent-divider" />

      <label className="settings-label">
        Listening Mode
        <Select
          value={settings.listeningMode}
          onValueChange={(v) =>
            update("listeningMode", v as "ptt" | "always-on")
          }
        >
          <SelectTrigger>
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="always-on">Always On</SelectItem>
            <SelectItem value="ptt">Push to Talk</SelectItem>
          </SelectContent>
        </Select>
      </label>

      {settings.listeningMode === "ptt" && (
        <KeyCaptureField
          label="Push-to-Talk Key"
          description="Press the key to capture it"
          keyCode={settings.pttKeycode}
          onChange={(code) => update("pttKeycode", code)}
        />
      )}
    </div>
  );
}
