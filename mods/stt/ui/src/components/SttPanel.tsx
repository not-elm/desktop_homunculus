import { useCallback } from "react";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@hmcs/ui";
import { audio, Webview } from "@hmcs/sdk";
import type { stt as sttTypes } from "@hmcs/sdk";
import { useStt, type ModelCardState } from "../hooks/useStt";

type SttState = sttTypes.SttState;

function statusLabel(state: SttState): string {
  switch (state.state) {
    case "idle":
      return "IDLE";
    case "loading":
      return "INITIALIZING...";
    case "listening":
      return "LISTENING";
    case "error":
      return `ERROR: ${state.message}`;
  }
}

function dotClass(state: SttState): string {
  switch (state.state) {
    case "loading":
      return "stt-dot--loading";
    case "listening":
      return "stt-dot--listening";
    case "error":
      return "stt-dot--error";
    default:
      return "";
  }
}

const DOTS = Array.from({ length: 15 }, (_, i) => i);

function LanguageSelector({
  language,
  setLanguage,
  languages,
}: {
  language: string;
  setLanguage: (lang: string) => void;
  languages: sttTypes.LanguageEntry[];
}) {
  return (
    <div className="settings-label">
      Language
      <Select value={language} onValueChange={setLanguage}>
        <SelectTrigger className="w-full">
          <SelectValue />
        </SelectTrigger>
        <SelectContent>
          {languages.map((lang) => (
            <SelectItem key={lang.code} value={lang.code}>
              {lang.label}
            </SelectItem>
          ))}
        </SelectContent>
      </Select>
    </div>
  );
}

export function SttPanel() {
  const {
    sttState,
    models,
    selectedModel,
    selectModel,
    downloadModel,
    cancelDownload,
    language,
    setLanguage,
    languages,
    startSession,
    stopSession,
    errorMessage,
  } = useStt();

  const handleClose = useCallback(() => {
    audio.se.play("se:close");
    Webview.current()?.close();
  }, []);

  const isListening = sttState.state === "listening";
  const isLoading = sttState.state === "loading";
  const canStart = selectedModel !== null && !isListening && !isLoading;

  return (
    <div className="settings-panel holo-refract-border holo-noise">
      {/* Decorative elements */}
      <div className="settings-highlight" />
      <div className="settings-bottom-line" />
      <div className="settings-scanline" />
      <span className="settings-corner settings-corner--tl" />
      <span className="settings-corner settings-corner--tr" />
      <span className="settings-corner settings-corner--bl" />
      <span className="settings-corner settings-corner--br" />

      <div className="settings-header">
        <h1 className="settings-title">Speech to Text</h1>
      </div>

      <div className="settings-content">
        <div className="settings-section">
          {/* Status Indicator */}
          <div className="stt-status" aria-live="polite">
            <div className="stt-status__header">
              <span
                className={`stt-status__indicator${
                  isListening
                    ? " stt-status__indicator--listening"
                    : sttState.state === "error"
                      ? " stt-status__indicator--error"
                      : ""
                }`}
              />
              <span
                className={`stt-status__label${
                  sttState.state === "error" ? " stt-status__label--error" : ""
                }`}
              >
                {statusLabel(sttState)}
              </span>
            </div>
            <div className="stt-dots">
              {DOTS.map((i) => (
                <span
                  key={i}
                  className={`stt-dot ${dotClass(sttState)}`}
                  style={{ animationDelay: `${i * 0.08}s` }}
                />
              ))}
            </div>
          </div>

          {/* Language Selector */}
          <LanguageSelector
            language={language}
            setLanguage={setLanguage}
            languages={languages}
          />

          {/* Start/Stop Button */}
          <button
            className={`stt-toggle ${
              isListening ? "stt-toggle--stop" : "stt-toggle--start"
            }`}
            onClick={isListening ? stopSession : startSession}
            disabled={isLoading || (!isListening && !canStart)}
          >
            {isListening
              ? "■ Stop"
              : isLoading
                ? "Initializing..."
                : "▶ Start Listening"}
          </button>

          {selectedModel === null && !isListening && !isLoading && (
            <span className="stt-hint">
              Download and select a model to start
            </span>
          )}

          {/* Error Message */}
          {errorMessage && <div className="stt-error">{errorMessage}</div>}

          {/* Models Section */}
          <div className="stt-section-divider">
            <span className="stt-section-divider__label">Models</span>
            <span className="stt-section-divider__line" />
          </div>

          <div className="stt-models-grid">
            {models.map((model) => (
              <ModelCard
                key={model.size}
                model={model}
                selected={selectedModel === model.size}
                onSelect={() => selectModel(model.size)}
                onDownload={() => downloadModel(model.size)}
                onCancel={() => cancelDownload(model.size)}
              />
            ))}
          </div>
        </div>
      </div>

      <div className="settings-footer">
        <button className="settings-close" onClick={handleClose}>
          Close
        </button>
      </div>
    </div>
  );
}

function ModelCard({
  model,
  selected,
  onSelect,
  onDownload,
  onCancel,
}: {
  model: ModelCardState;
  selected: boolean;
  onSelect: () => void;
  onDownload: () => void;
  onCancel: () => void;
}) {
  const isReady = model.status === "downloaded";
  const isDownloading = model.status === "downloading";

  return (
    <button
      type="button"
      className={`stt-model-card${isReady ? " stt-model-card--ready" : ""}${
        selected ? " stt-model-card--selected" : ""
      }`}
      onClick={isReady ? onSelect : undefined}
      aria-pressed={selected}
      aria-label={`${model.label} model, ${model.fileSize}${
        isReady ? ", ready" : isDownloading ? ", downloading" : ", not downloaded"
      }${selected ? ", selected" : ""}`}
    >
      <span className="stt-model-card__name">{model.label}</span>
      <span className="stt-model-card__size">{model.fileSize}</span>

      {model.status === "not_downloaded" && (
        <span
          className="stt-model-card__download"
          role="button"
          tabIndex={0}
          onClick={(e) => {
            e.stopPropagation();
            onDownload();
          }}
          onKeyDown={(e) => {
            if (e.key === "Enter" || e.key === " ") {
              e.stopPropagation();
              e.preventDefault();
              onDownload();
            }
          }}
        >
          Download
        </span>
      )}

      {isDownloading && (
        <>
          <div className="stt-progress">
            <div
              className="stt-progress__fill"
              style={{ width: `${model.downloadProgress ?? 0}%` }}
            />
          </div>
          <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center" }}>
            <span className="stt-model-card__size">
              {Math.round(model.downloadProgress ?? 0)}%
            </span>
            <span
              className="stt-model-card__cancel"
              role="button"
              tabIndex={0}
              onClick={(e) => {
                e.stopPropagation();
                onCancel();
              }}
              onKeyDown={(e) => {
                if (e.key === "Enter" || e.key === " ") {
                  e.stopPropagation();
                  e.preventDefault();
                  onCancel();
                }
              }}
            >
              ✕
            </span>
          </div>
        </>
      )}

      {isReady && (
        <span className="stt-model-card__status stt-model-card__status--ready">
          ✓ Ready
        </span>
      )}
    </button>
  );
}
