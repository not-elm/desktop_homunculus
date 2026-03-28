import { host } from "./host";

/**
 * Native OS dialog API namespace for file and directory selection.
 *
 * Provides access to native OS file picker dialogs. These dialogs are
 * rendered by the operating system and appear as standard system windows.
 *
 * @example
 * ```typescript
 * import { dialog } from "@hmcs/sdk";
 *
 * // Pick a single file
 * const file = await dialog.pickFile({ filters: [{ name: "Images", extensions: ["png", "jpg"] }] });
 *
 * // Pick multiple files
 * const files = await dialog.pickFiles();
 *
 * // Pick a folder
 * const folder = await dialog.pickFolder();
 * ```
 */
export namespace dialog {
  /**
   * A file type filter for native file dialogs.
   *
   * @example
   * ```typescript
   * const imageFilter: dialog.FileFilter = {
   *   name: "Images",
   *   extensions: ["png", "jpg", "jpeg", "gif"]
   * };
   * ```
   */
  export interface FileFilter {
    /** Display name for the filter (e.g. "Images"). */
    name: string;
    /** File extensions without leading dot (e.g. `["png", "jpg"]`). */
    extensions: string[];
  }

  /** Options for file picker dialogs. */
  export interface PickFileOptions {
    /** Dialog window title. */
    title?: string;
    /** Initial directory to open the dialog in. */
    defaultPath?: string;
    /** File type filters shown in the dialog. */
    filters?: FileFilter[];
  }

  /** Options for folder picker dialog. */
  export interface PickFolderOptions {
    /** Dialog window title. */
    title?: string;
    /** Initial directory to open the dialog in. */
    defaultPath?: string;
  }

  /**
   * Opens a native OS single-file picker dialog.
   *
   * Displays the platform's standard file selection dialog. Returns the
   * selected file path as a string, or `null` if the user cancels.
   *
   * @param options - Optional dialog configuration
   * @returns The selected file path, or `null` if cancelled
   *
   * @example
   * ```typescript
   * const file = await dialog.pickFile({
   *   title: "Select a VRM model",
   *   filters: [{ name: "VRM", extensions: ["vrm"] }],
   *   defaultPath: "/Users/me/models"
   * });
   * if (file) {
   *   console.log(`Selected: ${file}`);
   * }
   * ```
   */
  export async function pickFile(options?: PickFileOptions): Promise<string | null> {
    const response = await host.post(host.createUrl("dialog/pick-file"), options);
    const body = (await response.json()) as { path: string | null };
    return body.path;
  }

  /**
   * Opens a native OS multi-file picker dialog.
   *
   * Displays the platform's standard multi-file selection dialog. Returns
   * an array of selected file paths, or an empty array if the user cancels.
   *
   * @param options - Optional dialog configuration
   * @returns Array of selected file paths (empty if cancelled)
   *
   * @example
   * ```typescript
   * const files = await dialog.pickFiles({
   *   title: "Select images",
   *   filters: [{ name: "Images", extensions: ["png", "jpg", "jpeg"] }]
   * });
   * for (const file of files) {
   *   console.log(`Selected: ${file}`);
   * }
   * ```
   */
  export async function pickFiles(options?: PickFileOptions): Promise<string[]> {
    const response = await host.post(host.createUrl("dialog/pick-files"), options);
    const body = (await response.json()) as { paths: string[] };
    return body.paths;
  }

  /**
   * Opens a native OS directory picker dialog.
   *
   * Displays the platform's standard folder selection dialog. Returns the
   * selected directory path as a string, or `null` if the user cancels.
   *
   * @param options - Optional dialog configuration
   * @returns The selected directory path, or `null` if cancelled
   *
   * @example
   * ```typescript
   * const folder = await dialog.pickFolder();
   * if (folder) {
   *   console.log(`User chose: ${folder}`);
   * }
   * ```
   */
  export async function pickFolder(options?: PickFolderOptions): Promise<string | null> {
    const response = await host.post(host.createUrl("dialog/pick-folder"), options);
    const body = (await response.json()) as { path: string | null };
    return body.path;
  }
}
