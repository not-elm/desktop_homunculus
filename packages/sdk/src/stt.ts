import {host, HomunculusApiError} from "./host";

/**
 * Speech-to-Text (STT) API namespace for speech recognition and model management.
 *
 * Provides a single-shot recognition API that captures audio from the microphone,
 * runs VAD and Whisper inference, and returns the recognized text. Also includes
 * model download/management and language listing utilities.
 *
 * @example
 * ```typescript
 * // Recognize a single sentence
 * const result = await stt.recognize({ language: "ja" });
 * console.log(result.text);
 *
 * // List available models
 * const models = await stt.models.list();
 * ```
 */
export namespace stt {
    /** Whisper model sizes available for STT. */
    export type SttModelSize = "tiny" | "base" | "small" | "medium" | "large-v3-turbo" | "large-v3";

    /** A transcription result from the STT engine. */
    export interface SttResult {
        /** The transcribed text. */
        text: string;
        /** Timestamp in seconds from session start. */
        timestamp: number;
        /** Detected or specified language code. */
        language: string;
    }

    /** STT error codes returned by the engine. */
    export type SttErrorCode =
        | "model_not_available"
        | "model_load_failed"
        | "pipeline_failed"
        | "no_microphone"
        | "microphone_permission_denied"
        | "download_failed"
        | "invalid_model_size"
        | "invalid_language"
        | "session_not_found"
        | "session_expired";

    /**
     * Type guard for STT-specific API errors.
     *
     * @param e - The caught error
     * @param code - Optional specific error code to check
     * @returns `true` if the error is an STT API error (optionally matching the given code)
     *
     * @example
     * ```typescript
     * try {
     *   await stt.recognize({ language: "ja" });
     * } catch (e) {
     *   if (stt.isSttError(e, "no_microphone")) {
     *     console.error("No microphone found");
     *   } else if (stt.isSttError(e, "model_not_available")) {
     *     console.error("Model not downloaded");
     *   }
     * }
     * ```
     */
    export function isSttError(
        e: unknown,
        code?: SttErrorCode,
    ): e is HomunculusApiError {
        if (!(e instanceof HomunculusApiError)) return false;
        if (code === undefined) return e.code !== undefined;
        return e.code === code;
    }

    /**
     * Recognize a single sentence from the microphone.
     *
     * Starts a capture → VAD → Whisper pipeline on the engine, waits for the
     * first recognized sentence, then destroys the pipeline. Long-polls until
     * speech is detected or timeout (60s).
     *
     * @param options - Optional language and model size
     * @param signal - Optional AbortSignal to cancel recognition
     * @returns The recognized text, timestamp, and detected language
     * @throws {HomunculusApiError} On timeout (408), invalid options (422), or service error (503)
     *
     * @example
     * ```typescript
     * const result = await stt.recognize({ language: "ja" });
     * console.log(result.text); // "こんにちは"
     * ```
     *
     * @example
     * ```typescript
     * // With cancellation
     * const controller = new AbortController();
     * const promise = stt.recognize({ language: "ja" }, controller.signal);
     * // Later: controller.abort();
     * ```
     */
    export async function recognize(
        options?: { language?: string; modelSize?: SttModelSize },
        signal?: AbortSignal,
    ): Promise<SttResult> {
        const response = await host.post(
            host.createUrl("stt/recognize"),
            options ?? {},
            signal,
        );
        return await response.json() as SttResult;
    }

    /** Information about a downloaded STT model. */
    export interface ModelInfo {
        /** The model size. */
        modelSize: SttModelSize;
        /** File size in bytes. */
        sizeBytes: number;
        /** Relative file path. */
        path: string;
    }

    /** Response from the non-streaming model download endpoint. */
    export interface ModelDownloadResponse {
        /** The model size. */
        modelSize: SttModelSize;
        /** Download status. */
        status: "downloaded" | "alreadyExists" | "downloading";
        /** File path (present when downloaded or already exists). */
        path?: string;
    }

    /** A progress or completion event from the streaming download endpoint. */
    export type DownloadEvent =
        | { type: "progress"; downloadedBytes: number; totalBytes: number; percentage: number }
        | { type: "complete"; modelSize: SttModelSize; path: string }
        | { type: "error"; message: string };

    /**
     * Model management sub-namespace for downloading and listing STT models.
     */
    export namespace models {
        /**
         * Lists all downloaded STT models.
         *
         * @returns Array of downloaded model information
         *
         * @example
         * ```typescript
         * const models = await stt.models.list();
         * for (const m of models) {
         *   console.log(`${m.modelSize}: ${m.sizeBytes} bytes`);
         * }
         * ```
         */
        export async function list(): Promise<ModelInfo[]> {
            const response = await host.get(host.createUrl("stt/models"));
            return await response.json() as ModelInfo[];
        }

        /**
         * Downloads an STT model with optional progress streaming.
         *
         * When called with `await`, returns the final download response.
         * When iterated with `for await...of`, yields progress events.
         *
         * @param options - Download options including model size
         * @returns An async iterable of download events (also awaitable for final result)
         *
         * @example
         * ```typescript
         * // Simple download (no progress)
         * const result = await stt.models.download({ modelSize: "small" });
         * console.log(result.status); // "downloaded" | "alreadyExists"
         *
         * // Download with progress
         * for await (const event of stt.models.download({ modelSize: "small" })) {
         *   if (event.type === "progress") {
         *     console.log(`${event.percentage.toFixed(1)}%`);
         *   } else if (event.type === "complete") {
         *     console.log(`Done: ${event.path}`);
         *   }
         * }
         * ```
         */
        export function download(options: {
            modelSize: SttModelSize;
            signal?: AbortSignal;
        }): DownloadStream {
            return new DownloadStream(options);
        }

        /**
         * Cancels an in-progress model download.
         *
         * @param modelSize - The model size to cancel
         * @returns `true` if a download was cancelled, `false` if no active download
         *
         * @example
         * ```typescript
         * const cancelled = await stt.models.cancelDownload("small");
         * ```
         */
        export async function cancelDownload(modelSize: SttModelSize): Promise<boolean> {
            try {
                await host.deleteMethod(
                    host.createUrl("stt/models/download", { modelSize }),
                );
                return true;
            } catch (e) {
                if (e instanceof HomunculusApiError && e.statusCode === 404) {
                    return false;
                }
                throw e;
            }
        }
    }

    /**
     * A download stream that is both an async iterable (for progress)
     * and a thenable (for simple await).
     */
    export class DownloadStream implements AsyncIterable<DownloadEvent>, PromiseLike<ModelDownloadResponse> {
        private readonly options: { modelSize: SttModelSize; signal?: AbortSignal };

        constructor(options: { modelSize: SttModelSize; signal?: AbortSignal }) {
            this.options = options;
        }

        async *[Symbol.asyncIterator](): AsyncIterator<DownloadEvent> {
            const stream = host.postStream<DownloadEvent>(
                host.createUrl("stt/models/download/stream"),
                { modelSize: this.options.modelSize },
                this.options.signal,
            );
            yield* stream;
        }

        then<TResult1 = ModelDownloadResponse, TResult2 = never>(
            onfulfilled?: ((value: ModelDownloadResponse) => TResult1 | PromiseLike<TResult1>) | null,
            onrejected?: ((reason: unknown) => TResult2 | PromiseLike<TResult2>) | null,
        ): Promise<TResult1 | TResult2> {
            const promise = (async (): Promise<ModelDownloadResponse> => {
                const response = await host.post(
                    host.createUrl("stt/models/download"),
                    { modelSize: this.options.modelSize },
                );
                return await response.json() as ModelDownloadResponse;
            })();
            return promise.then(onfulfilled, onrejected);
        }
    }

    /** A supported language entry. */
    export interface LanguageEntry {
        /** ISO 639-1 (or similar) language code. */
        code: string;
        /** Human-readable display name. */
        label: string;
    }

    /**
     * Fetches the list of supported STT languages from the engine.
     *
     * Returns language codes with human-readable labels generated via
     * `Intl.DisplayNames`. Codes not recognized by the browser fall back
     * to the raw code string.
     *
     * @returns Array of language entries sorted with "auto" first
     *
     * @example
     * ```typescript
     * const languages = await stt.languages();
     * // [{ code: "auto", label: "Auto Detect" }, { code: "en", label: "English" }, ...]
     * ```
     */
    export async function languages(): Promise<LanguageEntry[]> {
        const response = await host.get(host.createUrl("stt/languages"));
        const codes = await response.json() as string[];

        let displayNames: Intl.DisplayNames | null = null;
        try {
            displayNames = new Intl.DisplayNames(["en"], { type: "language" });
        } catch {
            // Fallback: use raw codes
        }

        return codes.map((code) => {
            if (code === "auto") return { code, label: "Auto Detect" };
            let label = code;
            if (displayNames) {
                try {
                    label = displayNames.of(code) ?? code;
                } catch {
                    label = code;
                }
            }
            return { code, label };
        });
    }

    /**
     * Push-to-Talk (PTT) API for manual recording control.
     *
     * @example
     * ```typescript
     * // Start recording
     * const { sessionId } = await stt.ptt.start({ language: "ja" });
     *
     * // ... user holds button ...
     *
     * // Stop and get result
     * const result = await stt.ptt.stop(sessionId);
     * console.log(result.text);
     * ```
     */
    export namespace ptt {
        /** Options for starting a PTT session. */
        export interface StartOptions {
            /** Recognition language. Defaults to "auto". */
            language?: string;
            /** Whisper model size. Defaults to "base". */
            modelSize?: SttModelSize;
            /** Session timeout in seconds (max 300). Defaults to 60. */
            timeoutSecs?: number;
        }

        /** Response from starting a PTT session. */
        export interface StartResponse {
            /** Unique session identifier. Pass to `stop()` to end recording. */
            sessionId: string;
        }

        /**
         * Start a PTT recording session.
         *
         * Begins capturing audio from the default microphone. If an active
         * session exists, it is automatically cancelled.
         *
         * @param options - Recognition options
         * @returns Session ID for use with `stop()`
         *
         * @example
         * ```typescript
         * const { sessionId } = await stt.ptt.start();
         * ```
         */
        export async function start(
            options?: StartOptions,
        ): Promise<StartResponse> {
            return host.post<StartOptions>(
                host.createUrl("/stt/ptt/start"),
                options ?? {},
            );
        }

        /**
         * Stop a PTT session and get the recognition result.
         *
         * Stops recording, runs Whisper inference, and returns the transcription.
         *
         * @param sessionId - Session ID from `start()`
         * @returns Recognition result with transcribed text
         *
         * @example
         * ```typescript
         * const result = await stt.ptt.stop(sessionId);
         * console.log(result.text, result.language);
         * ```
         */
        export async function stop(sessionId: string): Promise<SttResult> {
            return host.post(
                host.createUrl(`/stt/ptt/${sessionId}/stop`),
                {},
            );
        }
    }
}
