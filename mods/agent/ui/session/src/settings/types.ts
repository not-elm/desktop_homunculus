export type SettingsCategory = "phrases" | "permissions" | "api-model";

export type MainPanelContent =
  | { kind: "worktreeDetail"; workspaceIndex: number; worktreeName: string }
  | { kind: "workspaceOverview"; workspaceIndex: number }
  | { kind: "settingsForm"; category: SettingsCategory }
  | { kind: "empty" };
