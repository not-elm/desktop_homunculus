import {host, HomunculusApiError} from "./host";
import {EventSource} from "eventsource";

/**
 * Speech-to-Text (STT) API namespace for controlling real-time speech recognition.
 *
 * Provides session lifecycle management, real-time transcription event streaming,
 * and model management for Whisper-based speech recognition.
 *
 * Only one STT session can be active at a time. Starting a new session while
 * one is already listening will implicitly restart with the new options.
 *
 * @example
 * ```typescript
 * // Start transcription and listen for results
 * await stt.session.start({ language: "ja", modelSize: "small" });
 *
 * const stream = stt.stream({
 *   onResult: (result) => console.log(result.text),
 * });
 *
 * // Later, stop
 * await stt.session.stop();
 * stream.close();
 * ```
 */
export namespace stt {
    /** Whisper model sizes available for STT. */
    export type SttModelSize = "tiny" | "base" | "small" | "medium";

    /**
     * Session state as a tagged union. Matches the engine's `SttState` enum.
     *
     * Use the `state` field to discriminate:
     * - `"idle"` — No active session
     * - `"loading"` — Model is being loaded
     * - `"listening"` — Actively transcribing
     * - `"error"` — Session encountered an error
     */
    export type SttState =
        | { state: "idle" }
        | { state: "loading"; language: string; modelSize: SttModelSize }
        | { state: "listening"; language: string; modelSize: SttModelSize }
        | { state: "error"; error: string; message: string };

    /** A transcription result from the STT engine. */
    export interface SttResult {
        /** The transcribed text. */
        text: string;
        /** Timestamp in seconds from session start. */
        timestamp: number;
        /** Detected or specified language code. */
        language: string;
    }

    /** An error event from the STT session. */
    export interface SttSessionError {
        /** The error code. */
        error: string;
        /** A human-readable error message. */
        message: string;
    }

    /** Options for starting an STT session. */
    export interface SttStartOptions {
        /**
         * Language code for transcription (ISO 639-1).
         * Use `"auto"` for automatic language detection.
         * @defaultValue `"auto"`
         */
        language?: string;
        /**
         * Whisper model size to use.
         * @defaultValue `"small"`
         */
        modelSize?: SttModelSize;
    }

    /** STT error codes returned by the engine. */
    export type SttErrorCode =
        | "session_already_active"
        | "session_loading"
        | "model_not_available"
        | "model_load_failed"
        | "pipeline_failed"
        | "no_microphone"
        | "microphone_permission_denied"
        | "download_failed"
        | "invalid_model_size"
        | "invalid_language";

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
     *   await stt.session.start({ language: "ja" });
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
     * Session management sub-namespace.
     *
     * Only one STT session can be active at a time. Calling `start()` while
     * a session is in `listening` state will implicitly stop the current
     * session and start a new one. Calling `start()` while in `loading`
     * state will return a `session_loading` error.
     */
    export namespace session {
        /**
         * Starts an STT session with the given options.
         *
         * If a session is already in `listening` state, it will be implicitly
         * stopped and a new session started (implicit restart). If a session is
         * in `loading` state, a `session_loading` error is returned.
         *
         * @param options - Session configuration (language, model size)
         * @returns The new session state
         *
         * @example
         * ```typescript
         * // Start with defaults (auto language, small model)
         * const state = await stt.session.start();
         *
         * // Start with specific options
         * const state = await stt.session.start({
         *   language: "ja",
         *   modelSize: "medium",
         * });
         * ```
         */
        export async function start(options?: SttStartOptions): Promise<SttState> {
            const response = await host.post(host.createUrl("stt/start"), options ?? {});
            const body = await response.json() as SttState & { restarted?: boolean };
            if (body.restarted) {
                console.warn("STT session was implicitly restarted. Previous session was stopped.");
            }
            return body;
        }

        /**
         * Stops the current STT session. Idempotent — safe to call even if
         * no session is active.
         *
         * @returns The session state after stopping (always `idle`)
         *
         * @example
         * ```typescript
         * await stt.session.stop();
         * ```
         */
        export async function stop(): Promise<SttState> {
            const response = await host.post(host.createUrl("stt/stop"));
            return await response.json() as SttState;
        }

        /**
         * Gets the current STT session status.
         *
         * @returns The current session state
         *
         * @example
         * ```typescript
         * const status = await stt.session.status();
         * if (status.state === "listening") {
         *   console.log("STT is active");
         * }
         * ```
         */
        export async function status(): Promise<SttState> {
            const response = await host.get(host.createUrl("stt/status"));
            return await response.json() as SttState;
        }
    }

    /** Callbacks for SSE stream events. All callbacks are optional. */
    export interface StreamCallbacks {
        /** Called when a transcription result is received. */
        onResult?: (result: SttResult) => void | Promise<void>;
        /** Called when the session state changes. */
        onStatus?: (state: SttState) => void | Promise<void>;
        /** Called when a session error occurs. */
        onSessionError?: (error: SttSessionError) => void | Promise<void>;
        /** Called when the session is stopped. */
        onStopped?: () => void | Promise<void>;
    }

    /**
     * Wrapper around an SSE connection to the STT event stream.
     *
     * The server sends an initial `status` event on connect (late-join sync),
     * so there is no need to separately query the current state.
     */
    export class SttStream {
        private readonly es: EventSource;

        constructor(callbacks: StreamCallbacks) {
            const url = host.createUrl("stt/stream");
            this.es = new EventSource(url.toString());

            if (callbacks.onStatus) {
                const cb = callbacks.onStatus;
                this.es.addEventListener("status", async (event: MessageEvent) => {
                    try {
                        const state: SttState = JSON.parse(event.data);
                        await cb(state);
                    } catch (error) {
                        console.error("Error processing STT status event:", error);
                    }
                });
            }

            if (callbacks.onResult) {
                const cb = callbacks.onResult;
                this.es.addEventListener("result", async (event: MessageEvent) => {
                    try {
                        const result: SttResult = JSON.parse(event.data);
                        await cb(result);
                    } catch (error) {
                        console.error("Error processing STT result event:", error);
                    }
                });
            }

            if (callbacks.onSessionError) {
                const cb = callbacks.onSessionError;
                this.es.addEventListener("session_error", async (event: MessageEvent) => {
                    try {
                        const err: SttSessionError = JSON.parse(event.data);
                        await cb(err);
                    } catch (error) {
                        console.error("Error processing STT session_error event:", error);
                    }
                });
            }

            if (callbacks.onStopped) {
                const cb = callbacks.onStopped;
                this.es.addEventListener("stopped", async () => {
                    try {
                        await cb();
                    } catch (error) {
                        console.error("Error processing STT stopped event:", error);
                    }
                });
            }
        }

        /**
         * Closes the SSE connection.
         *
         * @example
         * ```typescript
         * const stream = stt.stream({ onResult: (r) => console.log(r.text) });
         * // ... later
         * stream.close();
         * ```
         */
        close(): void {
            this.es.close();
        }
    }

    /**
     * Creates a persistent SSE connection to receive real-time STT events.
     *
     * The server sends the current session state immediately on connect
     * (late-join sync), so the `onStatus` callback will fire right away.
     *
     * @param callbacks - Event handlers for different STT event types
     * @returns An `SttStream` instance for managing the connection
     *
     * @example
     * ```typescript
     * // Listen for transcription results only
     * const stream = stt.stream({
     *   onResult: (result) => {
     *     console.log(`[${result.language}] ${result.text}`);
     *   },
     * });
     *
     * // Listen for all events
     * const stream = stt.stream({
     *   onResult: (result) => console.log(result.text),
     *   onStatus: (state) => console.log("State:", state.state),
     *   onSessionError: (err) => console.error(err.message),
     *   onStopped: () => console.log("Session ended"),
     * });
     *
     * // Close the stream when done
     * stream.close();
     * ```
     */
    export function stream(callbacks: StreamCallbacks): SttStream {
        return new SttStream(callbacks);
    }
}
