import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectLabel,
  SelectTrigger,
  SelectValue,
  Textarea,
} from '@hmcs/ui';
import { useState } from 'react';
import type { VoicevoxSettings } from './hooks/useVoicevoxSettings';
import { useVoicevoxSettings } from './hooks/useVoicevoxSettings';

const PARAMS: {
  key: keyof VoicevoxSettings;
  label: string;
  desc: string;
  min: number;
  max: number;
  step: number;
}[] = [
  {
    key: 'speedScale',
    label: 'Speed',
    desc: 'Speech speed',
    min: 0.5,
    max: 2.0,
    step: 0.05,
  },
  {
    key: 'pitchScale',
    label: 'Pitch',
    desc: 'Voice pitch (0 = baseline)',
    min: -0.15,
    max: 0.15,
    step: 0.01,
  },
  {
    key: 'intonationScale',
    label: 'Intonation',
    desc: 'Intonation strength',
    min: 0.0,
    max: 2.0,
    step: 0.05,
  },
  {
    key: 'volumeScale',
    label: 'Volume',
    desc: 'Volume level',
    min: 0.0,
    max: 2.0,
    step: 0.05,
  },
  {
    key: 'pauseLength',
    label: 'Pause Length',
    desc: 'Pause duration at punctuation marks',
    min: 0,
    max: 2.0,
    step: 0.01,
  },
  {
    key: 'prePhonemeLength',
    label: 'Pre Phoneme Length',
    desc: 'Silence before speech starts',
    min: 0,
    max: 1.5,
    step: 0.01,
  },
  {
    key: 'postPhonemeLength',
    label: 'Post Phoneme Length',
    desc: 'Silence after speech ends',
    min: 0,
    max: 1.5,
    step: 0.01,
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
    personaId,
    speaking,
    handleSave,
    handleReset,
    handleClose,
    handleRetry,
    handleSpeak,
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
            <div className="voicevox-error-text">No speakers found</div>
          </div>
        ) : (
          <SettingsForm
            speakers={speakers}
            settings={settings}
            onSettingsChange={setSettings}
            invalidSpeaker={invalidSpeaker}
            speaking={speaking}
            disabled={!connected || speakers.length === 0 || invalidSpeaker || !personaId}
            onSpeak={handleSpeak}
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

function Header({ name, connected }: { name: string; connected: boolean }) {
  return (
    <div className="settings-header">
      <h1 className="settings-title">VOICEVOX</h1>
      <span className="settings-entity-name">{name}</span>
      <span
        className={`voicevox-status ${connected ? 'voicevox-status--connected' : 'voicevox-status--disconnected'}`}
      >
        <span className="voicevox-status-dot" />
        {connected ? 'Connected' : 'Disconnected'}
      </span>
    </div>
  );
}

function DisconnectedView({ onRetry }: { onRetry: () => void }) {
  return (
    <div className="voicevox-error">
      <div className="voicevox-error-text">Cannot connect to VOICEVOX</div>
      <button type="button" className="voicevox-error-retry" onClick={onRetry}>
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
  speaking,
  disabled,
  onSpeak,
}: {
  speakers: { name: string; styles: { name: string; id: number }[] }[];
  settings: VoicevoxSettings;
  onSettingsChange: (s: VoicevoxSettings) => void;
  invalidSpeaker: boolean;
  speaking: boolean;
  disabled: boolean;
  onSpeak: (text: string) => void;
}) {
  return (
    <>
      {invalidSpeaker && (
        <div className="voicevox-warning">
          Previous speaker is unavailable. Please select a new one.
        </div>
      )}

      <label className="settings-label" htmlFor="voicevox-speaker-select">
        Speaker
        <Select
          value={settings.speakerId === -1 ? undefined : String(settings.speakerId)}
          onValueChange={(value) => onSettingsChange({ ...settings, speakerId: Number(value) })}
        >
          <SelectTrigger id="voicevox-speaker-select" className="w-full">
            <SelectValue placeholder="— Select a speaker —" />
          </SelectTrigger>
          <SelectContent>
            {speakers.map((speaker) => (
              <SelectGroup key={speaker.name}>
                <SelectLabel>{speaker.name}</SelectLabel>
                {speaker.styles.map((style) => (
                  <SelectItem key={style.id} value={String(style.id)}>
                    {speaker.name}-{style.name}
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

      <div className="voicevox-divider" />
      <SpeechTest speaking={speaking} disabled={disabled} onSpeak={onSpeak} />
    </>
  );
}

function SpeechTest({
  speaking,
  disabled,
  onSpeak,
}: {
  speaking: boolean;
  disabled: boolean;
  onSpeak: (text: string) => void;
}) {
  const [text, setText] = useState('');

  return (
    <div className="voicevox-speech-test">
      <div className="voicevox-section-title">Speech Test</div>
      <Textarea
        className="resize-none"
        rows={3}
        placeholder="Enter text to test..."
        value={text}
        onChange={(e) => setText(e.target.value)}
        disabled={disabled || speaking}
      />
      <div className="voicevox-speech-test-actions">
        <button
          type="button"
          className="voicevox-speech-test-btn"
          disabled={disabled || speaking || text.trim().length === 0}
          onClick={() => onSpeak(text.trim())}
        >
          {speaking ? 'Speaking...' : '▶ Speak'}
        </button>
      </div>
    </div>
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
      <button type="button" className="settings-close" onClick={onClose}>
        Close
      </button>
      <button type="button" className="settings-close" onClick={onReset}>
        Reset
      </button>
      <button
        type="button"
        className={`settings-save ${saved ? 'settings-save--success' : ''}`}
        onClick={onSave}
        disabled={saving || disabled}
      >
        {saving ? 'Saving...' : saved ? 'Saved!' : 'Save'}
      </button>
    </div>
  );
}
