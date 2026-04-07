const defaultMenuItems = [
  { id: "1", modName: "Elmer", text: "Greet", command: "greet" },
  { id: "2", modName: "Elmer", text: "Dance", command: "dance" },
  { id: "3", modName: "VoiceVox", text: "Speak", command: "speak" },
];

let _menuItems = [...defaultMenuItems];

/** Override menu items returned by mods.menus() for the current story. */
export function __setMockMenuItems(items: typeof defaultMenuItems) {
  _menuItems = items;
}

/** Reset all mocks to defaults. Call in meta.beforeEach. */
export function __resetMocks() {
  _menuItems = [...defaultMenuItems];
}

export const Webview = {
  current: () => ({
    linkedPersona: async () => ({
      id: "elmer",
      name: async () => "Elmer",
    }),
    close: async () => {},
  }),
  open: async () => {},
};

export const mods = {
  menus: async () => _menuItems,
  executeCommand: async () => {},
};
