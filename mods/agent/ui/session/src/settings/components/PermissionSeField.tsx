import type { assets } from '@hmcs/sdk';
import { audio } from '@hmcs/sdk';
import { Play } from 'lucide-react';
import { type RefObject, useCallback, useEffect, useRef, useState } from 'react';
import { usePermissionSe } from '../hooks/usePermissionSe';

/** Inline useClickOutside — agent mod does not have a shared one. */
function useClickOutside(ref: RefObject<HTMLElement | null>, handler: () => void) {
  useEffect(() => {
    function listener(e: MouseEvent) {
      if (ref.current && !ref.current.contains(e.target as Node)) handler();
    }
    document.addEventListener('mousedown', listener);
    return () => document.removeEventListener('mousedown', listener);
  }, [ref, handler]);
}

export function PermissionSeField() {
  const { value, assetList, onChange, importSound, loading } = usePermissionSe();

  if (loading || value === undefined) return null;

  return (
    <div className="settings-label">
      Permission SE
      <span className="settings-label-desc">Sound effect played when permission is requested</span>
      <SeDropdown
        value={value}
        assetList={assetList}
        onChange={onChange}
        onBrowse={importSound}
      />
    </div>
  );
}

function SeDropdown({
  value,
  assetList,
  onChange,
  onBrowse,
}: {
  value: string | null;
  assetList: assets.AssetInfo[];
  onChange: (assetId: string | null) => Promise<void>;
  onBrowse: () => Promise<void>;
}) {
  const [open, setOpen] = useState(false);
  const containerRef = useRef<HTMLDivElement>(null);

  useClickOutside(
    containerRef,
    useCallback(() => setOpen(false), []),
  );

  function handleSelect(assetId: string | null) {
    onChange(assetId);
    setOpen(false);
  }

  async function handleBrowse() {
    setOpen(false);
    await onBrowse();
  }

  function handlePreview() {
    if (value) {
      audio.se.play(value).catch(() => {});
    }
  }

  const displayValue = value === null ? 'None' : value;

  return (
    <div ref={containerRef} style={{ position: 'relative' }}>
      <div style={{ display: 'flex', gap: 4, alignItems: 'center' }}>
        <button
          type="button"
          className="settings-input"
          style={{
            flex: 1,
            textAlign: 'left',
            cursor: 'pointer',
            display: 'flex',
            justifyContent: 'space-between',
            alignItems: 'center',
          }}
          onClick={() => setOpen((prev) => !prev)}
        >
          <span
            style={{
              overflow: 'hidden',
              textOverflow: 'ellipsis',
              whiteSpace: 'nowrap',
              opacity: value === null ? 0.5 : 1,
            }}
          >
            {displayValue}
          </span>
          <span style={{ opacity: 0.5, fontSize: 10 }}>{open ? '\u25B2' : '\u25BC'}</span>
        </button>
        {value && (
          <button
            type="button"
            className="settings-input"
            style={{
              padding: '4px 8px',
              cursor: 'pointer',
              display: 'flex',
              alignItems: 'center',
            }}
            onClick={handlePreview}
            title="Preview"
          >
            <Play size={14} />
          </button>
        )}
      </div>

      {open && (
        <SeDropdownPanel
          assetList={assetList}
          selectedValue={value}
          onSelect={handleSelect}
          onBrowse={handleBrowse}
        />
      )}
    </div>
  );
}

function SeDropdownPanel({
  assetList,
  selectedValue,
  onSelect,
  onBrowse,
}: {
  assetList: assets.AssetInfo[];
  selectedValue: string | null;
  onSelect: (assetId: string | null) => void;
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
      <SeOption
        label="None"
        description="Disable permission SE"
        selected={selectedValue === null}
        onClick={() => onSelect(null)}
      />

      <div
        style={{
          height: 1,
          background: 'oklch(0.72 0.14 192 / 0.1)',
          margin: '4px 0',
        }}
      />

      {assetList.map((asset) => (
        <SeOption
          key={asset.id}
          label={asset.id}
          description={asset.description}
          selected={asset.id === selectedValue}
          onClick={() => onSelect(asset.id)}
        />
      ))}

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
        + Add from local file...
      </button>
    </div>
  );
}

function SeOption({
  label,
  description,
  selected,
  onClick,
}: {
  label: string;
  description?: string;
  selected: boolean;
  onClick: () => void;
}) {
  return (
    <button
      type="button"
      onClick={onClick}
      style={{
        width: '100%',
        padding: '6px 12px',
        background: selected ? 'oklch(0.72 0.14 192 / 0.12)' : 'transparent',
        border: 'none',
        color: 'oklch(0.88 0.02 250)',
        fontSize: 12,
        textAlign: 'left',
        cursor: 'pointer',
        fontFamily: 'monospace',
        letterSpacing: '0.3px',
      }}
      onMouseEnter={(e) => {
        if (!selected) e.currentTarget.style.background = 'oklch(0.72 0.14 192 / 0.06)';
      }}
      onMouseLeave={(e) => {
        if (!selected) e.currentTarget.style.background = 'transparent';
      }}
    >
      {label}
      {description && (
        <span
          style={{
            display: 'block',
            fontSize: 10,
            color: 'oklch(0.55 0.02 250)',
            fontFamily: 'inherit',
          }}
        >
          {description}
        </span>
      )}
    </button>
  );
}
