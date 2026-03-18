import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectLabel,
  SelectTrigger,
  SelectValue,
} from "@hmcs/ui";
import { useVoicevoxSettings } from "./hooks/useVoicevoxSettings";
import type { VoicevoxSettings } from "./hooks/useVoicevoxSettings";

const PARAMS: {
  key: keyof VoicevoxSettings;
  label: string;
  desc: string;
  min: number;
  max: number;
  step: number;
}[] = [
  {
    key: "speedScale",
    label: "Speed",
    desc: "読み上げの速さ",
    min: 0.5,
    max: 2.0,
    step: 0.05,
  },
  {
    key: "pitchScale",
    label: "Pitch",
    desc: "声の高さ（0が基準）",
    min: -0.15,
    max: 0.15,
    step: 0.01,
  },
  {
    key: "intonationScale",
    label: "Intonation",
    desc: "抑揚の強さ",
    min: 0.0,
    max: 2.0,
    step: 0.05,
  },
  {
    key: "volumeScale",
    label: "Volume",
    desc: "音量",
    min: 0.0,
    max: 2.0,
    step: 0.05,
  },
];

export function App() {
  const {
    loading,
    connected,
    speakers,
    settings,
    setSettings,
    saving,
    saved,
    characterName,
    invalidSpeaker,
    handleSave,
    handleReset,
    handleClose,
    handleRetry,
  } = useVoicevoxSettings();

  if (loading) {
    return (
      <div className="settings-panel settings-loading">
        <div className="settings-loading-text">Loading...</div>
      </div>
    );
  }

  return (
    <div className="settings-panel holo-refract-border holo-noise">
      <Decorations />
      <Header name={characterName} connected={connected} />

      <div className="settings-content">
        {!connected ? (
          <DisconnectedView onRetry={handleRetry} />
        ) : speakers.length === 0 ? (
          <div className="voicevox-error">
            <div className="voicevox-error-text">話者が見つかりません</div>
          </div>
        ) : (
          <SettingsForm
            speakers={speakers}
            settings={settings}
            onSettingsChange={setSettings}
            invalidSpeaker={invalidSpeaker}
          />
        )}
      </div>

      <Footer
        onClose={handleClose}
        onReset={handleReset}
        onSave={handleSave}
        saving={saving}
        saved={saved}
        disabled={!connected || speakers.length === 0 || invalidSpeaker}
      />
    </div>
  );
}

function Decorations() {
  return (
    <>
      <div className="settings-highlight" />
      <div className="settings-bottom-line" />
      <div className="settings-scanline" />
      <span className="settings-corner settings-corner--tl" />
      <span className="settings-corner settings-corner--tr" />
      <span className="settings-corner settings-corner--bl" />
      <span className="settings-corner settings-corner--br" />
    </>
  );
}

function Header({
  name,
  connected,
}: {
  name: string;
  connected: boolean;
}) {
  return (
    <div className="settings-header">
      <h1 className="settings-title">VOICEVOX</h1>
      <span className="settings-entity-name">{name}</span>
      <span
        className={`voicevox-status ${connected ? "voicevox-status--connected" : "voicevox-status--disconnected"}`}
      >
        <span className="voicevox-status-dot" />
        {connected ? "Connected" : "Disconnected"}
      </span>
    </div>
  );
}

function DisconnectedView({ onRetry }: { onRetry: () => void }) {
  return (
    <div className="voicevox-error">
      <div className="voicevox-error-text">
        VOICEVOX に接続できません
      </div>
      <button className="voicevox-error-retry" onClick={onRetry}>
        Retry
      </button>
    </div>
  );
}

function SettingsForm({
  speakers,
  settings,
  onSettingsChange,
  invalidSpeaker,
}: {
  speakers: { name: string; styles: { name: string; id: number }[] }[];
  settings: VoicevoxSettings;
  onSettingsChange: (s: VoicevoxSettings) => void;
  invalidSpeaker: boolean;
}) {
  return (
    <>
      {invalidSpeaker && (
        <div className="voicevox-warning">
          以前の話者は利用できません。新しい話者を選択してください。
        </div>
      )}

      <label className="settings-label">
        Speaker
        <Select
          value={
            settings.speakerId === -1 ? undefined : String(settings.speakerId)
          }
          onValueChange={(value) =>
            onSettingsChange({ ...settings, speakerId: Number(value) })
          }
        >
          <SelectTrigger className="w-full">
            <SelectValue placeholder="— 話者を選択 —" />
          </SelectTrigger>
          <SelectContent>
            {speakers.map((speaker) => (
              <SelectGroup key={speaker.name}>
                <SelectLabel>{speaker.name}</SelectLabel>
                {speaker.styles.map((style) => (
                  <SelectItem key={style.id} value={String(style.id)}>
                    {style.name}
                  </SelectItem>
                ))}
              </SelectGroup>
            ))}
          </SelectContent>
        </Select>
      </label>

      <div className="voicevox-divider" />
      <div className="voicevox-section-title">Voice Parameters</div>

      {PARAMS.map((param) => (
        <label key={param.key} className="settings-label">
          {param.label}
          <div className="settings-slider-row">
            <input
              type="range"
              className="settings-slider"
              min={param.min}
              max={param.max}
              step={param.step}
              value={settings[param.key] as number}
              onChange={(e) =>
                onSettingsChange({
                  ...settings,
                  [param.key]: parseFloat(e.target.value),
                })
              }
            />
            <span className="settings-slider-value">
              {(settings[param.key] as number).toFixed(2)}
            </span>
          </div>
          <div className="voicevox-param-desc">{param.desc}</div>
        </label>
      ))}
    </>
  );
}

function Footer({
  onClose,
  onReset,
  onSave,
  saving,
  saved,
  disabled,
}: {
  onClose: () => void;
  onReset: () => void;
  onSave: () => void;
  saving: boolean;
  saved: boolean;
  disabled: boolean;
}) {
  return (
    <div className="settings-footer">
      <button className="settings-close" onClick={onClose}>
        Close
      </button>
      <button className="settings-close" onClick={onReset}>
        Reset
      </button>
      <button
        className={`settings-save ${saved ? "settings-save--success" : ""}`}
        onClick={onSave}
        disabled={saving || disabled}
      >
        {saving ? "Saving..." : saved ? "Saved!" : "Save"}
      </button>
    </div>
  );
}
