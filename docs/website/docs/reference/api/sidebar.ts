import type { SidebarsConfig } from "@docusaurus/plugin-content-docs";

const sidebar: SidebarsConfig = {
  apisidebar: [
    {
      type: "doc",
      id: "reference/api/homunculus-api",
    },
    {
      type: "category",
      label: "vrm",
      items: [
        {
          type: "doc",
          id: "reference/api/list-vrms",
          label: "List VRM entity IDs",
          className: "api-method get",
        },
        {
          type: "doc",
          id: "reference/api/spawn-vrm",
          label: "Spawn a VRM model",
          className: "api-method post",
        },
        {
          type: "doc",
          id: "reference/api/snapshot-vrms",
          label: "Get detailed snapshot of all VRM instances",
          className: "api-method get",
        },
        {
          type: "doc",
          id: "reference/api/wait-for-vrm-load",
          label: "Wait for a VRM model to load",
          className: "api-method get",
        },
        {
          type: "doc",
          id: "reference/api/stream-vrm-events",
          label: "Stream VRM load events",
          className: "api-method get",
        },
        {
          type: "doc",
          id: "reference/api/despawn-vrm",
          label: "Despawn a VRM model",
          className: "api-method delete",
        },
        {
          type: "doc",
          id: "reference/api/get-vrm-state",
          label: "Get the state of a VRM model",
          className: "api-method get",
        },
        {
          type: "doc",
          id: "reference/api/set-vrm-state",
          label: "Set the state of a VRM model",
          className: "api-method put",
        },
        {
          type: "doc",
          id: "reference/api/subscribe-to-vrm-events",
          label: "Subscribe to VRM events",
          className: "api-method get",
        },
        {
          type: "doc",
          id: "reference/api/set-vrm-look-target",
          label: "Set VRM look-at target",
          className: "api-method put",
        },
        {
          type: "doc",
          id: "reference/api/set-vrm-look-cursor",
          label: "Set VRM to look at cursor",
          className: "api-method put",
        },
        {
          type: "doc",
          id: "reference/api/disable-vrm-look",
          label: "Disable VRM look-at functionality",
          className: "api-method delete",
        },
        {
          type: "doc",
          id: "reference/api/speak-timeline",
          label: "Speak with timeline",
          className: "api-method post",
        },
        {
          type: "doc",
          id: "reference/api/get-vrm-bone",
          label: "Get a VRM bone entity",
          className: "api-method get",
        },
        {
          type: "doc",
          id: "reference/api/list-vrm-expressions",
          label: "List VRM expressions",
          className: "api-method get",
        },
        {
          type: "doc",
          id: "reference/api/set-vrm-expressions",
          label: "Set VRM expressions",
          className: "api-method put",
        },
        {
          type: "doc",
          id: "reference/api/modify-vrm-expressions",
          label: "Modify VRM expressions",
          className: "api-method patch",
        },
        {
          type: "doc",
          id: "reference/api/clear-vrm-expressions",
          label: "Clear all VRM expressions",
          className: "api-method delete",
        },
        {
          type: "doc",
          id: "reference/api/modify-vrm-mouth-expressions",
          label: "Modify VRM mouth expressions",
          className: "api-method patch",
        },
        {
          type: "doc",
          id: "reference/api/get-vrm-position",
          label: "Get the position of a VRM model",
          className: "api-method get",
        },
        {
          type: "doc",
          id: "reference/api/get-vrm-persona",
          label: "Get VRM persona",
          className: "api-method get",
        },
        {
          type: "doc",
          id: "reference/api/set-vrm-persona",
          label: "Set VRM persona",
          className: "api-method put",
        },
        {
          type: "doc",
          id: "reference/api/list-vrm-spring-bones",
          label: "List VRM spring bone chains",
          className: "api-method get",
        },
        {
          type: "doc",
          id: "reference/api/get-vrm-spring-bone",
          label: "Get a VRM spring bone chain",
          className: "api-method get",
        },
        {
          type: "doc",
          id: "reference/api/update-vrm-spring-bone",
          label: "Update VRM spring bone properties",
          className: "api-method put",
        },
        {
          type: "doc",
          id: "reference/api/list-vrm-animations",
          label: "List all VRMA animations for a VRM model",
          className: "api-method get",
        },
        {
          type: "doc",
          id: "reference/api/play-vrma-animation",
          label: "Play a VRMA animation",
          className: "api-method post",
        },
        {
          type: "doc",
          id: "reference/api/stop-vrma-animation",
          label: "Stop a VRMA animation",
          className: "api-method post",
        },
        {
          type: "doc",
          id: "reference/api/get-vrma-state",
          label: "Get VRMA animation state",
          className: "api-method get",
        },
        {
          type: "doc",
          id: "reference/api/set-vrma-speed",
          label: "Set VRMA playback speed",
          className: "api-method put",
        },
      ],
    },
    {
      type: "category",
      label: "coordinates",
      items: [
        {
          type: "doc",
          id: "reference/api/convert-to-viewport",
          label: "Convert world coordinates to viewport",
          className: "api-method get",
        },
        {
          type: "doc",
          id: "reference/api/convert-to-world",
          label: "Convert viewport coordinates to world",
          className: "api-method get",
        },
      ],
    },
    {
      type: "category",
      label: "displays",
      items: [
        {
          type: "doc",
          id: "reference/api/get-all-displays",
          label: "Get all display information",
          className: "api-method get",
        },
      ],
    },
    {
      type: "category",
      label: "effects",
      items: [
        {
          type: "doc",
          id: "reference/api/show-stamp-effect",
          label: "Show a stamp effect",
          className: "api-method post",
        },
      ],
    },
    {
      type: "category",
      label: "entities",
      items: [
        {
          type: "doc",
          id: "reference/api/find-entity-by-name",
          label: "Find an entity by name",
          className: "api-method get",
        },
        {
          type: "doc",
          id: "reference/api/get-entity-name",
          label: "Get the name of an entity",
          className: "api-method get",
        },
        {
          type: "doc",
          id: "reference/api/get-entity-transform",
          label: "Get the transform of an entity",
          className: "api-method get",
        },
        {
          type: "doc",
          id: "reference/api/set-entity-transform",
          label: "Set the transform of an entity",
          className: "api-method put",
        },
        {
          type: "doc",
          id: "reference/api/move-entity",
          label: "Move an entity",
          className: "api-method post",
        },
        {
          type: "doc",
          id: "reference/api/tween-position",
          label: "Tween an entity's position to a target value",
          className: "api-method post",
        },
        {
          type: "doc",
          id: "reference/api/tween-rotation",
          label: "Tween an entity's rotation to a target value",
          className: "api-method post",
        },
        {
          type: "doc",
          id: "reference/api/tween-scale",
          label: "Tween an entity's scale to a target value",
          className: "api-method post",
        },
      ],
    },
    {
      type: "category",
      label: "webviews",
      items: [
        {
          type: "doc",
          id: "reference/api/list-webviews",
          label: "List all open webviews",
          className: "api-method get",
        },
        {
          type: "doc",
          id: "reference/api/open-webview",
          label: "Open a webview",
          className: "api-method post",
        },
        {
          type: "doc",
          id: "reference/api/get-webview",
          label: "Get webview details",
          className: "api-method get",
        },
        {
          type: "doc",
          id: "reference/api/patch-webview",
          label: "Partially update a webview",
          className: "api-method patch",
        },
        {
          type: "doc",
          id: "reference/api/delete-webview",
          label: "Delete a webview",
          className: "api-method delete",
        },
        {
          type: "doc",
          id: "reference/api/navigate-webview",
          label: "Navigate webview",
          className: "api-method post",
        },
        {
          type: "doc",
          id: "reference/api/navigate-back-webview",
          label: "Navigate back",
          className: "api-method post",
        },
        {
          type: "doc",
          id: "reference/api/navigate-forward-webview",
          label: "Navigate forward",
          className: "api-method post",
        },
        {
          type: "doc",
          id: "reference/api/reload-webview",
          label: "Reload webview",
          className: "api-method post",
        },
        {
          type: "doc",
          id: "reference/api/is-webview-closed",
          label: "Check if a webview is closed",
          className: "api-method get",
        },
        {
          type: "doc",
          id: "reference/api/get-webview-linked-vrm",
          label: "Get the linked VRM for a webview",
          className: "api-method get",
        },
        {
          type: "doc",
          id: "reference/api/set-webview-linked-vrm",
          label: "Set the linked VRM for a webview",
          className: "api-method put",
        },
        {
          type: "doc",
          id: "reference/api/unlink-webview-vrm",
          label: "Remove the linked VRM from a webview",
          className: "api-method delete",
        },
      ],
    },
    {
      type: "category",
      label: "preferences",
      items: [
        {
          type: "doc",
          id: "reference/api/list-preference-keys",
          label: "List all preference keys",
          className: "api-method get",
        },
        {
          type: "doc",
          id: "reference/api/load-preference",
          label: "Load a preference value by key",
          className: "api-method get",
        },
        {
          type: "doc",
          id: "reference/api/save-preference",
          label: "Save a preference value by key",
          className: "api-method put",
        },
      ],
    },
    {
      type: "category",
      label: "shadow-panel",
      items: [
        {
          type: "doc",
          id: "reference/api/get-shadow-panel-alpha",
          label: "Get the current alpha value of the shadow panel",
          className: "api-method get",
        },
        {
          type: "doc",
          id: "reference/api/set-shadow-panel-alpha",
          label: "Set the alpha value of the shadow panel",
          className: "api-method put",
        },
      ],
    },
    {
      type: "category",
      label: "signals",
      items: [
        {
          type: "doc",
          id: "reference/api/list-signals",
          label: "List signal channels",
          className: "api-method get",
        },
        {
          type: "doc",
          id: "reference/api/stream-signal",
          label: "Stream signal events",
          className: "api-method get",
        },
        {
          type: "doc",
          id: "reference/api/send-signal",
          label: "Send a signal",
          className: "api-method post",
        },
      ],
    },
    {
      type: "category",
      label: "mods",
      items: [
        {
          type: "doc",
          id: "reference/api/list-mods",
          label: "List all loaded mods",
          className: "api-method get",
        },
        {
          type: "doc",
          id: "reference/api/get-mod",
          label: "Get a single mod by name",
          className: "api-method get",
        },
        {
          type: "doc",
          id: "reference/api/list-mod-menus",
          label: "List registered mod menus",
          className: "api-method get",
        },
      ],
    },
    {
      type: "category",
      label: "commands",
      items: [
        {
          type: "doc",
          id: "reference/api/execute-command",
          label: "Execute a command",
          className: "api-method post",
        },
      ],
    },
    {
      type: "category",
      label: "app",
      items: [
        {
          type: "doc",
          id: "reference/api/health-check",
          label: "Health check",
          className: "api-method get",
        },
        {
          type: "doc",
          id: "reference/api/exit-application",
          label: "Exit the application",
          className: "api-method post",
        },
        {
          type: "doc",
          id: "reference/api/get-info",
          label: "Application info",
          className: "api-method get",
        },
      ],
    },
    {
      type: "category",
      label: "audio",
      items: [
        {
          type: "doc",
          id: "reference/api/play-se",
          label: "Play a sound effect",
          className: "api-method post",
        },
        {
          type: "doc",
          id: "reference/api/play-bgm",
          label: "Play background music",
          className: "api-method post",
        },
        {
          type: "doc",
          id: "reference/api/get-bgm-status",
          label: "Get BGM status",
          className: "api-method get",
        },
        {
          type: "doc",
          id: "reference/api/update-bgm",
          label: "Update BGM playback parameters",
          className: "api-method patch",
        },
        {
          type: "doc",
          id: "reference/api/stop-bgm",
          label: "Stop background music",
          className: "api-method post",
        },
        {
          type: "doc",
          id: "reference/api/pause-bgm",
          label: "Pause background music",
          className: "api-method post",
        },
        {
          type: "doc",
          id: "reference/api/resume-bgm",
          label: "Resume background music",
          className: "api-method post",
        },
      ],
    },
    {
      type: "category",
      label: "assets",
      items: [
        {
          type: "doc",
          id: "reference/api/list-assets",
          label: "List available assets",
          className: "api-method get",
        },
      ],
    },
  ],
};

export default sidebar.apisidebar;
