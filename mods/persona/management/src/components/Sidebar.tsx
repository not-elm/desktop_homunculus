import type { PersonaSnapshot } from '@hmcs/sdk';
import { Persona } from '@hmcs/sdk';

interface SidebarProps {
  personas: PersonaSnapshot[];
  selectedId: string | null;
  onSelect: (id: string) => void;
  onCreateClick: () => void;
}

export default function Sidebar({ personas, selectedId, onSelect, onCreateClick }: SidebarProps) {
  return (
    <div className="sidebar">
      <div className="sidebar-list">
        {personas.map((p) => (
          <SidebarItem
            key={p.id}
            persona={p}
            selected={p.id === selectedId}
            onSelect={() => onSelect(p.id)}
          />
        ))}
      </div>
      <button type="button" className="sidebar-create" onClick={onCreateClick}>
        + Create
      </button>
    </div>
  );
}

function SidebarItem({
  persona,
  selected,
  onSelect,
}: {
  persona: PersonaSnapshot;
  selected: boolean;
  onSelect: () => void;
}) {
  const initial = persona.name?.charAt(0).toUpperCase() ?? '?';

  return (
    <button
      type="button"
      className={`sidebar-item ${selected ? 'sidebar-item--selected' : ''}`}
      onClick={onSelect}
    >
      <div className="sidebar-item-avatar">
        <img
          src={new Persona(persona.id).thumbnailUrl() || undefined}
          alt=""
          onError={(e) => {
            e.currentTarget.style.display = 'none';
            e.currentTarget.nextElementSibling?.classList.remove('hidden');
          }}
        />
        <span className="sidebar-item-initial hidden">{initial}</span>
      </div>
      <div className="sidebar-item-info">
        <span className="sidebar-item-name">{persona.name}</span>
        <span className="sidebar-item-status">
          <span className={`status-dot ${persona.spawned ? 'active' : 'inactive'}`} />
          {persona.spawned ? 'Spawned' : 'Offline'}
        </span>
      </div>
    </button>
  );
}
