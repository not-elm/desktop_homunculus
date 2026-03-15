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
  { size: "tiny", label: "Tiny", fileSize: "32.5 MB" },
  { size: "base", label: "Base", fileSize: "59.8 MB" },
  { size: "small", label: "Small", fileSize: "189.8 MB" },
];

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

    (async () => {
      const [downloadedModels, langList] = await Promise.all([
        stt.models.list(),
        stt.languages(),
      ]);
      if (cancelled) return;

      setLanguages(langList);

      const savedLang = await preferences.load<string>("stt::language");
      if (savedLang) setLanguage(savedLang);

      const downloadedSizes = new Set(downloadedModels.map((m) => m.modelSize));
      setModels(
        MODEL_DEFINITIONS.map((d) => ({
          ...d,
          status: downloadedSizes.has(d.size) ? ("downloaded" as const) : ("not_downloaded" as const),
        })),
      );

      const sizes: SttModelSize[] = ["small", "base", "tiny"];
      const defaultSelected = sizes.find((s) => downloadedSizes.has(s)) ?? null;
      setSelectedModel(defaultSelected);
    })();

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
  }, []);

  const downloadModel = useCallback(async (size: SttModelSize) => {
    const controller = new AbortController();
    abortControllers.current.set(size, controller);

    setModels((prev) =>
      prev.map((m) =>
        m.size === size ? { ...m, status: "downloading" as const, downloadProgress: 0 } : m,
      ),
    );

    try {
      for await (const event of stt.models.download({
        modelSize: size,
        signal: controller.signal,
      })) {
        if (event.type === "progress") {
          setModels((prev) =>
            prev.map((m) =>
              m.size === size ? { ...m, downloadProgress: event.percentage } : m,
            ),
          );
        } else if (event.type === "complete") {
          setModels((prev) =>
            prev.map((m) =>
              m.size === size
                ? { ...m, status: "downloaded" as const, downloadProgress: undefined }
                : m,
            ),
          );
          setSelectedModel((prev) => prev ?? size);
        } else if (event.type === "error") {
          setModels((prev) =>
            prev.map((m) =>
              m.size === size
                ? { ...m, status: "not_downloaded" as const, downloadProgress: undefined }
                : m,
            ),
          );
          setErrorMessage(event.message);
          setTimeout(() => setErrorMessage(null), 3000);
        }
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
    setModels((prev) =>
      prev.map((m) =>
        m.size === size
          ? { ...m, status: "not_downloaded" as const, downloadProgress: undefined }
          : m,
      ),
    );
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
