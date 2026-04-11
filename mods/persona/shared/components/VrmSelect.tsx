import type { assets } from '@hmcs/sdk';
import { useCallback, useRef, useState } from 'react';
import { useClickOutside } from '../hooks/useClickOutside';
import { useVrmAssets } from '../hooks/useVrmAssets';

interface VrmSelectProps {
  personaId: string;
  value: string | null;
  onChange: (assetId: string | null) => void;
  disabled?: boolean;
}

export default function VrmSelect({ personaId, value, onChange, disabled }: VrmSelectProps) {
  const [open, setOpen] = useState(false);
  const containerRef = useRef<HTMLDivElement>(null);
  const { modAssets, localAssets, importVrm } = useVrmAssets();

  useClickOutside(
    containerRef,
    useCallback(() => setOpen(false), []),
  );

  async function handleBrowse() {
    setOpen(false);
    const assetId = await importVrm(personaId);
    if (assetId) {
      onChange(assetId);
    }
  }

  function handleSelect(assetId: string) {
    onChange(assetId);
    setOpen(false);
  }

  const displayValue = value ?? 'None';

  return (
    <div className="detail-field" ref={containerRef} style={{ position: 'relative' }}>
      <div className="detail-field-label">VRM Model</div>
      <button
        type="button"
        className="settings-input"
        style={{
          width: '100%',
          textAlign: 'left',
          cursor: disabled ? 'not-allowed' : 'pointer',
          display: 'flex',
          justifyContent: 'space-between',
          alignItems: 'center',
        }}
        onClick={() => !disabled && setOpen((prev) => !prev)}
        disabled={disabled}
      >
        <span
          style={{
            overflow: 'hidden',
            textOverflow: 'ellipsis',
            whiteSpace: 'nowrap',
          }}
        >
          {displayValue}
        </span>
        <span style={{ opacity: 0.5, fontSize: 10 }}>{open ? '\u25B2' : '\u25BC'}</span>
      </button>

      {open && (
        <DropdownPanel
          modAssets={modAssets}
          localAssets={localAssets}
          selectedValue={value}
          onSelect={handleSelect}
          onBrowse={handleBrowse}
        />
      )}
    </div>
  );
}

function DropdownPanel({
  modAssets,
  localAssets,
  selectedValue,
  onSelect,
  onBrowse,
}: {
  modAssets: assets.AssetInfo[];
  localAssets: assets.AssetInfo[];
  selectedValue: string | null;
  onSelect: (assetId: string) => void;
  onBrowse: () => void;
}) {
  return (
    <div
      style={{
        position: 'absolute',
        top: '100%',
        left: 0,
        right: 0,
        marginTop: 4,
        background: 'oklch(0.15 0.01 250 / 0.95)',
        border: '1px solid oklch(0.72 0.14 192 / 0.25)',
        borderRadius: 6,
        maxHeight: 240,
        overflowY: 'auto',
        zIndex: 50,
        scrollbarWidth: 'none',
      }}
    >
      {modAssets.length > 0 && (
        <AssetGroup
          label="MOD"
          assets={modAssets}
          selectedValue={selectedValue}
          onSelect={onSelect}
        />
      )}

      {localAssets.length > 0 && (
        <AssetGroup
          label="LOCAL"
          assets={localAssets}
          selectedValue={selectedValue}
          onSelect={onSelect}
        />
      )}

      <div
        style={{
          height: 1,
          background: 'oklch(0.72 0.14 192 / 0.1)',
          margin: '4px 0',
        }}
      />

      <button
        type="button"
        onClick={onBrowse}
        style={{
          width: '100%',
          padding: '8px 12px',
          background: 'transparent',
          border: 'none',
          color: 'oklch(0.72 0.14 192 / 0.7)',
          fontSize: 12,
          letterSpacing: '0.05em',
          textAlign: 'left',
          cursor: 'pointer',
          fontFamily: 'inherit',
        }}
        onMouseEnter={(e) => {
          e.currentTarget.style.background = 'oklch(0.72 0.14 192 / 0.08)';
        }}
        onMouseLeave={(e) => {
          e.currentTarget.style.background = 'transparent';
        }}
      >
        + Browse for local .vrm file...
      </button>
    </div>
  );
}

function AssetGroup({
  label,
  assets: items,
  selectedValue,
  onSelect,
}: {
  label: string;
  assets: assets.AssetInfo[];
  selectedValue: string | null;
  onSelect: (assetId: string) => void;
}) {
  return (
    <div>
      <div
        style={{
          fontSize: 9,
          letterSpacing: '0.15em',
          textTransform: 'uppercase',
          color: 'oklch(0.55 0.02 250)',
          padding: '8px 12px 4px',
        }}
      >
        {label}
      </div>
      {items.map((asset) => (
        <button
          key={asset.id}
          type="button"
          onClick={() => onSelect(asset.id)}
          style={{
            width: '100%',
            padding: '6px 12px',
            background: asset.id === selectedValue ? 'oklch(0.72 0.14 192 / 0.12)' : 'transparent',
            border: 'none',
            color: 'oklch(0.88 0.02 250)',
            fontSize: 12,
            textAlign: 'left',
            cursor: 'pointer',
            fontFamily: 'monospace',
            letterSpacing: '0.3px',
          }}
          onMouseEnter={(e) => {
            if (asset.id !== selectedValue) {
              e.currentTarget.style.background = 'oklch(0.72 0.14 192 / 0.06)';
            }
          }}
          onMouseLeave={(e) => {
            if (asset.id !== selectedValue) {
              e.currentTarget.style.background = 'transparent';
            }
          }}
        >
          {asset.id}
          {asset.description && (
            <span
              style={{
                display: 'block',
                fontSize: 10,
                color: 'oklch(0.55 0.02 250)',
                fontFamily: 'inherit',
              }}
            >
              {asset.description}
            </span>
          )}
        </button>
      ))}
    </div>
  );
}
