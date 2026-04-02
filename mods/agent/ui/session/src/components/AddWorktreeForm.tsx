import { useCallback, useEffect, useState } from "react";
import { rpc } from "@hmcs/sdk/rpc";
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
    <div className="agent-worktree-form">
      <input
        className="settings-input agent-worktree-form-input"
        type="text"
        placeholder="Worktree name..."
        value={name}
        onChange={(e) => setName(e.target.value)}
        onKeyDown={handleKeyDown}
        autoFocus
      />
      <select
        className="settings-input agent-worktree-form-select"
        value={selectedBranch}
        onChange={(e) => setSelectedBranch(e.target.value)}
      >
        {branches.map((b) => (
          <option key={b} value={b}>
            {b}
          </option>
        ))}
      </select>
      {error && <p className="agent-dialog-error">{error}</p>}
      <div className="agent-worktree-form-actions">
        <button className="settings-close" type="button" onClick={onCancel}>
          Cancel
        </button>
        <button
          className="settings-save"
          type="button"
          disabled={busy || !name.trim() || !selectedBranch}
          onClick={handleCreate}
        >
          Create
        </button>
      </div>
    </div>
  );
}
