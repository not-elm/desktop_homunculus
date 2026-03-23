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
 * const result = await dialog.pickFolder();
 * if (result) {
 *   console.log(`Selected: ${result}`);
 * }
 * ```
 */
export namespace dialog {
  /**
   * Opens a native OS directory picker dialog.
   *
   * Displays the platform's standard folder selection dialog. Returns the
   * selected directory path as a string, or `null` if the user cancels.
   *
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
  export async function pickFolder(): Promise<string | null> {
    const response = await host.post(host.createUrl("dialog/pick-folder"));
    const body = (await response.json()) as { path: string | null };
    return body.path;
  }
}
