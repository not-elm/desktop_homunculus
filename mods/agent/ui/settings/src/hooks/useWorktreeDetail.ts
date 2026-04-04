import { useCallback, useEffect, useState } from "react";
import { rpc } from "@hmcs/sdk/rpc";

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

export function useWorktreeDetail(
  workspacePath: string | null,
  worktreeName: string | null,
) {
  const [data, setData] = useState<WorktreeData | null>(null);
  const [loading, setLoading] = useState(false);

  const fetchDetail = useCallback(async () => {
    if (!workspacePath || !worktreeName) {
      setData(null);
      return;
    }
    setLoading(true);
    try {
      const result = await rpc.call<{ worktrees: WorktreeData[] }>({
        modName: "@hmcs/agent",
        method: "list-worktrees",
        body: { workspacePath },
      });
      const found = result.worktrees.find((wt) => wt.name === worktreeName) ?? null;
      setData(found);
    } catch {
      setData(null);
    } finally {
      setLoading(false);
    }
  }, [workspacePath, worktreeName]);

  useEffect(() => {
    fetchDetail();
  }, [fetchDetail]);

  return { data, loading, refresh: fetchDetail };
}
