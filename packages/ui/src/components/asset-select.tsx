import * as PopoverPrimitive from '@radix-ui/react-popover';
import { ChevronDownIcon } from 'lucide-react';
import * as React from 'react';

import { cn } from '@/lib/utils';

export interface AssetSelectItem {
  id: string;
  description?: string;
}

export interface AssetSelectGroup {
  label: string;
  items: AssetSelectItem[];
}

export interface AssetSelectProps {
  value: string | null;
  onValueChange: (value: string | null) => void;
  items: AssetSelectItem[] | AssetSelectGroup[];
  allowNone?: boolean;
  noneLabel?: string;
  disabled?: boolean;
  placeholder?: string;
  onBrowse?: () => void;
  browseLabel?: string;
  renderAction?: () => React.ReactNode;
  className?: string;
}

function isGrouped(
  items: AssetSelectItem[] | AssetSelectGroup[],
): items is AssetSelectGroup[] {
  return items.length > 0 && 'label' in items[0];
}

function ItemButton({
  id,
  description,
  selected,
  onSelect,
}: {
  id: string;
  description?: string;
  selected: boolean;
  onSelect: (id: string) => void;
}) {
  return (
    <button
      type="button"
      data-selected={selected}
      className={cn(
        'flex w-full cursor-default items-center rounded-md px-2 py-1.5 text-left text-sm font-mono tracking-wide outline-hidden select-none transition-colors',
        'data-[selected=true]:bg-accent hover:bg-accent/50',
      )}
      onClick={() => onSelect(id)}
    >
      <span className="flex flex-col">
        <span>{id}</span>
        {description && (
          <span className="block text-[10px] font-sans text-muted-foreground">
            {description}
          </span>
        )}
      </span>
    </button>
  );
}

function GroupSection({
  group,
  value,
  onSelect,
}: {
  group: AssetSelectGroup;
  value: string | null;
  onSelect: (id: string) => void;
}) {
  return (
    <div data-slot="asset-select-group">
      <div className="px-2 py-1.5 text-[9px] font-medium uppercase tracking-widest text-muted-foreground">
        {group.label}
      </div>
      {group.items.map((item) => (
        <ItemButton
          key={item.id}
          id={item.id}
          description={item.description}
          selected={value === item.id}
          onSelect={onSelect}
        />
      ))}
    </div>
  );
}

function FlatItems({
  items,
  value,
  onSelect,
}: {
  items: AssetSelectItem[];
  value: string | null;
  onSelect: (id: string) => void;
}) {
  return (
    <>
      {items.map((item) => (
        <ItemButton
          key={item.id}
          id={item.id}
          description={item.description}
          selected={value === item.id}
          onSelect={onSelect}
        />
      ))}
    </>
  );
}

export function AssetSelect({
  value,
  onValueChange,
  items,
  allowNone = false,
  noneLabel = 'None',
  disabled = false,
  placeholder = 'None',
  onBrowse,
  browseLabel = 'Browse...',
  renderAction,
  className,
}: AssetSelectProps) {
  const [open, setOpen] = React.useState(false);

  function handleSelect(id: string) {
    onValueChange(id);
    setOpen(false);
  }

  function handleSelectNone() {
    onValueChange(null);
    setOpen(false);
  }

  function handleBrowse() {
    setOpen(false);
    onBrowse?.();
  }

  const hasItems = items.length > 0;

  return (
    <div data-slot="asset-select" className={cn('w-full', className)}>
      <PopoverPrimitive.Root open={open} onOpenChange={setOpen}>
        <div className="flex items-center gap-2">
          <PopoverPrimitive.Trigger
            data-slot="asset-select-trigger"
            disabled={disabled}
            className={cn(
              'border-border bg-input flex w-full items-center justify-between gap-2 rounded-lg border px-3 py-2 text-sm backdrop-blur-md transition-all duration-200 outline-none disabled:cursor-not-allowed disabled:opacity-50',
              !value && 'text-muted-foreground/60',
            )}
          >
            <span className="truncate font-mono tracking-wide">
              {value ?? placeholder}
            </span>
            <ChevronDownIcon className="size-4 shrink-0 opacity-50" />
          </PopoverPrimitive.Trigger>
          {renderAction?.()}
        </div>
        <PopoverPrimitive.Portal>
          <PopoverPrimitive.Content
            data-slot="asset-select-content"
            align="start"
            sideOffset={4}
            className={cn(
              'bg-popover text-popover-foreground max-h-60 w-[var(--radix-popover-trigger-width)] overflow-y-auto rounded-lg border border-border p-1 shadow-holo-sm backdrop-blur-lg no-scrollbar',
              'data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 z-50 origin-(--radix-popover-content-transform-origin)',
            )}
          >
            {allowNone && (
              <ItemButton
                id={noneLabel}
                selected={value === null}
                onSelect={handleSelectNone}
              />
            )}
            {allowNone && hasItems && (
              <div className="holo-separator -mx-1 my-1 h-px" />
            )}
            {isGrouped(items) ? (
              items.map((group) => (
                <GroupSection
                  key={group.label}
                  group={group}
                  value={value}
                  onSelect={handleSelect}
                />
              ))
            ) : (
              <FlatItems items={items} value={value} onSelect={handleSelect} />
            )}
            {onBrowse && (
              <>
                <div className="holo-separator -mx-1 my-1 h-px" />
                <button
                  type="button"
                  className="w-full px-3 py-2 text-left text-xs tracking-wide text-primary/70 transition-colors hover:bg-accent/50"
                  onClick={handleBrowse}
                >
                  {browseLabel}
                </button>
              </>
            )}
          </PopoverPrimitive.Content>
        </PopoverPrimitive.Portal>
      </PopoverPrimitive.Root>
    </div>
  );
}
