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
    <div className="detail-body">
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
    <div className="detail-left">
      <ThumbnailCard thumbnailUrl={thumbnailUrl} onThumbnailChange={onThumbnailChange} />

      <VrmSelect personaId={personaId} value={vrmAssetId} onChange={onVrmChange} />

      <div className="detail-auto-row">
        <div>
          <div className="detail-auto-label">Auto Spawn</div>
          <div className="detail-auto-sublabel">Launch at startup</div>
        </div>
        <button
          type="button"
          className={`toggle-mini ${autoSpawn ? 'on' : 'off'}`}
          onClick={onAutoSpawnToggle}
          aria-label="Toggle auto spawn"
          role="switch"
          aria-checked={autoSpawn}
        >
          <span className="knob" />
        </button>
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
    <div className="detail-right">
      <div className="detail-field">
        <div className="detail-field-label">ID</div>
        <input
          type="text"
          className="settings-input"
          value={personaId}
          readOnly
          style={{ opacity: 0.5, cursor: 'not-allowed', width: '100%' }}
        />
      </div>
      <PersonaFields values={formValues} onChange={onFormChange} />
    </div>
  );
}
