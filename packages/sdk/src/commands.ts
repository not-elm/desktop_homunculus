/**
 * Command script utilities for Desktop Homunculus mods.
 *
 * This module provides helpers for parsing stdin input and writing structured
 * output in mod command scripts (`bin/` scripts invoked via the HTTP command
 * execution API).
 *
 * **Input:** {@link input.parse} / {@link input.parseMenu} / {@link input.read}
 * **Output:** {@link output.succeed} / {@link output.fail} / {@link output.write} / {@link output.writeError}
 *
 * @remarks
 * This module uses Node.js APIs (`process.stdin`, `fs.writeFileSync`) and is
 * not browser-compatible. Import from `@hmcs/sdk/commands` — it is intentionally
 * not re-exported from the main `@hmcs/sdk` entry point.
 *
 * @example
 * ```typescript
 * import { z } from "zod";
 * import { input, output, StdinParseError } from "@hmcs/sdk/commands";
 *
 * const schema = z.object({ name: z.string() });
 *
 * try {
 *   const data = await input.parse(schema);
 *   output.succeed({ greeting: `Hello, ${data.name}!` });
 * } catch (err) {
 *   output.fail("GREET_FAILED", (err as Error).message);
 * }
 * ```
 *
 * @packageDocumentation
 */

import { writeFileSync } from "node:fs";
import { z, type ZodType } from "zod";
import { Vrm } from "./vrm";

/**
 * Safely serialize a value to JSON. Returns a fallback error JSON string
 * if serialization fails (e.g., circular references or BigInt values).
 */
function safeStringify(data: unknown): string {
  try {
    return JSON.stringify(data);
  } catch {
    return '{"code":"SERIALIZE_ERROR","message":"Failed to serialize output"}';
  }
}

/**
 * Error thrown by {@link input.parse} when stdin is empty, contains invalid JSON,
 * or fails Zod schema validation.
 *
 * @example
 * ```typescript
 * import { input, StdinParseError } from "@hmcs/sdk/commands";
 *
 * try {
 *   const data = await input.parse(schema);
 * } catch (err) {
 *   if (err instanceof StdinParseError) {
 *     console.error(JSON.stringify({ code: err.code, message: err.message }));
 *     process.exit(1);
 *   }
 *   throw err;
 * }
 * ```
 */
export class StdinParseError extends Error {
  override readonly name = "StdinParseError";

  constructor(
    /** Structured error code identifying the failure stage. */
    public readonly code: "EMPTY_STDIN" | "INVALID_JSON" | "VALIDATION_ERROR",
    message: string,
    /** For `VALIDATION_ERROR`, contains the `ZodError` instance. */
    public readonly details?: unknown,
  ) {
    super(message);
  }
}

/**
 * Input helpers for reading and parsing stdin in bin command scripts.
 *
 * @example
 * ```typescript
 * import { z } from "zod";
 * import { input } from "@hmcs/sdk/commands";
 *
 * const data = await input.parse(
 *   z.object({ name: z.string(), count: z.number() })
 * );
 * ```
 */
export namespace input {
  /**
   * Read all of stdin as a UTF-8 string.
   *
   * Consumes the entire `process.stdin` stream via async iteration and returns
   * the concatenated result. Useful when you need the raw string without JSON
   * parsing or validation.
   *
   * @example
   * ```typescript
   * import { input } from "@hmcs/sdk/commands";
   *
   * const raw = await input.read();
   * console.log("Received:", raw);
   * ```
   */
  export async function read(): Promise<string> {
    const chunks: Buffer[] = [];
    for await (const chunk of process.stdin) {
      chunks.push(chunk);
    }
    return Buffer.concat(chunks).toString("utf-8");
  }

  /**
   * Read JSON from stdin and validate it against a Zod schema.
   *
   * Performs three steps:
   * 1. Reads all of stdin via {@link input.read}
   * 2. Parses the raw string as JSON
   * 3. Validates the parsed object against the provided Zod schema
   *
   * @typeParam T - The output type inferred from the Zod schema
   * @param schema - A Zod schema to validate the parsed JSON against
   * @returns The validated and typed input object
   * @throws {StdinParseError} With `code: "EMPTY_STDIN"` if stdin is empty or whitespace-only
   * @throws {StdinParseError} With `code: "INVALID_JSON"` if stdin is not valid JSON
   * @throws {StdinParseError} With `code: "VALIDATION_ERROR"` if the JSON does not match the schema
   *
   * @example
   * ```typescript
   * import { z } from "zod";
   * import { input } from "@hmcs/sdk/commands";
   *
   * const data = await input.parse(
   *   z.object({
   *     entity: z.number(),
   *     text: z.union([z.string(), z.array(z.string())]),
   *     speaker: z.number().default(0),
   *   })
   * );
   * ```
   */
  export async function parse<T>(schema: ZodType<T>): Promise<T> {
    const raw = await read();

    if (raw.trim().length === 0) {
      throw new StdinParseError("EMPTY_STDIN", "No input received on stdin");
    }

    let json: unknown;
    try {
      json = JSON.parse(raw);
    } catch {
      throw new StdinParseError(
        "INVALID_JSON",
        `Invalid JSON: ${raw.slice(0, 200)}`,
      );
    }

    const result = schema.safeParse(json);
    if (!result.success) {
      throw new StdinParseError(
        "VALIDATION_ERROR",
        `Validation failed: ${result.error.message}`,
        result.error,
      );
    }

    return result.data;
  }

  /**
   * Parse menu command stdin and return the linked VRM instance.
   *
   * Menu commands receive `{ "linkedVrm": <entityId> }` on stdin from the
   * menu UI. This helper validates the input and returns a ready-to-use
   * {@link Vrm} instance.
   *
   * @returns A {@link Vrm} instance for the linked entity
   * @throws {StdinParseError} With `code: "EMPTY_STDIN"` if stdin is empty
   * @throws {StdinParseError} With `code: "INVALID_JSON"` if stdin is not valid JSON
   * @throws {StdinParseError} With `code: "VALIDATION_ERROR"` if `linkedVrm` is missing or not a number
   *
   * @example
   * ```typescript
   * import { input } from "@hmcs/sdk/commands";
   *
   * const vrm = await input.parseMenu();
   * await vrm.setExpressions({ happy: 1.0 });
   * ```
   */
  export async function parseMenu(): Promise<Vrm> {
    const parsed = await parse(z.object({ linkedVrm: z.number() }));
    return new Vrm(parsed.linkedVrm);
  }
}

/**
 * Output helpers for writing structured results and errors in bin command scripts.
 *
 * @example
 * ```typescript
 * import { output } from "@hmcs/sdk/commands";
 *
 * output.succeed({ count: 42, status: "done" });
 * ```
 */
export namespace output {
  /**
   * Write a JSON-serialized result to stdout (fd 1).
   *
   * Serializes `data` with `JSON.stringify` and writes it followed by a newline
   * to file descriptor 1 using synchronous I/O. This ensures the output is
   * flushed before the process exits.
   *
   * @param data - The value to serialize as JSON and write to stdout
   *
   * @example
   * ```typescript
   * import { output } from "@hmcs/sdk/commands";
   *
   * output.write({ count: 42, status: "done" });
   * // stdout receives: {"count":42,"status":"done"}\n
   * ```
   */
  export function write(data: unknown): void {
    writeFileSync(1, safeStringify(data) + "\n");
  }

  /**
   * Write a structured JSON error to stderr (fd 2).
   *
   * Serializes an object with `code` and `message` fields and writes it followed
   * by a newline to file descriptor 2 using synchronous I/O.
   *
   * @param code - A machine-readable error code (e.g., `"NOT_FOUND"`, `"TIMEOUT"`)
   * @param message - A human-readable error description
   *
   * @example
   * ```typescript
   * import { output } from "@hmcs/sdk/commands";
   *
   * output.writeError("NOT_FOUND", "Entity 42 does not exist");
   * // stderr receives: {"code":"NOT_FOUND","message":"Entity 42 does not exist"}\n
   * ```
   */
  export function writeError(code: string, message: string): void {
    writeFileSync(2, JSON.stringify({ code, message }) + "\n");
  }

  /**
   * Exit the process with code 0, optionally writing a JSON result to stdout.
   *
   * When called with a `data` argument, serializes it to stdout via
   * {@link output.write} before exiting. When called without arguments,
   * exits immediately without writing to stdout — useful for commands
   * that perform a side effect (e.g., opening a UI) with no return value.
   *
   * @param data - Optional value to serialize as JSON and write to stdout
   *
   * @example
   * ```typescript
   * import { input, output } from "@hmcs/sdk/commands";
   *
   * // With payload
   * const data = await input.parse(schema);
   * const result = await doWork(data);
   * output.succeed({ processed: result.count });
   *
   * // Without payload (side-effect only command)
   * await openWebview();
   * output.succeed();
   * ```
   */
  export function succeed(data?: unknown): never {
    if (data !== undefined) {
      write(data);
    }
    process.exit(0);
  }

  /**
   * Write a structured error to stderr and exit the process.
   *
   * This is a convenience wrapper that calls {@link output.writeError} followed by
   * `process.exit(exitCode)`. Use this when a bin command encounters a fatal error.
   *
   * @param code - A machine-readable error code (e.g., `"NOT_FOUND"`, `"TIMEOUT"`)
   * @param message - A human-readable error description
   * @param exitCode - Process exit code (default: `1`)
   *
   * @example
   * ```typescript
   * import { output } from "@hmcs/sdk/commands";
   *
   * if (!response.ok) {
   *   output.fail("API_ERROR", `Server returned ${response.status}`);
   * }
   * ```
   */
  export function fail(code: string, message: string, exitCode: number = 1): never {
    writeError(code, message);
    process.exit(exitCode);
  }
}
