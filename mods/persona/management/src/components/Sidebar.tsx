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
    <div className="flex w-[190px] flex-shrink-0 flex-col border-r border-[oklch(0.72_0.14_192/0.08)] bg-[oklch(0.72_0.14_192/0.02)] px-2 py-2.5">
      <div className="no-scrollbar flex-1 overflow-y-auto">
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
        className="mt-2 flex-shrink-0 cursor-pointer rounded-md border border-dashed border-[oklch(0.72_0.14_192/0.15)] bg-transparent px-2 py-1.5 text-center font-[inherit] text-[10px] text-[oklch(0.72_0.14_192/0.35)] transition-all duration-200 ease-linear hover:border-[oklch(0.72_0.14_192/0.3)] hover:bg-[oklch(0.72_0.14_192/0.04)] hover:text-[oklch(0.72_0.14_192/0.7)]"
        onClick={onCreateClick}
      >
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
      className={cn(
        'mb-1 flex w-full cursor-pointer items-center gap-2 rounded-md border border-transparent bg-transparent px-2 py-[7px] text-left font-[inherit] transition-all duration-150 ease-linear hover:bg-[oklch(0.72_0.14_192/0.04)]',
        selected && 'border-[oklch(0.72_0.14_192/0.2)] bg-[oklch(0.72_0.14_192/0.1)]',
      )}
      onClick={onSelect}
    >
      <div
        className={cn(
          'flex h-[26px] w-[26px] flex-shrink-0 items-center justify-center overflow-hidden rounded-full bg-[oklch(0.72_0.14_192/0.08)]',
          selected && 'bg-[oklch(0.72_0.14_192/0.12)]',
        )}
      >
        <img
          className="h-full w-full object-cover"
          src={new Persona(persona.id).thumbnailUrl() || undefined}
          alt=""
          onError={(e) => {
            e.currentTarget.style.display = 'none';
            e.currentTarget.nextElementSibling?.classList.remove('hidden');
          }}
        />
        <span className="hidden text-[10px] font-medium text-[oklch(0.72_0.14_192/0.4)]">
          {initial}
        </span>
      </div>
      <div className="flex min-w-0 flex-col">
        <span
          className={cn(
            'overflow-hidden truncate text-[11px] text-[oklch(0.72_0.14_192/0.5)]',
            selected && 'font-semibold text-[oklch(0.72_0.14_192/0.95)]',
          )}
        >
          {persona.name}
        </span>
        <span className="mt-px flex items-center gap-[3px] text-[8px] text-[oklch(0.55_0.02_250/0.5)]">
          <span
            className={cn(
              'inline-block h-[5px] w-[5px] flex-shrink-0 rounded-full',
              persona.spawned
                ? 'bg-[oklch(0.68_0.17_155)] [box-shadow:0_0_4px_oklch(0.68_0.17_155/0.4)] animate-pulse-dot motion-reduce:animate-none'
                : 'bg-[oklch(0.4_0.02_250)]',
            )}
          />
          {persona.spawned ? 'Spawned' : 'Offline'}
        </span>
      </div>
    </button>
  );
}
