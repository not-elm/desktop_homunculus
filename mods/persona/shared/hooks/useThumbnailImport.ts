import { useCallback } from "react";
import { assets, fileDialog } from "@hmcs/sdk";

export interface UseThumbnailImportReturn {
  /** Opens a file dialog, imports the selected image, and returns the new asset ID. */
  importThumbnail: (personaId: string) => Promise<string | null>;
}

/**
 * Provides a file-import action for persona thumbnail images.
 *
 * The returned asset ID embeds a deterministic hash of the source file path
 * as an extra segment (`image:local:${personaId}:${pathHash}`). Picking a
 * different file yields a different ID, which forces the HTTP URL used by
 * `<img>` to change and the browser to re-fetch. Picking the same file
 * yields the same ID and is a no-op.
 */
export function useThumbnailImport(): UseThumbnailImportReturn {
  const importThumbnail = useCallback(
    async (personaId: string): Promise<string | null> => {
      const path = await fileDialog.open({
        accept: [".png", ".jpg", ".jpeg", ".webp"],
        title: "Select thumbnail image",
      });
      if (!path) return null;

      try {
        const hash = await hashPath(path);
        const assetId = `image:local:${personaId}:${hash}`;
        await assets.importAsset({
          sourcePath: path,
          assetId,
          assetType: "image",
          description: `Thumbnail for ${personaId}`,
        });
        return assetId;
      } catch (e) {
        console.error("Failed to import thumbnail:", e);
        return null;
      }
    },
    [],
  );

  return { importThumbnail };
}

/**
 * Returns a deterministic 16-character lowercase hex string derived from the
 * first 8 bytes of SHA-256(path). Same input → same output. 64 bits is more
 * than enough collision resistance for the set of file paths a single user
 * will ever pick.
 */
async function hashPath(path: string): Promise<string> {
  const bytes = new TextEncoder().encode(path);
  const digest = await crypto.subtle.digest("SHA-256", bytes);
  return Array.from(new Uint8Array(digest).slice(0, 8))
    .map((b) => b.toString(16).padStart(2, "0"))
    .join("");
}
