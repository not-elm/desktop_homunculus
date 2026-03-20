import { useState } from "react";

const LANGUAGES = [
  { code: "en", label: "EN" },
  { code: "ja", label: "JA" },
  { code: "ko", label: "KO" },
  { code: "zh", label: "ZH" },
  { code: "es", label: "ES" },
  { code: "fr", label: "FR" },
  { code: "de", label: "DE" },
  { code: "pt", label: "PT" },
  { code: "ru", label: "RU" },
  { code: "ar", label: "AR" },
];

interface BasicTabProps {
  metadata: string;
  names: Record<string, string>;
  scale: number;
  onNameSave: (lang: string, name: string) => void;
  onNameDelete: (lang: string) => void;
  onNamesChange: (names: Record<string, string>) => void;
  onScaleChange: (scale: number) => void;
}

export function BasicTab({
  metadata,
  names,
  scale,
  onNameSave,
  onNameDelete,
  onNamesChange,
  onScaleChange,
}: BasicTabProps) {
  const [showLangPicker, setShowLangPicker] = useState(false);

  const usedLanguages = Object.keys(names);
  const availableLanguages = LANGUAGES.filter(
    (lang) => !usedLanguages.includes(lang.code),
  );

  return (
    <div className="settings-section">
      <div className="settings-label">Name</div>
      <div style={{ display: "flex", flexDirection: "column", gap: 8 }}>
        {usedLanguages.map((lang) => (
          <NameRow
            key={lang}
            lang={lang}
            value={names[lang]}
            placeholder={metadata}
            onSave={onNameSave}
            onDelete={onNameDelete}
            onChange={(value) =>
              onNamesChange({ ...names, [lang]: value })
            }
          />
        ))}

        <AddLanguageButton
          show={showLangPicker}
          available={availableLanguages}
          onToggle={() => setShowLangPicker((prev) => !prev)}
          onSelect={(code) => {
            onNamesChange({ ...names, [code]: "" });
            setShowLangPicker(false);
          }}
        />
      </div>

      <label className="settings-label">
        Scale
        <div className="settings-slider-row">
          <input
            type="range"
            className="settings-slider"
            min={0.1}
            max={3}
            step={0.05}
            value={scale}
            onChange={(e) => onScaleChange(parseFloat(e.target.value))}
          />
          <span className="settings-slider-value">{scale.toFixed(2)}</span>
        </div>
      </label>
    </div>
  );
}

interface NameRowProps {
  lang: string;
  value: string;
  placeholder: string;
  onSave: (lang: string, name: string) => void;
  onDelete: (lang: string) => void;
  onChange: (value: string) => void;
}

function NameRow({ lang, value, placeholder, onSave, onDelete, onChange }: NameRowProps) {
  return (
    <div style={{ display: "flex", alignItems: "center", gap: 8 }}>
      <span
        style={{
          fontSize: "0.75rem",
          fontWeight: 600,
          letterSpacing: "0.08em",
          color: "oklch(0.72 0.14 192 / 0.7)",
          minWidth: "2.5em",
          textTransform: "uppercase",
        }}
      >
        {lang.toUpperCase()}
      </span>
      <input
        type="text"
        className="settings-input"
        style={{ flex: 1 }}
        value={value}
        placeholder={placeholder}
        onChange={(e) => onChange(e.target.value)}
        onBlur={() => onSave(lang, value)}
      />
      <button
        type="button"
        onClick={() => onDelete(lang)}
        style={{
          background: "transparent",
          border: "1px solid oklch(0.7 0.16 350 / 0.3)",
          borderRadius: 4,
          color: "oklch(0.7 0.16 350 / 0.7)",
          cursor: "pointer",
          fontSize: "0.85rem",
          lineHeight: 1,
          padding: "4px 8px",
          transition: "color 180ms ease, border-color 180ms ease",
        }}
      >
        &times;
      </button>
    </div>
  );
}

interface AddLanguageButtonProps {
  show: boolean;
  available: { code: string; label: string }[];
  onToggle: () => void;
  onSelect: (code: string) => void;
}

function AddLanguageButton({ show, available, onToggle, onSelect }: AddLanguageButtonProps) {
  if (available.length === 0) return null;

  return (
    <div style={{ position: "relative" }}>
      <button
        type="button"
        onClick={onToggle}
        style={{
          background: "transparent",
          border: "1px dashed oklch(0.72 0.14 192 / 0.3)",
          borderRadius: 6,
          color: "oklch(0.72 0.14 192 / 0.6)",
          cursor: "pointer",
          fontSize: "0.78rem",
          letterSpacing: "0.06em",
          padding: "6px 14px",
          transition: "color 180ms ease, border-color 180ms ease",
          width: "100%",
        }}
      >
        + Add Language
      </button>

      {show && (
        <div
          style={{
            marginTop: 4,
            display: "flex",
            flexWrap: "wrap",
            gap: 4,
          }}
        >
          {available.map((lang) => (
            <button
              key={lang.code}
              type="button"
              onClick={() => onSelect(lang.code)}
              style={{
                background: "oklch(0.12 0.01 250 / 0.8)",
                border: "1px solid oklch(0.72 0.14 192 / 0.2)",
                borderRadius: 4,
                color: "oklch(0.72 0.14 192 / 0.8)",
                cursor: "pointer",
                fontSize: "0.72rem",
                fontWeight: 600,
                letterSpacing: "0.08em",
                padding: "4px 10px",
                transition: "background 180ms ease, border-color 180ms ease",
              }}
            >
              {lang.label}
            </button>
          ))}
        </div>
      )}
    </div>
  );
}
