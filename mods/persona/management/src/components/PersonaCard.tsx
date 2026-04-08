import { Persona, type PersonaSnapshot } from "@hmcs/sdk";

interface PersonaCardProps {
  persona: PersonaSnapshot;
  onEdit: (id: string) => void;
  onDelete: (id: string) => void;
  onSpawn: (id: string) => void;
  onDespawn: (id: string) => void;
  onAutoSpawnChange: (id: string, value: boolean) => void;
}

export default function PersonaCard({
  persona,
  onEdit,
  onDelete,
  onSpawn,
  onDespawn,
  onAutoSpawnChange,
}: PersonaCardProps) {
  const isSpawned = persona.spawned;
  const autoSpawn = persona.metadata?.["auto-spawn"] === true;
  const thumbnailUrl = new Persona(persona.id).thumbnailUrl();

  return (
    <div
      className={`persona-card ${isSpawned ? "spawned" : "not-spawned"}`}
      onClick={() => onEdit(persona.id)}
    >
      {isSpawned && <CornerAccents />}

      <CardHeader
        thumbnailUrl={thumbnailUrl}
        name={persona.name ?? persona.id}
        vrmAssetId={persona.vrmAssetId}
        profile={persona.profile}
      />

      <SpawnArea
        isSpawned={isSpawned}
        onSpawn={() => onSpawn(persona.id)}
        onDespawn={() => onDespawn(persona.id)}
      />

      <SettingsRow
        autoSpawn={autoSpawn}
        onToggle={(value) => onAutoSpawnChange(persona.id, value)}
      />

      <CardActions
        onEdit={() => onEdit(persona.id)}
        onDelete={() => onDelete(persona.id)}
      />
    </div>
  );
}

function CornerAccents() {
  return (
    <>
      <div className="card-corner card-corner--tl" />
      <div className="card-corner card-corner--tr" />
      <div className="card-corner card-corner--bl" />
      <div className="card-corner card-corner--br" />
    </>
  );
}

function CardHeader({
  thumbnailUrl,
  name,
  vrmAssetId,
  profile,
}: {
  thumbnailUrl: string;
  name: string;
  vrmAssetId?: string | null;
  profile: string;
}) {
  return (
    <div className="card-header">
      <div className="card-thumb">
        <div className="card-thumb-inner">
          <img src={thumbnailUrl} alt={name} />
        </div>
      </div>
      <div className="card-info">
        <div className="card-name">{name}</div>
        {vrmAssetId && <div className="card-vrm">{vrmAssetId}</div>}
        {profile && <div className="card-profile">{profile}</div>}
      </div>
    </div>
  );
}

function SpawnArea({
  isSpawned,
  onSpawn,
  onDespawn,
}: {
  isSpawned: boolean;
  onSpawn: () => void;
  onDespawn: () => void;
}) {
  function handleClick(e: React.MouseEvent) {
    e.stopPropagation();
    if (isSpawned) {
      onDespawn();
    } else {
      onSpawn();
    }
  }

  return (
    <div className="card-spawn-area">
      <div className={`status-dot ${isSpawned ? "active" : "inactive"}`} />
      <span className={`status-label ${isSpawned ? "active" : "inactive"}`}>
        {isSpawned ? "Online" : "Offline"}
      </span>
      <button
        className={`spawn-btn ${isSpawned ? "deactivate" : "activate"}`}
        onClick={handleClick}
      >
        {isSpawned ? "Despawn" : "Spawn"}
      </button>
    </div>
  );
}

function SettingsRow({
  autoSpawn,
  onToggle,
}: {
  autoSpawn: boolean;
  onToggle: (value: boolean) => void;
}) {
  function handleClick(e: React.MouseEvent) {
    e.stopPropagation();
    onToggle(!autoSpawn);
  }

  return (
    <div className="card-settings">
      <div className="auto-spawn-row">
        <span className="auto-spawn-label">Auto spawn at startup</span>
        <button
          className={`toggle-mini ${autoSpawn ? "on" : "off"}`}
          onClick={handleClick}
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

function CardActions({
  onEdit,
  onDelete,
}: {
  onEdit: () => void;
  onDelete: () => void;
}) {
  function handleEdit(e: React.MouseEvent) {
    e.stopPropagation();
    onEdit();
  }

  function handleDelete(e: React.MouseEvent) {
    e.stopPropagation();
    onDelete();
  }

  return (
    <div className="card-actions">
      <button className="action-btn" onClick={handleEdit}>
        Edit
      </button>
      <button className="action-btn delete" onClick={handleDelete}>
        Delete
      </button>
    </div>
  );
}
