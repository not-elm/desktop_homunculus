import { useCallback, useEffect, useRef, useState } from "react";
import { preferences, stt } from "@hmcs/sdk";
import type { stt as sttTypes } from "@hmcs/sdk";

type SttModelSize = sttTypes.SttModelSize;
type SttState = sttTypes.SttState;

export interface ModelCardState {
  size: SttModelSize;
  label: string;
  fileSize: string;
  status: "not_downloaded" | "downloading" | "downloaded";
  downloadProgress?: number;
}

const MODEL_DEFINITIONS: Omit<ModelCardState, "status" | "downloadProgress">[] = [
  { size: "tiny", label: "Tiny", fileSize: "32.2 MB" },
  { size: "base", label: "Base", fileSize: "59.7 MB" },
  { size: "small", label: "Small", fileSize: "190 MB" },
  { size: "medium", label: "Medium", fileSize: "539 MB" },
  { size: "large-v3-turbo", label: "Large v3 Turbo", fileSize: "574 MB" },
  { size: "large-v3", label: "Large v3", fileSize: "1.08 GB" },
];

/** Priority order for selecting a default model when none is saved. */
const MODEL_PRIORITY: SttModelSize[] = [
  "small", "medium", "large-v3-turbo", "large-v3", "base", "tiny",
];

/** Returns an updater that patches the model matching `size` with the given fields. */
function updateModelBySize(
  size: SttModelSize,
  patch: Partial<Pick<ModelCardState, "status" | "downloadProgress">>,
): (prev: ModelCardState[]) => ModelCardState[] {
  return (prev) => prev.map((m) => (m.size === size ? { ...m, ...patch } : m));
}

/** Builds initial model card states from definitions and a set of downloaded sizes. */
function buildModelCards(downloadedSizes: Set<SttModelSize>): ModelCardState[] {
  return MODEL_DEFINITIONS.map((d) => ({
    ...d,
    status: downloadedSizes.has(d.size) ? ("downloaded" as const) : ("not_downloaded" as const),
  }));
}

/** Picks the best default model: saved preference if still downloaded, otherwise highest-priority downloaded. */
function pickDefaultModel(
  savedModel: SttModelSize | null,
  downloadedSizes: Set<SttModelSize>,
): SttModelSize | null {
  if (savedModel && downloadedSizes.has(savedModel)) return savedModel;
  return MODEL_PRIORITY.find((s) => downloadedSizes.has(s)) ?? null;
}

export function useStt() {
  const [sttState, setSttState] = useState<SttState>({ state: "idle" });
  const [models, setModels] = useState<ModelCardState[]>(
    MODEL_DEFINITIONS.map((d) => ({ ...d, status: "not_downloaded" as const })),
  );
  const [selectedModel, setSelectedModel] = useState<SttModelSize | null>(null);
  const [language, setLanguage] = useState("auto");
  const [languages, setLanguages] = useState<sttTypes.LanguageEntry[]>([]);
  const [errorMessage, setErrorMessage] = useState<string | null>(null);

  const abortControllers = useRef<Map<SttModelSize, AbortController>>(new Map());

  useEffect(() => {
    let cancelled = false;

    async function loadInitialState() {
      const [downloadedModels, langList] = await Promise.all([
        stt.models.list(),
        stt.languages(),
      ]);
      if (cancelled) return;

      setLanguages(langList);

      const [savedLang, savedModel] = await Promise.all([
        preferences.load<string>("stt::language"),
        preferences.load<SttModelSize>("stt::model"),
      ]);
      if (savedLang) setLanguage(savedLang);

      const downloadedSizes = new Set(downloadedModels.map((m) => m.modelSize));
      setModels(buildModelCards(downloadedSizes));
      setSelectedModel(pickDefaultModel(savedModel, downloadedSizes));
    }

    loadInitialState();

    const stream = stt.stream({
      onStatus: (state) => {
        if (cancelled) return;
        setSttState(state);
        if (state.state === "listening" || state.state === "loading") {
          setLanguage(state.language);
          setSelectedModel(state.modelSize);
        }
      },
    });

    return () => {
      cancelled = true;
      stream.close();
    };
  }, []);

  const changeLanguage = useCallback((lang: string) => {
    setLanguage(lang);
    preferences.save("stt::language", lang);
  }, []);

  const selectModel = useCallback((size: SttModelSize) => {
    setModels((prev) => {
      const model = prev.find((m) => m.size === size);
      if (model?.status !== "downloaded") return prev;
      return prev;
    });
    setSelectedModel(size);
    preferences.save("stt::model", size);
  }, []);

  const downloadModel = useCallback(async (size: SttModelSize) => {
    const controller = new AbortController();
    abortControllers.current.set(size, controller);

    setModels(updateModelBySize(size, { status: "downloading", downloadProgress: 0 }));

    function handleDownloadEvent(event: sttTypes.DownloadEvent) {
      if (event.type === "progress") {
        setModels(updateModelBySize(size, { downloadProgress: event.percentage }));
      } else if (event.type === "complete") {
        markModelDownloaded(size);
      } else if (event.type === "error") {
        markModelFailed(size, event.message);
      }
    }

    function markModelDownloaded(modelSize: SttModelSize) {
      setModels(updateModelBySize(modelSize, { status: "downloaded", downloadProgress: undefined }));
      setSelectedModel((prev) => {
        const next = prev ?? modelSize;
        preferences.save("stt::model", next);
        return next;
      });
    }

    function markModelFailed(modelSize: SttModelSize, message: string) {
      setModels(updateModelBySize(modelSize, { status: "not_downloaded", downloadProgress: undefined }));
      setErrorMessage(message);
      setTimeout(() => setErrorMessage(null), 3000);
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
          m.size === size && m.status === "downloading"
            ? { ...m, status: "not_downloaded" as const, downloadProgress: undefined }
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
    setModels(updateModelBySize(size, { status: "not_downloaded", downloadProgress: undefined }));
  }, []);

  const startSession = useCallback(async () => {
    if (selectedModel === null) return;
    try {
      await stt.session.start({ language, modelSize: selectedModel });
    } catch (e) {
      if (stt.isSttError(e, "no_microphone")) {
        setErrorMessage("No microphone found. Please connect a microphone.");
      } else if (stt.isSttError(e, "microphone_permission_denied")) {
        setErrorMessage(
          "Microphone access denied. Please grant permission in system settings.",
        );
      } else if (stt.isSttError(e)) {
        setErrorMessage((e as Error).message);
      } else {
        setErrorMessage("An unexpected error occurred.");
      }
      setTimeout(() => setErrorMessage(null), 3000);
    }
  }, [language, selectedModel]);

  const stopSession = useCallback(async () => {
    await stt.session.stop();
  }, []);

  return {
    sttState,
    models,
    selectedModel,
    selectModel,
    downloadModel,
    cancelDownload,
    language,
    setLanguage: changeLanguage,
    languages,
    startSession,
    stopSession,
    errorMessage,
  };
}
