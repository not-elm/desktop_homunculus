import { audio, Webview } from '@hmcs/sdk';
import { cn, Toolbar } from '@hmcs/ui';
import { useCallback } from 'react';
import { type ModelCardState, useStt } from '../hooks/useStt';
import { Decorations } from './Decorations';

export function SttPanel() {
  const { models, downloadModel, cancelDownload, errorMessage } = useStt();

  const handleClose = useCallback(() => {
    audio.se.play('se:close');
    Webview.current()?.close();
  }, []);

  return (
    <div className="holo-noise relative box-border flex h-screen max-h-screen max-w-screen flex-col overflow-hidden rounded-xl bg-panel/92 animate-settings-in motion-reduce:animate-none motion-reduce:opacity-100">
      <Decorations />

      <Toolbar title="Speech to Text" onClose={handleClose} />

      <div className="no-scrollbar relative z-[7] flex min-h-0 flex-1 flex-col gap-4 overflow-y-auto p-5">
        <div className="flex flex-col gap-4">
          {errorMessage && (
            <div className="py-1.5 text-[0.7rem] tracking-[0.04em] text-destructive">
              {errorMessage}
            </div>
          )}

          <div className="mt-1 flex items-center gap-2">
            <span className="whitespace-nowrap text-[0.7rem] uppercase tracking-[0.12em] text-hud-text-subdued">
              Models
            </span>
            <span className="h-px flex-1 bg-gradient-to-r from-primary/15 to-transparent" />
          </div>

          <div className="grid grid-cols-2 gap-2">
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
      className={cn(
        'flex flex-col gap-1.5 rounded-lg border bg-input p-3 text-left',
        isReady ? 'border-primary/30' : 'border-muted-foreground/25',
      )}
      title={`${model.label} model, ${model.fileSize}${
        isReady ? ', ready' : isDownloading ? ', downloading' : ', not downloaded'
      }`}
    >
      <span className="text-[0.8rem] font-semibold uppercase tracking-[0.08em] text-foreground">
        {model.label}
      </span>
      <span className="font-mono text-[0.7rem] text-muted-foreground">{model.fileSize}</span>

      {model.status === 'not_downloaded' && (
        <button
          type="button"
          className="inline-block cursor-pointer rounded border border-primary/25 bg-primary/10 px-2 py-1 text-[0.7rem] uppercase tracking-[0.06em] text-primary transition-[background,border-color] duration-200 ease-out hover:border-primary/40 hover:bg-primary/20"
          onClick={onDownload}
        >
          Download
        </button>
      )}

      {isDownloading && (
        <>
          <div className="h-1 overflow-hidden rounded-sm bg-hud-surface-toggle-off/40">
            <div
              className="h-full rounded-sm bg-primary transition-[width] duration-200 ease-out"
              style={{ width: `${model.downloadProgress ?? 0}%` }}
            />
          </div>
          <div className="flex items-center justify-between">
            <span className="font-mono text-[0.7rem] text-muted-foreground">
              {Math.round(model.downloadProgress ?? 0)}%
            </span>
            <button
              type="button"
              className="cursor-pointer border-none bg-transparent px-1 py-0.5 text-xs text-muted-foreground hover:text-destructive"
              onClick={onCancel}
            >
              ✕
            </button>
          </div>
        </>
      )}

      {isReady && <span className="text-[0.7rem] tracking-[0.04em] text-success">✓ Ready</span>}
    </div>
  );
}
