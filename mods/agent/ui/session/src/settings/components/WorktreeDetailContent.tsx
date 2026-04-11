import { GitBranch } from 'lucide-react';
import type { WorktreeData } from './WorktreeNode.tsx';

interface WorktreeDetailContentProps {
  worktree: WorktreeData;
}

export function WorktreeDetailContent({ worktree }: WorktreeDetailContentProps) {
  return (
    <>
      <div className="ws-detail-title">
        <GitBranch className="ws-icon" />
        {worktree.name}
      </div>
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
          {' / '}
          <span className="ws-detail-minus">-{worktree.deletions}</span>
        </span>
      </div>
      <StatusSection worktree={worktree} />
    </>
  );
}

function StatusSection({ worktree }: { worktree: WorktreeData }) {
  return (
    <div className="ws-detail-separator">
      <div className="ws-detail-status">
        <span
          className={`ws-dot ${worktree.hasUncommittedChanges ? 'ws-dot--dirty' : 'ws-dot--clean'}`}
        />
        <span className="ws-detail-status-text">
          {worktree.hasUncommittedChanges
            ? 'Uncommitted changes'
            : 'Clean — no uncommitted changes'}
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
        <span className="ws-detail-merge-icon ws-detail-merge-icon--ok">✓</span>
        <span className="ws-detail-merge-icon--ok">Can fast-forward merge</span>
      </div>
    );
  }
  return (
    <div className="ws-detail-status">
      <span className="ws-detail-merge-icon ws-detail-merge-icon--no">✗</span>
      <span className="ws-detail-merge-icon--no">Cannot fast-forward merge</span>
    </div>
  );
}
