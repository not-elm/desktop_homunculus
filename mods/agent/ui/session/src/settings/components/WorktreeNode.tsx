import { Popover, PopoverAnchor } from '@hmcs/ui';
import { GitBranch } from 'lucide-react';
import { useRef, useState } from 'react';
import { TreeOverflowMenu } from './TreeOverflowMenu.tsx';
import { WorktreeDetailPopover } from './WorktreeDetailPopover.tsx';

export interface WorktreeData {
  name: string;
  branch: string;
  baseBranch: string;
  commits: number;
  filesChanged: number;
  insertions: number;
  deletions: number;
  hasUncommittedChanges: boolean;
  canMerge: boolean;
}

interface WorktreeNodeProps {
  worktree: WorktreeData;
  isSelected: boolean;
  tabIndex: number;
  onSelect: () => void;
  onRemove: () => void;
  onKeyDown: (e: React.KeyboardEvent) => void;
}

export function WorktreeNode({
  worktree,
  isSelected,
  tabIndex,
  onSelect,
  onRemove,
  onKeyDown,
}: WorktreeNodeProps) {
  const [showDetail, setShowDetail] = useState(false);
  const anchorRef = useRef<HTMLDivElement>(null);

  const overflowItems = buildOverflowItems(() => setShowDetail(true), onRemove);

  return (
    <Popover open={showDetail} onOpenChange={setShowDetail}>
      <PopoverAnchor asChild>
        <div
          ref={anchorRef}
          className="ws-row ws-row--worktree"
          role="treeitem"
          aria-level={2}
          aria-selected={isSelected}
          tabIndex={tabIndex}
          data-wt-name={worktree.name}
          onClick={onSelect}
          onKeyDown={onKeyDown}
        >
          <span className="ws-radio" aria-hidden="true" aria-checked={isSelected}>
            {isSelected && <span className="ws-radio-dot" />}
          </span>
          <GitBranch className="ws-icon" />
          <span className="ws-name">{worktree.branch}</span>
          <TreeOverflowMenu items={overflowItems} />
        </div>
      </PopoverAnchor>
      <WorktreeDetailPopover worktree={worktree} anchorRef={anchorRef} />
    </Popover>
  );
}

function buildOverflowItems(onShowDetails: () => void, onRemove: () => void) {
  return [
    { label: 'Details', onClick: onShowDetails },
    { label: 'Remove Worktree', onClick: onRemove, destructive: true },
  ];
}
