import { cn } from '@hmcs/ui';
import { PersonaFields, type PersonaFormValues } from '@persona/shared/components/PersonaFields';
import { ThumbnailCard } from '@persona/shared/components/ThumbnailCard';
import VrmSelect from '@persona/shared/components/VrmSelect';

interface PersonaDetailBodyProps {
  personaId: string;
  thumbnailUrl: string | null;
  onThumbnailChange: () => void;
  vrmAssetId: string | null;
  onVrmChange: (id: string | null) => void;
  autoSpawn: boolean;
  onAutoSpawnToggle: () => void;
  formValues: PersonaFormValues;
  onFormChange: (values: PersonaFormValues) => void;
}

/**
 * Shared 2-column body for the persona detail view.
 * Left: thumbnail, VRM selector, auto-spawn toggle.
 * Right: readonly ID, persona form fields.
 */
export function PersonaDetailBody({
  personaId,
  thumbnailUrl,
  onThumbnailChange,
  vrmAssetId,
  onVrmChange,
  autoSpawn,
  onAutoSpawnToggle,
  formValues,
  onFormChange,
}: PersonaDetailBodyProps) {
  return (
    <div className="flex flex-1 gap-5 overflow-y-auto px-[18px] py-[14px] [scrollbar-width:none] [&::-webkit-scrollbar]:hidden">
      <LeftColumn
        personaId={personaId}
        thumbnailUrl={thumbnailUrl}
        onThumbnailChange={onThumbnailChange}
        vrmAssetId={vrmAssetId}
        onVrmChange={onVrmChange}
        autoSpawn={autoSpawn}
        onAutoSpawnToggle={onAutoSpawnToggle}
      />
      <RightColumn personaId={personaId} formValues={formValues} onFormChange={onFormChange} />
    </div>
  );
}

function LeftColumn({
  personaId,
  thumbnailUrl,
  onThumbnailChange,
  vrmAssetId,
  onVrmChange,
  autoSpawn,
  onAutoSpawnToggle,
}: {
  personaId: string;
  thumbnailUrl: string | null;
  onThumbnailChange: () => void;
  vrmAssetId: string | null;
  onVrmChange: (id: string | null) => void;
  autoSpawn: boolean;
  onAutoSpawnToggle: () => void;
}) {
  return (
    <div className="w-[140px] flex-shrink-0">
      <ThumbnailCard thumbnailUrl={thumbnailUrl} onThumbnailChange={onThumbnailChange} />

      <VrmSelect personaId={personaId} value={vrmAssetId} onChange={onVrmChange} />

      <div className="mt-3 flex items-center justify-between">
        <div>
          <div className="text-xs tracking-[0.5px] text-[oklch(0.55_0.02_250)]">Auto Spawn</div>
          <div className="mt-0.5 text-[10px] tracking-[0.3px] text-[oklch(0.45_0.02_250)]">
            Launch at startup
          </div>
        </div>
        <AutoSpawnToggle enabled={autoSpawn} onToggle={onAutoSpawnToggle} />
      </div>
    </div>
  );
}

function AutoSpawnToggle({ enabled, onToggle }: { enabled: boolean; onToggle: () => void }) {
  return (
    <button
      type="button"
      className={cn(
        'relative h-4 w-8 cursor-pointer rounded-lg border transition-all duration-[250ms] ease-out',
        enabled
          ? 'border-[oklch(0.72_0.14_192/0.5)] bg-[oklch(0.72_0.14_192/0.35)]'
          : 'border-[oklch(0.4_0.02_250/0.4)] bg-[oklch(0.25_0.01_250)]',
      )}
      onClick={onToggle}
      aria-label="Toggle auto spawn"
      role="switch"
      aria-checked={enabled}
    >
      <span
        className={cn(
          'absolute top-px h-3 w-3 rounded-full transition-all duration-[250ms] ease-out',
          enabled
            ? 'right-px bg-[oklch(0.72_0.14_192)] shadow-[0_0_6px_oklch(0.72_0.14_192/0.4)]'
            : 'left-px bg-[oklch(0.45_0.02_250)]',
        )}
      />
    </button>
  );
}

function RightColumn({
  personaId,
  formValues,
  onFormChange,
}: {
  personaId: string;
  formValues: PersonaFormValues;
  onFormChange: (values: PersonaFormValues) => void;
}) {
  return (
    <div className="min-w-0 flex-1">
      <div className="mb-[14px]">
        <div className="mb-1.5 text-[9px] font-normal uppercase tracking-[0.15em] text-[oklch(0.72_0.14_192/0.4)]">
          ID
        </div>
        <input
          type="text"
          className="settings-input w-full cursor-not-allowed opacity-50"
          value={personaId}
          readOnly
        />
      </div>
      <PersonaFields values={formValues} onChange={onFormChange} />
    </div>
  );
}
