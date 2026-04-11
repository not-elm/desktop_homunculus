import { rpc } from '@hmcs/sdk/rpc';
import { useEffect, useState } from 'react';

/** Resolve the current git branch for a workspace via the get-current-branch RPC. */
export function useCurrentBranch(
  workspacePath: string | null,
  worktreeName: string | null,
): string | null {
  const [branch, setBranch] = useState<string | null>(null);

  useEffect(() => {
    if (!workspacePath) return;
    let cancelled = false;
    (async () => {
      try {
        const result = (await rpc.call({
          modName: '@hmcs/agent',
          method: 'get-current-branch',
          body: { workspacePath, worktreeName },
        })) as { branchName?: string };
        if (!cancelled && result.branchName) {
          setBranch(result.branchName);
        }
      } catch {
        if (!cancelled) setBranch(null);
      }
    })();
    return () => {
      cancelled = true;
    };
  }, [workspacePath, worktreeName]);

  return branch;
}
