import { AssetSelect, type AssetSelectGroup } from '@hmcs/ui';
import { useVrmAssets } from '../hooks/useVrmAssets';

interface VrmSelectProps {
  personaId: string;
  value: string | null;
  onChange: (assetId: string | null) => void;
  disabled?: boolean;
}

export default function VrmSelect({ personaId, value, onChange, disabled }: VrmSelectProps) {
  const { modAssets, localAssets, importVrm } = useVrmAssets();

  const groups: AssetSelectGroup[] = [
    ...(modAssets.length > 0 ? [{ label: 'MOD', items: modAssets }] : []),
    ...(localAssets.length > 0 ? [{ label: 'LOCAL', items: localAssets }] : []),
  ];

  async function handleBrowse() {
    const assetId = await importVrm(personaId);
    if (assetId) onChange(assetId);
  }

  return (
    <div className="detail-field">
      <div className="detail-field-label">VRM Model</div>
      <AssetSelect
        value={value}
        onValueChange={onChange}
        items={groups}
        disabled={disabled}
        onBrowse={handleBrowse}
        browseLabel="+ Browse for local .vrm file..."
      />
    </div>
  );
}
