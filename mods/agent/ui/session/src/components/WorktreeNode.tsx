import { GitBranch } from "lucide-react";
import { TreeOverflowMenu } from "./TreeOverflowMenu.tsx";

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
  const rowClass = `ws-row ws-row--worktree${isSelected ? " ws-row--selected" : ""}`;

  return (
    <div
      className={rowClass}
      role="treeitem"
      aria-level={2}
      aria-selected={isSelected}
      tabIndex={tabIndex}
      data-wt-name={worktree.name}
      onClick={onSelect}
      onKeyDown={onKeyDown}
    >
      <div className="ws-row-content">
        <GitBranch className="ws-icon" />
        <span className="ws-name">{worktree.name}</span>
        <StatusBadge worktree={worktree} />
        {worktree.canMerge && (
          <span className="ws-badge ws-badge--mergeable">mergeable</span>
        )}
        <TreeOverflowMenu items={[
          { label: "Remove Worktree", onClick: onRemove, destructive: true },
        ]} />
      </div>
      {isSelected && <MetaTier2 worktree={worktree} />}
    </div>
  );
}

function StatusBadge({ worktree }: { worktree: WorktreeData }) {
  return (
    <span className="ws-badge ws-badge--status">
      <span className={`ws-dot ${worktree.hasUncommittedChanges ? "ws-dot--dirty" : "ws-dot--clean"}`} />
      {worktree.branch}
    </span>
  );
}

function MetaTier2({ worktree }: { worktree: WorktreeData }) {
  return (
    <div className="ws-meta-t2">
      <span className="ws-meta-label">base</span>
      <span className="ws-meta-value">{worktree.baseBranch}</span>
      <span className="ws-meta-label">commits</span>
      <span className="ws-meta-value">{worktree.commits}</span>
      <span className="ws-meta-label">files</span>
      <span className="ws-meta-value">{worktree.filesChanged}</span>
      <span className="ws-meta-label">diff</span>
      <span className="ws-meta-value">
        <span className="ws-meta-plus">+{worktree.insertions}</span>
        {" / "}
        <span className="ws-meta-minus">-{worktree.deletions}</span>
      </span>
    </div>
  );
}
