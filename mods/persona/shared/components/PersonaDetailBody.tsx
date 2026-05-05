import { Input, Switch } from '@hmcs/ui';
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
    <div className="flex flex-row gap-5">
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
    <div className="flex w-[160px] shrink-0 flex-col gap-3">
      <ThumbnailCard thumbnailUrl={thumbnailUrl} onThumbnailChange={onThumbnailChange} />

      <VrmSelect personaId={personaId} value={vrmAssetId} onChange={onVrmChange} />

      <div className="flex items-center justify-between gap-2">
        <div className="flex flex-col">
          <div className="text-xs text-muted-foreground">Auto Spawn</div>
          <div className="text-[0.7rem] tracking-[0.04em] text-hud-text-subdued">
            Launch at startup
          </div>
        </div>
        <Switch
          checked={autoSpawn}
          onCheckedChange={() => onAutoSpawnToggle()}
          aria-label="Toggle auto spawn"
          className="h-4 w-8 [&>span]:size-3 [&>span]:data-[state=checked]:translate-x-4"
        />
      </div>
    </div>
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
    <div className="flex min-w-0 flex-1 flex-col gap-3">
      <div className="flex flex-col gap-1.5">
        <div className="text-xs uppercase tracking-[0.1em] text-primary/70">ID</div>
        <Input value={personaId} readOnly className="cursor-not-allowed opacity-50" />
      </div>
      <PersonaFields values={formValues} onChange={onFormChange} />
    </div>
  );
}
