import { ChevronRight, Folder, FolderGit2 } from 'lucide-react';
import { useState } from 'react';
import { TreeOverflowMenu } from './TreeOverflowMenu.tsx';
import { type WorktreeData, WorktreeNode } from './WorktreeNode.tsx';

export interface WorkspaceData {
  isGit: boolean;
  currentBranch: string | null;
  worktrees: WorktreeData[];
}

interface WorkspaceNodeProps {
  index: number;
  path: string;
  data: WorkspaceData | undefined;
  isSelected: boolean;
  selectedWorktree: string | null;
  tabIndex: number;
  onSelectWorkspace: () => void;
  onSelectWorktree: (name: string) => void;
  onRemoveWorkspace: () => void;
  onAddWorktree: () => void;
  onRemoveWorktree: (wt: WorktreeData) => void;
  onKeyDown: (e: React.KeyboardEvent) => void;
}

export function WorkspaceNode({
  index,
  path,
  data,
  isSelected,
  selectedWorktree,
  tabIndex,
  onSelectWorkspace,
  onSelectWorktree,
  onRemoveWorkspace,
  onAddWorktree,
  onRemoveWorktree,
  onKeyDown,
}: WorkspaceNodeProps) {
  const [expanded, setExpanded] = useState(true);
  const hasWorktrees = (data?.worktrees.length ?? 0) > 0;
  const hasChildren = hasWorktrees;
  const FolderIcon = data?.isGit ? FolderGit2 : Folder;
  const dirName = path.split(/[/\\]/).pop() || path;

  function handleRowClick() {
    onSelectWorkspace();
  }

  function handleChevronClick(e: React.MouseEvent) {
    e.stopPropagation();
    setExpanded(!expanded);
  }

  const overflowItems = buildOverflowItems(data?.isGit ?? false, onAddWorktree, onRemoveWorkspace);
  const collapsedHint = buildCollapsedHint(expanded, hasWorktrees, selectedWorktree, data);
  const rowClass = 'ws-row ws-row--workspace';

  return (
    <div
      role="treeitem"
      aria-level={1}
      aria-expanded={hasChildren ? expanded : undefined}
      aria-selected={isSelected}
      tabIndex={tabIndex}
      data-ws-index={index}
      onKeyDown={onKeyDown}
    >
      {/* biome-ignore lint/a11y/noStaticElementInteractions: click delegates to parent treeitem */}
      <div className={rowClass} onClick={handleRowClick} onKeyDown={onKeyDown}>
        <span className="ws-radio" aria-hidden="true">
          {isSelected && <span className="ws-radio-dot" />}
        </span>
        {hasChildren ? (
          <ChevronRight
            className={`ws-chevron${expanded ? ' ws-chevron--expanded' : ''}`}
            onClick={handleChevronClick}
          />
        ) : (
          <span style={{ width: 12, flexShrink: 0 }} />
        )}
        <FolderIcon className="ws-icon" />
        <span className="ws-name" title={path}>
          {dirName}
        </span>
        {collapsedHint && <span className="ws-collapsed-hint">{collapsedHint}</span>}
        {!expanded && hasWorktrees && (
          <span className="ws-badge ws-badge--branch">{data?.worktrees.length}</span>
        )}
        <TreeOverflowMenu items={overflowItems} />
      </div>

      {hasChildren && (
        <div className={`ws-children-wrapper${expanded ? '' : ' ws-children-wrapper--collapsed'}`}>
          <div className="ws-children-inner">
            {/* biome-ignore lint/a11y/useSemanticElements: group role is correct for tree children, fieldset is not appropriate */}
            <div className="ws-connector" role="group" aria-label="Worktrees">
              {data?.worktrees.map((wt) => (
                <WorktreeNode
                  key={wt.name}
                  worktree={wt}
                  isSelected={selectedWorktree === wt.name}
                  tabIndex={-1}
                  onSelect={() => onSelectWorktree(wt.name)}
                  onRemove={() => onRemoveWorktree(wt)}
                  onKeyDown={onKeyDown}
                />
              ))}
            </div>
          </div>
        </div>
      )}
    </div>
  );
}

function buildOverflowItems(
  isGit: boolean,
  onAddWorktree: () => void,
  onRemoveWorkspace: () => void,
) {
  const items: { label: string; onClick: () => void; destructive?: boolean }[] = [];
  if (isGit) items.push({ label: 'Add Worktree', onClick: onAddWorktree });
  items.push({ label: 'Remove Workspace', onClick: onRemoveWorkspace, destructive: true });
  return items;
}

function buildCollapsedHint(
  expanded: boolean,
  hasWorktrees: boolean,
  selectedWorktree: string | null,
  data: WorkspaceData | undefined,
): string | null {
  if (expanded || !hasWorktrees || !selectedWorktree || !data) return null;
  const active = data.worktrees.find((wt) => wt.name === selectedWorktree);
  return active ? active.branch : null;
}
