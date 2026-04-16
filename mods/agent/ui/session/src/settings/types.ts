export type SettingsCategory = 'phrases' | 'permissions' | 'services';

export type BodyContent =
  | { kind: 'sessionLog' }
  | { kind: 'settingsForm'; category: SettingsCategory }
  | { kind: 'empty' }
  | { kind: 'sessionHistory' }
  | { kind: 'pastSession'; uuid: string };
