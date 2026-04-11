import { PopoverContent } from '@hmcs/ui';
import { WorktreeDetailContent } from './WorktreeDetailContent.tsx';
import type { WorktreeData } from './WorktreeNode.tsx';

interface WorktreeDetailPopoverProps {
  worktree: WorktreeData;
  anchorRef: React.RefObject<HTMLDivElement | null>;
}

export function WorktreeDetailPopover({ worktree, anchorRef }: WorktreeDetailPopoverProps) {
  return (
    <PopoverContent
      align="end"
      sideOffset={8}
      className="w-auto min-w-[220px]"
      onFocusOutside={(e) => e.preventDefault()}
      onInteractOutside={(e) => {
        // Block dismiss when pointer lands inside the anchor row.
        // PopoverContentNonModal has a built-in triggerRef guard for this,
        // but triggerRef is null when using PopoverAnchor instead of PopoverTrigger.
        if (anchorRef.current?.contains(e.target as Node)) {
          e.preventDefault();
        }
      }}
    >
      <WorktreeDetailContent worktree={worktree} />
    </PopoverContent>
  );
}
