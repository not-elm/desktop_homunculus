import { useCallback } from "react";
import { assets, fileDialog } from "@hmcs/sdk";

export interface UseThumbnailImportReturn {
  /** Opens a file dialog, imports the selected image, and returns the new asset ID. */
  importThumbnail: (personaId: string) => Promise<string | null>;
}

/**
 * Provides a file-import action for persona thumbnail images.
 *
 * Follows the same pattern as useVrmAssets: native file dialog → asset import → return asset ID.
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
        const assetId = `image:local:${personaId}`;
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
