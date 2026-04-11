import { audio, Webview } from '@hmcs/sdk';
import { useCallback } from 'react';
import { type ModelCardState, useStt } from '../hooks/useStt';

export function SttPanel() {
  const { models, downloadModel, cancelDownload, errorMessage } = useStt();

  const handleClose = useCallback(() => {
    audio.se.play('se:close');
    Webview.current()?.close();
  }, []);

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
                onDownload={() => downloadModel(model.size)}
                onCancel={() => cancelDownload(model.size)}
              />
            ))}
          </div>
        </div>
      </div>

      <div className="settings-footer">
        <button type="button" className="settings-close" onClick={handleClose}>
          Close
        </button>
      </div>
    </div>
  );
}

function ModelCard({
  model,
  onDownload,
  onCancel,
}: {
  model: ModelCardState;
  onDownload: () => void;
  onCancel: () => void;
}) {
  const isReady = model.status === 'downloaded';
  const isDownloading = model.status === 'downloading';

  return (
    <div
      className={`stt-model-card${isReady ? ' stt-model-card--ready' : ''}`}
      title={`${model.label} model, ${model.fileSize}${
        isReady ? ', ready' : isDownloading ? ', downloading' : ', not downloaded'
      }`}
    >
      <span className="stt-model-card__name">{model.label}</span>
      <span className="stt-model-card__size">{model.fileSize}</span>

      {model.status === 'not_downloaded' && (
        <button type="button" className="stt-model-card__download" onClick={onDownload}>
          Download
        </button>
      )}

      {isDownloading && (
        <>
          <div className="stt-progress">
            <div
              className="stt-progress__fill"
              style={{ width: `${model.downloadProgress ?? 0}%` }}
            />
          </div>
          <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
            <span className="stt-model-card__size">{Math.round(model.downloadProgress ?? 0)}%</span>
            <button type="button" className="stt-model-card__cancel" onClick={onCancel}>
              ✕
            </button>
          </div>
        </>
      )}

      {isReady && (
        <span className="stt-model-card__status stt-model-card__status--ready">✓ Ready</span>
      )}
    </div>
  );
}
