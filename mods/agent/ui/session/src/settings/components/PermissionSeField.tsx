import { audio } from '@hmcs/sdk';
import { AssetSelect } from '@hmcs/ui';
import { Play } from 'lucide-react';
import { usePermissionSe } from '../hooks/usePermissionSe';

export function PermissionSeField() {
  const { value, assetList, onChange, importSound, loading } = usePermissionSe();

  if (loading || value === undefined) return null;

  return (
    <div className="settings-label">
      Permission SE
      <span className="settings-label-desc">Sound effect played when permission is requested</span>
      <AssetSelect
        value={value}
        onValueChange={onChange}
        items={assetList}
        allowNone
        noneLabel="None"
        onBrowse={importSound}
        browseLabel="+ Add from local file..."
        renderAction={() =>
          value ? (
            <button
              type="button"
              className="settings-input"
              style={{
                padding: '4px 8px',
                cursor: 'pointer',
                display: 'flex',
                alignItems: 'center',
              }}
              onClick={() => audio.se.play(value).catch(() => {})}
              title="Preview"
            >
              <Play size={14} />
            </button>
          ) : null
        }
      />
    </div>
  );
}
