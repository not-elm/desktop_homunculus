export type SettingsCategory = 'phrases' | 'permissions' | 'backend';

export type BodyContent =
  | { kind: 'sessionLog' }
  | { kind: 'settingsForm'; category: SettingsCategory }
  | { kind: 'empty' }
  | { kind: 'sessionHistory' }
  | { kind: 'pastSession'; uuid: string };
