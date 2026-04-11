import { assets, fileDialog, Persona as SdkPersona, Webview } from '@hmcs/sdk';
import { useCallback, useEffect, useState } from 'react';

// Re-define locally to avoid cross-entry-point import from ../../../../lib/.
// Must match the value in lib/permission-se.ts.
const DEFAULT_PERMISSION_SE = 'agent:request-permissions';

export interface UsePermissionSeReturn {
  /** Currently selected SE asset ID, or null if disabled. undefined while loading. */
  value: string | null | undefined;
  /** All sound assets (mod + local). */
  assetList: assets.AssetInfo[];
  /** Update the selected SE. Pass null to disable, or an asset ID string. */
  onChange: (assetId: string | null) => Promise<void>;
  /** Open file dialog, import a sound file, and select it. */
  importSound: () => Promise<void>;
  /** Whether the initial load is in progress. */
  loading: boolean;
}

export function usePermissionSe(): UsePermissionSeReturn {
  const [value, setValue] = useState<string | null | undefined>(undefined);
  const [assetList, setAssetList] = useState<assets.AssetInfo[]>([]);
  const [loading, setLoading] = useState(true);
  const [personaId, setPersonaId] = useState<string | null>(null);

  const fetchAssets = useCallback(async () => {
    try {
      const list = await assets.list({ type: 'sound' });
      setAssetList(list);
    } catch (e) {
      console.error('Failed to load sound assets:', e);
    }
  }, []);

  useEffect(() => {
    let cancelled = false;
    (async () => {
      try {
        const wv = await Webview.current();
        const p = wv ? await wv.linkedPersona() : null;
        if (cancelled) return;
        const id = p ? p.id : null;
        setPersonaId(id);

        const [, metadata] = await Promise.all([
          fetchAssets(),
          id ? SdkPersona.load(id).then((persona) => persona.metadata()) : undefined,
        ]);
        if (cancelled) return;

        if (metadata && metadata.permissionSe !== undefined) {
          setValue(metadata.permissionSe as string | null);
        } else {
          setValue(DEFAULT_PERMISSION_SE);
        }
      } catch (e) {
        console.error('Failed to load permission SE settings:', e);
        setValue(DEFAULT_PERMISSION_SE);
      } finally {
        if (!cancelled) setLoading(false);
      }
    })();
    return () => {
      cancelled = true;
    };
  }, [fetchAssets]);

  const onChange = useCallback(
    async (assetId: string | null) => {
      setValue(assetId);
      if (!personaId) return;
      try {
        const p = await SdkPersona.load(personaId);
        const existing = await p.metadata();
        await p.setMetadata({ ...existing, permissionSe: assetId });
      } catch (e) {
        console.error('Failed to save permission SE setting:', e);
      }
    },
    [personaId],
  );

  const importSound = useCallback(async () => {
    if (!personaId) return;
    const path = await fileDialog.open({
      accept: ['.mp3', '.wav', '.ogg'],
      title: 'Select sound file',
    });
    if (!path) return;

    try {
      const assetId = `se:local:${personaId}:${Date.now()}`;
      await assets.importAsset({
        sourcePath: path,
        assetId,
        assetType: 'sound',
        description: path.split('/').pop() ?? 'Imported SE',
      });
      await fetchAssets();
      await onChange(assetId);
    } catch (e) {
      console.error('Failed to import sound:', e);
    }
  }, [personaId, fetchAssets, onChange]);

  return { value, assetList, onChange, importSound, loading };
}
