import type { PersonaSnapshot } from '@hmcs/sdk';
import { Persona } from '@hmcs/sdk';
import { cn } from '@hmcs/ui';

interface SidebarProps {
  personas: PersonaSnapshot[];
  selectedId: string | null;
  onSelect: (id: string) => void;
  onCreateClick: () => void;
}

export default function Sidebar({ personas, selectedId, onSelect, onCreateClick }: SidebarProps) {
  return (
    <aside className="flex w-56 shrink-0 flex-col border-r border-primary/12 bg-input/40">
      <div className="no-scrollbar flex flex-1 flex-col overflow-y-auto">
        {personas.map((p) => (
          <SidebarItem
            key={p.id}
            persona={p}
            selected={p.id === selectedId}
            onSelect={() => onSelect(p.id)}
          />
        ))}
      </div>
      <button
        type="button"
        onClick={onCreateClick}
        className="cursor-pointer border-t border-primary/12 bg-primary/8 px-3 py-2.5 text-xs uppercase tracking-[0.08em] text-primary transition-colors duration-200 hover:bg-primary/15"
      >
        + Create
      </button>
    </aside>
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
      onClick={onSelect}
      className={cn(
        'flex w-full cursor-pointer items-center gap-2 px-3 py-2 text-left transition-colors duration-150',
        'text-muted-foreground hover:bg-primary/10 hover:text-foreground',
        selected && 'border-l-2 border-primary/60 bg-primary/15 text-foreground',
      )}
    >
      <div className="relative flex size-8 shrink-0 items-center justify-center overflow-hidden rounded-md bg-input">
        <img
          src={new Persona(persona.id).thumbnailUrl() || undefined}
          alt=""
          className="size-full object-cover"
          onError={(e) => {
            e.currentTarget.style.display = 'none';
            e.currentTarget.nextElementSibling?.classList.remove('hidden');
          }}
        />
        <span className="hidden text-sm font-semibold text-muted-foreground">{initial}</span>
      </div>
      <div className="flex min-w-0 flex-1 flex-col">
        <span className="truncate text-sm">{persona.name}</span>
        <span className="flex items-center gap-1 text-[0.7rem] tracking-[0.04em] text-hud-text-subdued">
          <span
            className={cn(
              'inline-block size-1.5 rounded-full',
              persona.spawned ? 'bg-success' : 'bg-muted-foreground/40',
            )}
          />
          {persona.spawned ? 'Spawned' : 'Offline'}
        </span>
      </div>
    </button>
  );
}
