import { useCallback, useEffect, useState } from "react";
import { rpc } from "@hmcs/sdk/rpc";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@hmcs/ui";
import type { BranchData } from "./WorkspaceTree.tsx";

interface AddWorktreeFormProps {
  workspacePath: string;
  onCreated: () => void;
  onCancel: () => void;
}

export function AddWorktreeForm({
  workspacePath,
  onCreated,
  onCancel,
}: AddWorktreeFormProps) {
  const [branches, setBranches] = useState<string[]>([]);
  const [currentBranch, setCurrentBranch] = useState<string | null>(null);
  const [selectedBranch, setSelectedBranch] = useState("");
  const [name, setName] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);

  const loadBranches = useCallback(async () => {
    try {
      const data = await rpc.call<BranchData>({
        modName: "@hmcs/agent",
        method: "list-branches",
        body: { workspacePath },
      });
      setBranches(data.branches);
      setCurrentBranch(data.current);
      setSelectedBranch(data.current ?? data.branches[0] ?? "");
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
        modName: "@hmcs/agent",
        method: "add-worktree",
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
    if (e.key === "Enter") handleCreate();
  }

  return (
    <div className="awt-form">
      <div className="awt-row">
        <div className="awt-label">Name</div>
        <input
          className="awt-input"
          type="text"
          placeholder="feature-name"
          value={name}
          onChange={(e) => setName(e.target.value)}
          onKeyDown={handleKeyDown}
          autoFocus
        />
      </div>
      <div className="awt-row">
        <div className="awt-label">Branch</div>
        <Select value={selectedBranch} onValueChange={setSelectedBranch}>
          <SelectTrigger className="awt-select-trigger">
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            {branches.map((b) => (
              <SelectItem key={b} value={b}>
                {b}{b === currentBranch ? " (current)" : ""}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
      </div>
      {error && <p className="awt-error">{error}</p>}
      <div className="awt-actions">
        <button className="awt-btn awt-btn-cancel" type="button" onClick={onCancel}>
          Cancel
        </button>
        <button
          className="awt-btn awt-btn-create"
          type="button"
          disabled={busy || !name.trim() || !selectedBranch}
          onClick={handleCreate}
        >
          Create Worktree
        </button>
      </div>
    </div>
  );
}
