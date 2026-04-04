import { PopoverContent } from "@hmcs/ui";
import type { WorktreeData } from "./WorktreeNode.tsx";

interface WorktreeDetailPopoverProps {
  worktree: WorktreeData;
}

export function WorktreeDetailPopover({ worktree }: WorktreeDetailPopoverProps) {
  return (
    <PopoverContent align="end" sideOffset={8} className="w-auto min-w-[220px]">
      <div className="ws-detail-popover">
        <span className="ws-detail-label">Branch</span>
        <span className="ws-detail-value">{worktree.branch}</span>
        <span className="ws-detail-label">Base</span>
        <span className="ws-detail-value">{worktree.baseBranch}</span>
        <span className="ws-detail-label">Commits</span>
        <span className="ws-detail-value">{worktree.commits}</span>
        <span className="ws-detail-label">Files changed</span>
        <span className="ws-detail-value">{worktree.filesChanged}</span>
        <span className="ws-detail-label">Diff</span>
        <span className="ws-detail-value">
          <span className="ws-detail-plus">+{worktree.insertions}</span>
          {" / "}
          <span className="ws-detail-minus">-{worktree.deletions}</span>
        </span>
      </div>
      <StatusSection worktree={worktree} />
    </PopoverContent>
  );
}

function StatusSection({ worktree }: { worktree: WorktreeData }) {
  return (
    <div style={{ marginTop: 8, borderTop: "1px solid oklch(1 0 0 / 0.08)", paddingTop: 8 }}>
      <div className="ws-detail-status">
        <span
          className={`ws-dot ${worktree.hasUncommittedChanges ? "ws-dot--dirty" : "ws-dot--clean"}`}
        />
        <span style={{ color: "oklch(0.70 0 0)" }}>
          {worktree.hasUncommittedChanges ? "Uncommitted changes" : "Clean — no uncommitted changes"}
        </span>
      </div>
      <MergeStatus canMerge={worktree.canMerge} />
    </div>
  );
}

function MergeStatus({ canMerge }: { canMerge: boolean }) {
  if (canMerge) {
    return (
      <div className="ws-detail-status">
        <span style={{ fontSize: 10, color: "oklch(0.60 0.14 155)" }}>✓</span>
        <span style={{ color: "oklch(0.60 0.14 155)" }}>Can fast-forward merge</span>
      </div>
    );
  }
  return (
    <div className="ws-detail-status">
      <span style={{ fontSize: 10, color: "oklch(0.50 0 0)" }}>✗</span>
      <span style={{ color: "oklch(0.50 0 0)" }}>Cannot fast-forward merge</span>
    </div>
  );
}
