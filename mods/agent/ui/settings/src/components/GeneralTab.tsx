import { useCallback, useEffect, useRef } from "react";
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
  const pttFieldRef = useRef<HTMLDivElement>(null);
  const shouldScrollRef = useRef(false);

  const scrollToPttField = useCallback(() => {
    if (!shouldScrollRef.current) return;
    shouldScrollRef.current = false;
    pttFieldRef.current?.scrollIntoView({ behavior: "smooth", block: "nearest" });
  }, []);

  useEffect(() => {
    if (settings.listeningMode === "ptt") {
      shouldScrollRef.current = true;
      // Fallback for prefers-reduced-motion where animation is disabled
      const mq = window.matchMedia("(prefers-reduced-motion: reduce)");
      if (mq.matches) {
        requestAnimationFrame(() => scrollToPttField());
      }
    }
  }, [settings.listeningMode, scrollToPttField]);

  function handlePttAnimationEnd(e: React.AnimationEvent<HTMLDivElement>) {
    if (e.animationName === "ptt-reveal" && e.target === e.currentTarget) {
      scrollToPttField();
    }
  }

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
      <div className="agent-listening-group">
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
          <div
            ref={pttFieldRef}
            className="agent-listening-ptt"
            onAnimationEnd={handlePttAnimationEnd}
          >
            <KeyCaptureField
              label="Push-to-Talk Key"
              description="Press the key to capture it"
              pttKey={settings.pttKey}
              onChange={(key) => update("pttKey", key)}
            />
          </div>
        )}
      </div>

      <div className="agent-divider" />

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
    </div>
  );
}
