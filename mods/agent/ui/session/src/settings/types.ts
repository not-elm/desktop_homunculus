export type SettingsCategory = "phrases" | "permissions" | "executor";

export type BodyContent =
  | { kind: "sessionLog" }
  | { kind: "settingsForm"; category: SettingsCategory }
  | { kind: "empty" };
