/**
 * Native file dialog for selecting files from the filesystem.
 *
 * @example
 * ```typescript
 * import { fileDialog } from "@hmcs/sdk";
 *
 * const path = await fileDialog.open({ accept: [".vrm"], title: "Select VRM" });
 * if (path) {
 *   console.log("Selected:", path);
 * }
 * ```
 *
 * @module
 */

import { host } from "./host.js";

/** Options for the native file dialog. */
export interface FileDialogOptions {
    /** File type filters (e.g., [".vrm", ".png", "image/jpeg"]) */
    accept?: string[];
    /** Dialog window title */
    title?: string;
}

/**
 * Opens a native file dialog and returns the selected file path.
 *
 * @returns The selected file path, or `null` if the user cancelled
 *
 * @example
 * ```typescript
 * const path = await fileDialog.open({
 *   accept: [".vrm"],
 *   title: "Select VRM file",
 * });
 * if (path) {
 *   console.log("Selected:", path);
 * }
 * ```
 */
export async function open(options?: FileDialogOptions): Promise<string | null> {
    const filters = buildFilters(options?.accept);
    const response = await host.post(host.createUrl("dialog/pick-file"), {
        title: options?.title,
        filters,
    });
    const body = await response.json() as { path: string | null };
    return body.path;
}

/** Converts extension strings (e.g. `[".vrm", ".png"]`) into backend `FileFilter` format. */
function buildFilters(accept?: string[]): { name: string; extensions: string[] }[] | undefined {
    if (!accept || accept.length === 0) return undefined;
    const extensions = accept.map((ext) => ext.replace(/^\./, ""));
    return [{ name: extensions.map((e) => e.toUpperCase()).join(", "), extensions }];
}
