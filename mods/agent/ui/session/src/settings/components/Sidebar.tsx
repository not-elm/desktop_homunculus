import type { WorkspaceSelection } from '../hooks/useSettingsDraft';
import type { SettingsCategory } from '../types';
import { SettingsNav } from './SettingsNav';
import { WorkspaceTree } from './WorkspaceTree';

interface SidebarProps {
  paths: string[];
  selection: WorkspaceSelection;
  onSelectionChange: (selection: WorkspaceSelection) => void;
  onAddWorkspace: (path: string) => void;
  onRemoveWorkspace: (index: number) => void;
  activeCategory: SettingsCategory | null;
  onCategorySelect: (category: SettingsCategory) => void;
  refreshKey?: number;
}

export function Sidebar({
  paths,
  selection,
  onSelectionChange,
  onAddWorkspace,
  onRemoveWorkspace,
  activeCategory,
  onCategorySelect,
  refreshKey,
}: SidebarProps) {
  return (
    <aside className="stg-sidebar h-full">
      <div className="stg-sidebar-tree">
        <WorkspaceTree
          paths={paths}
          selection={selection}
          onSelectionChange={onSelectionChange}
          onAddWorkspace={onAddWorkspace}
          onRemoveWorkspace={onRemoveWorkspace}
          refreshKey={refreshKey}
        />
      </div>
      <div className="stg-sidebar-divider" />
      <div className="stg-sidebar-nav">
        <SettingsNav activeCategory={activeCategory} onSelect={onCategorySelect} />
      </div>
    </aside>
  );
}
