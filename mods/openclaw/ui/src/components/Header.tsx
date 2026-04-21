export function Header({ personaName }: { personaName: string | null }) {
  return (
    <div className="settings-header">
      <h1 className="settings-title openclaw-title">Openclaw</h1>
      <span className="settings-entity-name">{personaName ?? ''}</span>
    </div>
  );
}
