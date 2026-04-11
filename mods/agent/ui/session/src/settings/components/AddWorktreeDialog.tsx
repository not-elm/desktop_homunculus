import { rpc } from '@hmcs/sdk/rpc';
import {
  buttonVariants,
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@hmcs/ui';
import { useCallback, useEffect, useState } from 'react';
import type { BranchData } from './WorkspaceTree.tsx';

interface AddWorktreeDialogProps {
  workspacePath: string;
  onCreated: () => void;
  onCancel: () => void;
}

export function AddWorktreeDialog({ workspacePath, onCreated, onCancel }: AddWorktreeDialogProps) {
  const [branches, setBranches] = useState<string[]>([]);
  const [currentBranch, setCurrentBranch] = useState<string | null>(null);
  const [selectedBranch, setSelectedBranch] = useState('');
  const [name, setName] = useState('');
  const [error, setError] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);

  const loadBranches = useCallback(async () => {
    try {
      const data = await rpc.call<BranchData>({
        modName: '@hmcs/agent',
        method: 'list-branches',
        body: { workspacePath },
      });
      setBranches(data.branches);
      setCurrentBranch(data.current);
      setSelectedBranch(data.current ?? data.branches[0] ?? '');
    } catch {
      setBranches([]);
    }
  }, [workspacePath]);

  useEffect(() => {
    loadBranches();
  }, [loadBranches]);

  async function handleCreate() {
    const trimmedName = name.trim();
    if (!trimmedName || !selectedBranch) return;
    setBusy(true);
    setError(null);
    try {
      await rpc.call({
        modName: '@hmcs/agent',
        method: 'add-worktree',
        body: { workspacePath, name: trimmedName, branch: selectedBranch },
      });
      onCreated();
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setBusy(false);
    }
  }

  function handleKeyDown(e: React.KeyboardEvent) {
    if (e.key === 'Enter') handleCreate();
  }

  return (
    <Dialog
      open
      onOpenChange={(open) => {
        if (!open) onCancel();
      }}
    >
      <DialogContent showCloseButton={false}>
        <DialogHeader>
          <DialogTitle>Add Worktree</DialogTitle>
          <DialogDescription asChild>
            <div className="space-y-3">
              <p>Create a new worktree from an existing branch.</p>
              <div className="space-y-1">
                <label className="text-xs font-medium text-muted-foreground">Name</label>
                <input
                  className="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm font-mono shadow-sm placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring"
                  type="text"
                  placeholder="feature-name"
                  value={name}
                  onChange={(e) => setName(e.target.value)}
                  onKeyDown={handleKeyDown}
                  autoFocus
                />
              </div>
              <div className="space-y-1">
                <label className="text-xs font-medium text-muted-foreground">Branch</label>
                <Select value={selectedBranch} onValueChange={setSelectedBranch}>
                  <SelectTrigger className="font-mono text-xs">
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent>
                    {branches.map((b) => (
                      <SelectItem key={b} value={b}>
                        {b}
                        {b === currentBranch ? ' (current)' : ''}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </div>
              {error && <p className="text-destructive text-xs">{error}</p>}
            </div>
          </DialogDescription>
        </DialogHeader>
        <DialogFooter>
          <DialogClose className={buttonVariants({ variant: 'outline' })} onClick={onCancel}>
            Cancel
          </DialogClose>
          <button
            className="inline-flex items-center justify-center rounded-md text-sm font-medium h-9 px-4 py-2 bg-primary/10 text-primary border border-primary/20 hover:bg-primary/20 disabled:opacity-40 disabled:cursor-not-allowed"
            type="button"
            disabled={busy || !name.trim() || !selectedBranch}
            onClick={handleCreate}
          >
            Create Worktree
          </button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
