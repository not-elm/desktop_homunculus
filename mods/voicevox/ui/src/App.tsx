import {
  Button,
  cn,
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectLabel,
  SelectTrigger,
  SelectValue,
  Slider,
  Textarea,
  Toolbar,
} from '@hmcs/ui';
import { useState } from 'react';
import { Decorations } from './components/Decorations';
import type { VoicevoxSettings } from './hooks/useVoicevoxSettings';
import { useVoicevoxSettings } from './hooks/useVoicevoxSettings';

const panelClasses =
  'holo-noise relative box-border flex h-screen max-h-screen max-w-screen flex-col overflow-hidden rounded-xl bg-panel/92 animate-settings-in motion-reduce:animate-none motion-reduce:opacity-100';

const labelClasses = 'flex flex-col gap-1.5 text-xs uppercase tracking-[0.1em] text-primary/70';

const descriptionClasses =
  'text-[0.7rem] tracking-[0.04em] normal-case leading-[1.4] text-hud-text-subdued';

const sectionHeadingClasses = 'text-[0.7rem] uppercase tracking-[0.12em] text-hud-text-subdued';

const sliderBoxClasses =
  'flex flex-row items-center gap-3 rounded-md border border-primary/20 bg-input px-3 py-2.5';

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
      <div className={cn(panelClasses, 'items-center justify-center')}>
        <div className="text-xs uppercase tracking-[0.12em] text-primary/50 animate-[holo-corner-pulse_2s_ease-in-out_infinite] motion-reduce:animate-none motion-reduce:opacity-50">
          Loading...
        </div>
      </div>
    );
  }

  return (
    <div className={panelClasses}>
      <Decorations />

      <Toolbar title="VOICEVOX" onClose={handleClose}>
        {characterName && (
          <span className="font-mono text-[10px] tracking-[0.04em] text-primary/50">
            {characterName}
          </span>
        )}
        <ConnectionBadge connected={connected} />
      </Toolbar>

      <div className="no-scrollbar relative z-[7] flex min-h-0 flex-1 flex-col gap-6 overflow-y-auto p-5">
        {!connected ? (
          <DisconnectedView onRetry={handleRetry} />
        ) : speakers.length === 0 ? (
          <div className="flex flex-col items-center justify-center gap-4 py-10 text-center">
            <div className="text-sm text-holo-rose/80">No speakers found</div>
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
        onReset={handleReset}
        onSave={handleSave}
        saving={saving}
        saved={saved}
        disabled={!connected || speakers.length === 0 || invalidSpeaker}
      />
    </div>
  );
}

function ConnectionBadge({ connected }: { connected: boolean }) {
  return (
    <span
      className={cn(
        'inline-flex items-center gap-1.5 rounded-full border px-2 py-0.5 text-[10px] uppercase tracking-[0.06em]',
        connected
          ? 'border-success/25 bg-success/10 text-success/85'
          : 'border-holo-rose/25 bg-holo-rose/10 text-holo-rose/85',
      )}
    >
      <span
        className={cn(
          'h-1.5 w-1.5 rounded-full',
          connected
            ? 'bg-success [box-shadow:0_0_6px_var(--success)]'
            : 'bg-holo-rose [box-shadow:0_0_6px_var(--holo-rose)]',
        )}
      />
      {connected ? 'Connected' : 'Disconnected'}
    </span>
  );
}

function DisconnectedView({ onRetry }: { onRetry: () => void }) {
  return (
    <div className="flex flex-col items-center justify-center gap-4 py-10 text-center">
      <div className="text-sm text-holo-rose/80">Cannot connect to VOICEVOX</div>
      <Button variant="hud" size="hud" className="relative z-[7]" onClick={onRetry}>
        Retry
      </Button>
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
        <div className="relative z-[7] rounded-md border border-holo-amber/20 bg-holo-amber/10 px-3 py-2 text-xs text-holo-amber/80">
          Previous speaker is unavailable. Please select a new one.
        </div>
      )}

      <label className={labelClasses} htmlFor="voicevox-speaker-select">
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

      <section className="flex flex-col gap-3">
        <h2 className={sectionHeadingClasses}>Voice Parameters</h2>
        {PARAMS.map((param) => (
          <ParameterField
            key={param.key}
            param={param}
            value={settings[param.key] as number}
            onChange={(value) => onSettingsChange({ ...settings, [param.key]: value })}
          />
        ))}
      </section>

      <section className="flex flex-col gap-3">
        <h2 className={sectionHeadingClasses}>Speech Test</h2>
        <SpeechTest speaking={speaking} disabled={disabled} onSpeak={onSpeak} />
      </section>
    </>
  );
}

function ParameterField({
  param,
  value,
  onChange,
}: {
  param: { key: string; label: string; desc: string; min: number; max: number; step: number };
  value: number;
  onChange: (value: number) => void;
}) {
  return (
    <label className={labelClasses}>
      {param.label}
      <div className={sliderBoxClasses}>
        <Slider
          className="flex-1"
          min={param.min}
          max={param.max}
          step={param.step}
          value={[value]}
          onValueChange={(arr) => onChange(arr[0])}
        />
        <span className="min-w-[3.5em] text-right font-mono text-xs text-foreground">
          {value.toFixed(2)}
        </span>
      </div>
      <span className={descriptionClasses}>{param.desc}</span>
    </label>
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
    <div className="relative z-[7] flex flex-col gap-2">
      <Textarea
        className="resize-none"
        rows={3}
        placeholder="Enter text to test..."
        value={text}
        onChange={(e) => setText(e.target.value)}
        disabled={disabled || speaking}
      />
      <div className="flex justify-end">
        <Button
          variant="hud"
          size="hud"
          disabled={disabled || speaking || text.trim().length === 0}
          onClick={() => onSpeak(text.trim())}
        >
          {speaking ? 'Speaking...' : '▶ Speak'}
        </Button>
      </div>
    </div>
  );
}

function Footer({
  onReset,
  onSave,
  saving,
  saved,
  disabled,
}: {
  onReset: () => void;
  onSave: () => void;
  saving: boolean;
  saved: boolean;
  disabled: boolean;
}) {
  return (
    <div className="relative z-[7] flex shrink-0 justify-end gap-2 border-t border-primary/12 bg-primary/4 px-3.5 py-2">
      <Button variant="hud-ghost" size="hud" onClick={onReset}>
        Reset
      </Button>
      <Button
        variant={saved ? 'hud-success' : 'hud'}
        size="hud"
        onClick={onSave}
        disabled={saving || disabled}
      >
        {saving ? 'Saving...' : saved ? 'Saved!' : 'Save'}
      </Button>
    </div>
  );
}
