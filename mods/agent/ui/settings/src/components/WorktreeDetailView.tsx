import { GitBranch } from "lucide-react";
import type { WorktreeData } from "../hooks/useWorktreeDetail";

interface WorktreeDetailViewProps {
  worktree: WorktreeData;
}

export function WorktreeDetailView({ worktree }: WorktreeDetailViewProps) {
  return (
    <div className="stg-detail">
      <DetailHeader worktree={worktree} />
      <StatusBar worktree={worktree} />
      <StatsGrid worktree={worktree} />
      <BranchInfo worktree={worktree} />
    </div>
  );
}

function DetailHeader({ worktree }: { worktree: WorktreeData }) {
  return (
    <div className="stg-detail-header">
      <GitBranch className="stg-detail-icon" />
      <h2 className="stg-detail-title">{worktree.name}</h2>
      <span className="stg-detail-branch-tag">{worktree.branch}</span>
    </div>
  );
}

function StatusBar({ worktree }: { worktree: WorktreeData }) {
  return (
    <div className="stg-status-bar">
      <div className="stg-status-item">
        <span className={`stg-status-dot ${worktree.hasUncommittedChanges ? "stg-status-dot--dirty" : "stg-status-dot--clean"}`} />
        <span className="sr-only">{worktree.hasUncommittedChanges ? "uncommitted" : "clean"}</span>
        <span>{worktree.hasUncommittedChanges ? "Uncommitted changes" : "Clean"}</span>
      </div>
      <span className="stg-status-sep">|</span>
      <div className="stg-status-item">
        {worktree.canMerge
          ? <span className="stg-merge-ok">&#10003; Can fast-forward merge</span>
          : <span className="stg-merge-no">&#10007; Cannot fast-forward merge</span>}
      </div>
    </div>
  );
}

function StatsGrid({ worktree }: { worktree: WorktreeData }) {
  return (
    <dl className="stg-stats-grid">
      <StatCard label="Commits" value={String(worktree.commits)} />
      <StatCard label="Files" value={String(worktree.filesChanged)} />
      <StatCard label="Insertions" value={`+${worktree.insertions}`} variant="plus" />
      <StatCard label="Deletions" value={`-${worktree.deletions}`} variant="minus" />
    </dl>
  );
}

function StatCard({ label, value, variant }: { label: string; value: string; variant?: "plus" | "minus" }) {
  return (
    <div className="stg-stat-card">
      <dt className="stg-stat-label">{label}</dt>
      <dd className={`stg-stat-value${variant ? ` stg-stat-value--${variant}` : ""}`}>{value}</dd>
    </div>
  );
}

function BranchInfo({ worktree }: { worktree: WorktreeData }) {
  return (
    <div className="stg-detail-section">
      <h3 className="stg-section-header">Branch Info</h3>
      <dl className="stg-detail-dl">
        <dt className="stg-dl-label">Branch</dt>
        <dd className="stg-dl-value">{worktree.branch}</dd>
        <dt className="stg-dl-label">Base</dt>
        <dd className="stg-dl-value">{worktree.baseBranch}</dd>
      </dl>
    </div>
  );
}
