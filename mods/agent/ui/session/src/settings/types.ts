export type SettingsCategory = "phrases" | "permissions";

export type BodyContent =
  | { kind: "sessionLog" }
  | { kind: "settingsForm"; category: SettingsCategory }
  | { kind: "empty" };
