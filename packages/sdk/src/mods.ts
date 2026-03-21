/**
 * Mod management namespace for executing commands defined in mod packages.
 *
 * Provides functions to run MOD commands from installed mods,
 * with support for passing arguments, stdin data, configuring timeouts,
 * and real-time NDJSON streaming of command output.
 *
 * @example
 * ```typescript
 * // Execute a command and collect the result
 * const result = await mods.executeCommand({ command: "greet" });
 * console.log(result.stdout);
 *
 * // Stream command output in real-time
 * for await (const event of mods.streamCommand({ command: "build" })) {
 *   if (event.type === "stdout") console.log(event.data);
 *   if (event.type === "stderr") console.error(event.data);
 *   if (event.type === "exit") console.log("Exit code:", event.exitCode);
 * }
 * ```
 */

import { host } from "./host";

export namespace mods {
    /**
     * Summary information about a loaded mod.
     *
     * @example
     * ```typescript
     * const allMods = await mods.list();
     * for (const mod of allMods) {
     *   console.log(`${mod.name}@${mod.version} (${mod.commands.length} commands)`);
     * }
     * ```
     */
    export interface ModInfo {
        /** The mod package name. */
        name: string;
        /** The mod package version. */
        version: string;
        /** Optional description from package.json. */
        description?: string;
        /** Optional author from package.json. */
        author?: string;
        /** Optional license from package.json. */
        license?: string;
        /** Service script path (auto-launched at startup), or null if no service. */
        serviceScriptPath?: string;
        /** Available MOD command names. */
        commands: string[];
        /** Asset declarations keyed by asset ID. */
        assets: Record<string, { path: string; type: string; description: string }>;
        /** Menu entries registered by this mod. */
        menus: Array<{ id: string; text: string; command: string }>;
        /** Absolute path to the mod's root directory. */
        modDir: string;
    }

    /**
     * List all loaded mods and their metadata.
     *
     * Returns summary information for every mod discovered at startup,
     * including available MOD commands and registered asset IDs.
     *
     * @returns Array of mod information objects
     *
     * @example
     * ```typescript
     * // List all installed mods
     * const allMods = await mods.list();
     * console.log(`${allMods.length} mods installed`);
     *
     * // Find mods with bin commands
     * const withCommands = allMods.filter(m => m.commands.length > 0);
     *
     * // Get assets from a specific mod
     * const elmer = allMods.find(m => m.name === "elmer");
     * if (elmer) {
     *   console.log("Elmer assets:", Object.keys(elmer.assets));
     * }
     * ```
     */
    export async function list(): Promise<ModInfo[]> {
        const response = await host.get(host.createUrl("mods"));
        return await response.json();
    }

    /**
     * Get detailed information about a specific mod by name.
     *
     * @param modName - The mod package name
     * @returns Mod information
     *
     * @example
     * ```typescript
     * const elmer = await mods.get("elmer");
     * console.log(`${elmer.name}@${elmer.version}`);
     * console.log("Commands:", elmer.commands);
     * ```
     */
    export async function get(modName: string): Promise<ModInfo> {
        const response = await host.get(host.createUrl("mods/by-name", { name: modName }));
        return await response.json();
    }

    /**
     * Request parameters for executing a mod command.
     *
     * @example
     * ```typescript
     * const request: mods.ExecuteCommandRequest = {
     *   command: "build",
     *   args: ["--verbose"],
     *   stdin: "input data",
     *   timeoutMs: 5000,
     * };
     * ```
     */
    export interface ExecuteCommandRequest {
        /** The command name to execute (resolved via npx from installed mod packages). */
        command: string;
        /** Arguments to pass to the script (after the script path). Max 64 args, each max 4096 chars. */
        args?: string[];
        /** Data to write to the process stdin. Stdin is closed after writing. Max 1 MiB. */
        stdin?: string;
        /** Timeout in milliseconds (1–300000). Defaults to 30000 (30s). */
        timeoutMs?: number;
    }

    /** A stdout line event from a streaming command execution. */
    export interface CommandStdoutEvent {
        type: "stdout";
        data: string;
    }

    /** A stderr line event from a streaming command execution. */
    export interface CommandStderrEvent {
        type: "stderr";
        data: string;
    }

    /** The exit event from a streaming command execution. Always the last event. */
    export interface CommandExitEvent {
        type: "exit";
        /** Process exit code, or null if the process was killed by a signal. */
        exitCode: number | null;
        /** Whether the process was killed due to timeout. */
        timedOut: boolean;
        /** Unix signal name if the process was killed by a signal. */
        signal?: string;
    }

    /** Union of all command event types emitted during streaming execution. */
    export type CommandEvent = CommandStdoutEvent | CommandStderrEvent | CommandExitEvent;

    /**
     * Buffered result of a command execution.
     *
     * @example
     * ```typescript
     * const result = await mods.executeCommand({ command: "hello" });
     * if (result.exitCode === 0) {
     *   console.log("Output:", result.stdout);
     * } else {
     *   console.error("Error:", result.stderr);
     * }
     * ```
     */
    export interface CommandResult {
        /** Process exit code, or null if the process was killed by a signal. */
        exitCode: number | null;
        /** Whether the process was killed due to timeout. */
        timedOut: boolean;
        /** Unix signal name if the process was killed by a signal. */
        signal?: string;
        /** All stdout output joined by newlines. */
        stdout: string;
        /** All stderr output joined by newlines. */
        stderr: string;
    }

    /** Wire format for NDJSON events from the server. */
    interface RawCommandEvent {
        type: "stdout" | "stderr" | "exit";
        data?: string;
        code?: number | null;
        timedOut?: boolean;
        signal?: string;
    }

    function toRequestBody(request: ExecuteCommandRequest): object {
        return {
            command: request.command,
            args: request.args,
            stdin: request.stdin,
            timeoutMs: request.timeoutMs,
        };
    }

    function toCommandEvent(raw: RawCommandEvent): CommandEvent {
        switch (raw.type) {
            case "stdout":
                return { type: "stdout", data: raw.data ?? "" };
            case "stderr":
                return { type: "stderr", data: raw.data ?? "" };
            case "exit":
                return {
                    type: "exit",
                    exitCode: raw.code ?? null,
                    timedOut: raw.timedOut ?? false,
                    signal: raw.signal,
                };
        }
    }

    /**
     * Execute a mod command with real-time NDJSON streaming output.
     *
     * Returns an async generator that yields {@link CommandEvent} objects
     * as the command produces output. The last event is always an `exit` event.
     *
     * @param request - Command execution parameters
     * @param signal - Optional AbortSignal for cancellation
     * @returns An async generator yielding command events
     *
     * @example
     * ```typescript
     * // Stream output from a long-running build
     * for await (const event of mods.streamCommand({ command: "build" })) {
     *   switch (event.type) {
     *     case "stdout": console.log(event.data); break;
     *     case "stderr": console.error(event.data); break;
     *     case "exit": console.log("Done, exit code:", event.exitCode); break;
     *   }
     * }
     * ```
     */
    export async function* streamCommand(
        request: ExecuteCommandRequest,
        signal?: AbortSignal,
    ): AsyncGenerator<CommandEvent> {
        const stream = host.postStream<RawCommandEvent>(
            host.createUrl("commands/execute"),
            toRequestBody(request),
            signal,
        );
        for await (const raw of stream) {
            yield toCommandEvent(raw);
        }
    }

    /**
     * Execute a mod command and collect the full result.
     *
     * This is a convenience wrapper around {@link streamCommand} that buffers
     * all output and returns a single {@link CommandResult}.
     *
     * @param request - Command execution parameters
     * @param signal - Optional AbortSignal for cancellation
     * @returns The collected command result with exit code, stdout, and stderr
     *
     * @example
     * ```typescript
     * // Simple execution
     * const result = await mods.executeCommand({ command: "build" });
     *
     * // With arguments
     * const result = await mods.executeCommand({
     *   command: "compile",
     *   args: ["--target", "es2020"],
     * });
     *
     * // With stdin data and custom timeout
     * const result = await mods.executeCommand({
     *   command: "transform",
     *   stdin: JSON.stringify({ input: "data" }),
     *   timeoutMs: 60000,
     * });
     * ```
     */
    export async function executeCommand(
        request: ExecuteCommandRequest,
        signal?: AbortSignal,
    ): Promise<CommandResult> {
        const stdoutLines: string[] = [];
        const stderrLines: string[] = [];
        let exitCode: number | null = null;
        let timedOut = false;
        let exitSignal: string | undefined;

        for await (const event of streamCommand(request, signal)) {
            switch (event.type) {
                case "stdout":
                    stdoutLines.push(event.data);
                    break;
                case "stderr":
                    stderrLines.push(event.data);
                    break;
                case "exit":
                    exitCode = event.exitCode;
                    timedOut = event.timedOut;
                    exitSignal = event.signal;
                    break;
            }
        }

        return {
            exitCode,
            timedOut,
            signal: exitSignal,
            stdout: stdoutLines.join("\n"),
            stderr: stderrLines.join("\n"),
        };
    }

    /**
     * Metadata for a mod-registered context menu item.
     *
     * @example
     * ```typescript
     * const items = await mods.menus();
     * for (const item of items) {
     *   console.log(`${item.modName}: ${item.text}`);
     * }
     * ```
     */
    export interface ModMenuMetadata {
        /** Unique identifier for the menu item. */
        id: string;
        /** The mod package name that registered this menu item. */
        modName: string;
        /** Display text shown in the context menu. */
        text: string;
        /** Bin command to execute when the menu item is selected. */
        command: string;
    }

    /**
     * Returns all registered mod menu items.
     *
     * Menu items are declared in each mod's `package.json` under the
     * `homunculus.menus` field and collected at startup.
     *
     * @example
     * ```typescript
     * import { mods } from "@hmcs/sdk";
     *
     * const menuItems = await mods.menus();
     * for (const item of menuItems) {
     *   console.log(`${item.modName}: ${item.text}`);
     * }
     * ```
     */
    export async function menus(): Promise<ModMenuMetadata[]> {
        const response = await host.get(host.createUrl("mods/menus"));
        return await response.json();
    }
}
