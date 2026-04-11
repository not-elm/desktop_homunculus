import { rpc } from '@hmcs/sdk/rpc';
import {
  AlertDialog,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
} from '@hmcs/ui';
import { useState } from 'react';

interface WorktreeDetails {
  name: string;
  commits: number;
  filesChanged: number;
  insertions: number;
  deletions: number;
  hasUncommittedChanges: boolean;
  canMerge: boolean;
}

interface RemoveWorktreeDialogProps {
  workspacePath: string;
  worktree: WorktreeDetails;
  onRemoved: () => void;
  onCancel: () => void;
}

export function RemoveWorktreeDialog({
  workspacePath,
  worktree,
  onRemoved,
  onCancel,
}: RemoveWorktreeDialogProps) {
  const [error, setError] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);

  async function handleAction(action: 'merge' | 'remove') {
    setBusy(true);
    setError(null);
    try {
      await rpc.call({
        modName: '@hmcs/agent',
        method: 'remove-worktree',
        body: { workspacePath, name: worktree.name, action },
      });
      onRemoved();
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Operation failed');
    } finally {
      setBusy(false);
    }
  }

  const actionsDisabled = busy || worktree.hasUncommittedChanges;

  return (
    <AlertDialog
      open
      onOpenChange={(open) => {
        if (!open) onCancel();
      }}
    >
      <AlertDialogContent>
        <AlertDialogHeader>
          <AlertDialogTitle>Remove worktree &ldquo;{worktree.name}&rdquo;?</AlertDialogTitle>
          <AlertDialogDescription asChild>
            <div>
              <ChangeSummary worktree={worktree} />
              {worktree.hasUncommittedChanges && (
                <p className="mt-2 text-amber-400 text-xs">
                  Cannot remove: worktree has uncommitted changes. Commit or stash them first.
                </p>
              )}
              {error && <p className="mt-2 text-destructive text-xs">{error}</p>}
            </div>
          </AlertDialogDescription>
        </AlertDialogHeader>
        <AlertDialogFooter>
          <AlertDialogCancel onClick={onCancel}>Cancel</AlertDialogCancel>
          {worktree.canMerge && (
            <button
              className="inline-flex items-center justify-center rounded-md text-sm font-medium h-9 px-4 py-2 bg-primary/10 text-primary border border-primary/20 hover:bg-primary/20 disabled:opacity-40 disabled:cursor-not-allowed"
              type="button"
              disabled={actionsDisabled}
              onClick={() => handleAction('merge')}
            >
              Merge &amp; Remove
            </button>
          )}
          <button
            className="inline-flex items-center justify-center rounded-md text-sm font-medium h-9 px-4 py-2 bg-destructive text-destructive-foreground hover:bg-destructive/90 disabled:opacity-40 disabled:cursor-not-allowed"
            type="button"
            disabled={actionsDisabled}
            onClick={() => handleAction('remove')}
          >
            Remove
          </button>
        </AlertDialogFooter>
      </AlertDialogContent>
    </AlertDialog>
  );
}

function ChangeSummary({ worktree }: { worktree: WorktreeDetails }) {
  if (worktree.commits === 0 && worktree.filesChanged === 0) {
    return <span className="text-muted-foreground text-xs">No changes from base branch.</span>;
  }

  return (
    <span className="font-mono text-xs">
      {worktree.commits} commit{worktree.commits !== 1 ? 's' : ''} &middot; {worktree.filesChanged}{' '}
      file{worktree.filesChanged !== 1 ? 's' : ''} changed{' '}
      <span className="text-green-400">+{worktree.insertions}</span>
      {' / '}
      <span className="text-red-400">-{worktree.deletions}</span>
    </span>
  );
}
