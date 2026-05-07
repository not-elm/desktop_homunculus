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
    <div className="mb-[14px]">
      <div className="mb-1.5 text-[9px] font-normal uppercase tracking-[0.15em] text-[oklch(0.72_0.14_192/0.4)]">
        VRM Model
      </div>
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
