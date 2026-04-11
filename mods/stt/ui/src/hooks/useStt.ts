import type { stt as sttTypes } from '@hmcs/sdk';
import { stt } from '@hmcs/sdk';
import { useCallback, useEffect, useRef, useState } from 'react';

type SttModelSize = sttTypes.SttModelSize;

export interface ModelCardState {
  size: SttModelSize;
  label: string;
  fileSize: string;
  status: 'not_downloaded' | 'downloading' | 'downloaded';
  downloadProgress?: number;
}

const MODEL_DEFINITIONS: Omit<ModelCardState, 'status' | 'downloadProgress'>[] = [
  { size: 'tiny', label: 'Tiny', fileSize: '32.2 MB' },
  { size: 'base', label: 'Base', fileSize: '59.7 MB' },
  { size: 'small', label: 'Small', fileSize: '190 MB' },
  { size: 'medium', label: 'Medium', fileSize: '539 MB' },
  { size: 'large-v3-turbo', label: 'Large v3 Turbo', fileSize: '574 MB' },
  { size: 'large-v3', label: 'Large v3', fileSize: '1.08 GB' },
];

/** Returns an updater that patches the model matching `size` with the given fields. */
function updateModelBySize(
  size: SttModelSize,
  patch: Partial<Pick<ModelCardState, 'status' | 'downloadProgress'>>,
): (prev: ModelCardState[]) => ModelCardState[] {
  return (prev) => prev.map((m) => (m.size === size ? { ...m, ...patch } : m));
}

/** Builds initial model card states from definitions and a set of downloaded sizes. */
function buildModelCards(downloadedSizes: Set<SttModelSize>): ModelCardState[] {
  return MODEL_DEFINITIONS.map((d) => ({
    ...d,
    status: downloadedSizes.has(d.size) ? ('downloaded' as const) : ('not_downloaded' as const),
  }));
}

export function useStt() {
  const [models, setModels] = useState<ModelCardState[]>(
    MODEL_DEFINITIONS.map((d) => ({ ...d, status: 'not_downloaded' as const })),
  );
  const [errorMessage, setErrorMessage] = useState<string | null>(null);

  const abortControllers = useRef<Map<SttModelSize, AbortController>>(new Map());

  useEffect(() => {
    let cancelled = false;

    async function loadInitialState() {
      const downloadedModels = await stt.models.list();
      if (cancelled) return;

      const downloadedSizes = new Set(downloadedModels.map((m) => m.modelSize));
      setModels(buildModelCards(downloadedSizes));
    }

    loadInitialState();

    return () => {
      cancelled = true;
    };
  }, []);

  const downloadModel = useCallback(async (size: SttModelSize) => {
    const controller = new AbortController();
    abortControllers.current.set(size, controller);

    setModels(updateModelBySize(size, { status: 'downloading', downloadProgress: 0 }));

    function handleDownloadEvent(event: sttTypes.DownloadEvent) {
      if (event.type === 'progress') {
        setModels(updateModelBySize(size, { downloadProgress: event.percentage }));
      } else if (event.type === 'complete') {
        setModels(updateModelBySize(size, { status: 'downloaded', downloadProgress: undefined }));
      } else if (event.type === 'error') {
        setModels(
          updateModelBySize(size, { status: 'not_downloaded', downloadProgress: undefined }),
        );
        setErrorMessage(event.message);
        setTimeout(() => setErrorMessage(null), 3000);
      }
    }

    try {
      for await (const event of stt.models.download({
        modelSize: size,
        signal: controller.signal,
      })) {
        handleDownloadEvent(event);
      }
    } catch {
      setModels((prev) =>
        prev.map((m) =>
          m.size === size && m.status === 'downloading'
            ? { ...m, status: 'not_downloaded' as const, downloadProgress: undefined }
            : m,
        ),
      );
    } finally {
      abortControllers.current.delete(size);
    }
  }, []);

  const cancelDownload = useCallback(async (size: SttModelSize) => {
    const controller = abortControllers.current.get(size);
    controller?.abort();
    await stt.models.cancelDownload(size);
    setModels(updateModelBySize(size, { status: 'not_downloaded', downloadProgress: undefined }));
  }, []);

  return {
    models,
    downloadModel,
    cancelDownload,
    errorMessage,
  };
}
