import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@hmcs/ui";

interface OverflowItem {
  label: string;
  onClick: () => void;
  destructive?: boolean;
}

interface TreeOverflowMenuProps {
  items: OverflowItem[];
}

export function TreeOverflowMenu({ items }: TreeOverflowMenuProps) {
  if (items.length === 0) return null;

  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <button
          className="ws-overflow"
          type="button"
          onClick={(e) => e.stopPropagation()}
          aria-label="More actions"
        >
          ...
        </button>
      </DropdownMenuTrigger>
      <DropdownMenuContent align="end">
        {items.map((item) => (
          <DropdownMenuItem
            key={item.label}
            variant={item.destructive ? "destructive" : "default"}
            onClick={(e) => {
              e.stopPropagation();
              item.onClick();
            }}
          >
            {item.label}
          </DropdownMenuItem>
        ))}
      </DropdownMenuContent>
    </DropdownMenu>
  );
}
