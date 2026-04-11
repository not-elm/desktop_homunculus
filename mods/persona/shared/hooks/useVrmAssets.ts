import { assets, fileDialog } from '@hmcs/sdk';
import { useCallback, useEffect, useState } from 'react';

export interface UseVrmAssetsReturn {
  modAssets: assets.AssetInfo[];
  localAssets: assets.AssetInfo[];
  /** Opens a file dialog, imports the selected VRM, refreshes the list, and returns the new asset ID. */
  importVrm: (personaId: string) => Promise<string | null>;
}

/**
 * Fetches the list of VRM assets and provides a file-import action.
 */
export function useVrmAssets(): UseVrmAssetsReturn {
  const [assetList, setAssetList] = useState<assets.AssetInfo[]>([]);

  const fetchAssets = useCallback(async () => {
    try {
      const list = await assets.list({ type: 'vrm' });
      setAssetList(list);
    } catch (e) {
      console.error('Failed to load VRM assets:', e);
    }
  }, []);

  useEffect(() => {
    fetchAssets();
  }, [fetchAssets]);

  const importVrm = useCallback(
    async (personaId: string): Promise<string | null> => {
      const path = await fileDialog.open({
        accept: ['.vrm'],
        title: 'Select VRM file',
      });
      if (!path) return null;

      try {
        const assetId = `vrm:local:${personaId}`;
        await assets.importAsset({
          sourcePath: path,
          assetId,
          assetType: 'vrm',
          description: `Imported VRM for ${personaId}`,
        });
        await fetchAssets();
        return assetId;
      } catch (e) {
        console.error('Failed to import VRM:', e);
        return null;
      }
    },
    [fetchAssets],
  );

  const modAssets = assetList.filter((a) => !a.id.startsWith('vrm:local:'));
  const localAssets = assetList.filter((a) => a.id.startsWith('vrm:local:'));

  return { modAssets, localAssets, importVrm };
}
