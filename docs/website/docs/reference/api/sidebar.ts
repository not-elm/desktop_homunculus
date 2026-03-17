import type { SidebarsConfig } from "@docusaurus/plugin-content-docs";

const sidebar: SidebarsConfig = {
  apisidebar: [
    {
      type: "doc",
      id: "reference/api/homunculus-api",
    },
    {
      type: "category",
      label: "app",
      items: [
        {
          type: "doc",
          id: "reference/api/exit",
          label: "Exit the application gracefully.",
          className: "api-method post",
        },
        {
          type: "doc",
          id: "reference/api/health",
          label: "Returns a simple health check response.",
          className: "api-method get",
        },
        {
          type: "doc",
          id: "reference/api/get",
          label: "Returns application metadata including version, platform, features, and loaded mods.",
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
          id: "reference/api/bgm-status",
          label: "Get current BGM status.",
          className: "api-method get",
        },
        {
          type: "doc",
          id: "reference/api/bgm-play",
          label: "Play background music (replaces current BGM).",
          className: "api-method post",
        },
        {
          type: "doc",
          id: "reference/api/bgm-update",
          label: "Update BGM settings (volume, speed).",
          className: "api-method patch",
        },
        {
          type: "doc",
          id: "reference/api/bgm-pause",
          label: "Pause background music.",
          className: "api-method post",
        },
        {
          type: "doc",
          id: "reference/api/bgm-resume",
          label: "Resume background music.",
          className: "api-method post",
        },
        {
          type: "doc",
          id: "reference/api/bgm-stop",
          label: "Stop background music.",
          className: "api-method post",
        },
        {
          type: "doc",
          id: "reference/api/se-play",
          label: "Play a one-shot sound effect.",
          className: "api-method post",
        },
      ],
    },
    {
      type: "category",
      label: "vrm",
      items: [
        {
          type: "doc",
          id: "reference/api/get",
          label: "List VRM model entities, optionally filtered by name.",
          className: "api-method get",
        },
        {
          type: "doc",
          id: "reference/api/snapshot",
          label: "Get detailed snapshot of all VRM instances.",
          className: "api-method get",
        },
        {
          type: "doc",
          id: "reference/api/spawn",
          label: "Spawn a VRM model.",
          className: "api-method post",
        },
        {
          type: "doc",
          id: "reference/api/stream",
          label: "Stream VRM model load events via SSE.",
          className: "api-method get",
        },
        {
          type: "doc",
          id: "reference/api/wait-load",
          label: "Wait for a VRM model to load.",
          className: "api-method get",
        },
        {
          type: "doc",
          id: "reference/api/despawn",
          label: "Despawn a VRM model.",
          className: "api-method delete",
        },
        {
          type: "doc",
          id: "reference/api/events",
          label: "Subscribe to VRM events via SSE.",
          className: "api-method get",
        },
        {
          type: "doc",
          id: "reference/api/list",
          label: "List all expressions and their current weights for a VRM model.",
          className: "api-method get",
        },
        {
          type: "doc",
          id: "reference/api/set",
          label: "Set all expression weights for a VRM model (replaces all current weights).",
          className: "api-method put",
        },
        {
          type: "doc",
          id: "reference/api/clear",
          label: "Clear all expression weights for a VRM model.",
          className: "api-method delete",
        },
        {
          type: "doc",
          id: "reference/api/modify",
          label: "Modify specific expression weights for a VRM model (merges with current weights).",
          className: "api-method patch",
        },
        {
          type: "doc",
          id: "reference/api/modify-mouth",
          label: "Modify mouth expression weights for a VRM model.",
          className: "api-method patch",
        },
        {
          type: "doc",
          id: "reference/api/unlook",
          label: "Disable look-at control for the specified VRM entity.",
          className: "api-method delete",
        },
        {
          type: "doc",
          id: "reference/api/cursor",
          label: "Set look-at to follow the cursor.",
          className: "api-method put",
        },
        {
          type: "doc",
          id: "reference/api/target",
          label: "Set look-at target to another entity.",
          className: "api-method put",
        },
        {
          type: "doc",
          id: "reference/api/get",
          label: "Get the persona of a VRM model.",
          className: "api-method get",
        },
        {
          type: "doc",
          id: "reference/api/put",
          label: "Set the persona of a VRM model.",
          className: "api-method put",
        },
        {
          type: "doc",
          id: "reference/api/get",
          label: "Get the position of a VRM model.",
          className: "api-method get",
        },
        {
          type: "doc",
          id: "reference/api/timeline",
          label: "Speak with a timeline of expression keyframes and audio data.",
          className: "api-method post",
        },
        {
          type: "doc",
          id: "reference/api/list",
          label: "List all spring bone chains for a VRM model.",
          className: "api-method get",
        },
        {
          type: "doc",
          id: "reference/api/get",
          label: "Get a specific spring bone chain by ID.",
          className: "api-method get",
        },
        {
          type: "doc",
          id: "reference/api/put",
          label: "Update properties of a spring bone chain.",
          className: "api-method put",
        },
        {
          type: "doc",
          id: "reference/api/get",
          label: "Get the state of a VRM model.",
          className: "api-method get",
        },
        {
          type: "doc",
          id: "reference/api/put",
          label: "Set the state of a VRM model.",
          className: "api-method put",
        },
        {
          type: "doc",
          id: "reference/api/get",
          label: "List all VRMA animations under a VRM entity.",
          className: "api-method get",
        },
        {
          type: "doc",
          id: "reference/api/play",
          label: "Play a VRM animation.",
          className: "api-method post",
        },
        {
          type: "doc",
          id: "reference/api/speed",
          label: "Set the playback speed for a VRM animation.",
          className: "api-method put",
        },
        {
          type: "doc",
          id: "reference/api/state",
          label: "Get the state of a VRM animation.",
          className: "api-method get",
        },
        {
          type: "doc",
          id: "reference/api/stop",
          label: "Stop a VRM animation.",
          className: "api-method post",
        },
        {
          type: "doc",
          id: "reference/api/get",
          label: "Get the entity ID of a specific bone in a VRM model.",
          className: "api-method get",
        },
      ],
    },
    {
      type: "category",
      label: "entities",
      items: [
        {
          type: "doc",
          id: "reference/api/get",
          label: "Find an entity by its name.",
          className: "api-method get",
        },
        {
          type: "doc",
          id: "reference/api/move-to",
          label: "Move an entity to a target position.",
          className: "api-method post",
        },
        {
          type: "doc",
          id: "reference/api/get",
          label: "Get the entity name.",
          className: "api-method get",
        },
        {
          type: "doc",
          id: "reference/api/get",
          label: "Get the transform of an entity.",
          className: "api-method get",
        },
        {
          type: "doc",
          id: "reference/api/put",
          label: "Set the transform of an entity.",
          className: "api-method put",
        },
        {
          type: "doc",
          id: "reference/api/tween-position",
          label: "Tween an entity's position to a target value.",
          className: "api-method post",
        },
        {
          type: "doc",
          id: "reference/api/tween-rotation",
          label: "Tween an entity's rotation to a target value.",
          className: "api-method post",
        },
        {
          type: "doc",
          id: "reference/api/tween-scale",
          label: "Tween an entity's scale to a target value.",
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
          id: "reference/api/list",
          label: "List all open webviews.",
          className: "api-method get",
        },
        {
          type: "doc",
          id: "reference/api/open",
          label: "Open a global webview in world space.",
          className: "api-method post",
        },
        {
          type: "doc",
          id: "reference/api/get",
          label: "Get detailed info for a specific webview.",
          className: "api-method get",
        },
        {
          type: "doc",
          id: "reference/api/delete",
          label: "Delete (close) a webview.",
          className: "api-method delete",
        },
        {
          type: "doc",
          id: "reference/api/patch",
          label: "Partial update of a webview.",
          className: "api-method patch",
        },
        {
          type: "doc",
          id: "reference/api/is-closed",
          label: "Check if a webview is closed.",
          className: "api-method get",
        },
        {
          type: "doc",
          id: "reference/api/get-linked-vrm",
          label: "Get the linked VRM for a webview.",
          className: "api-method get",
        },
        {
          type: "doc",
          id: "reference/api/set-linked-vrm",
          label: "Set the linked VRM for a webview.",
          className: "api-method put",
        },
        {
          type: "doc",
          id: "reference/api/unlink-vrm",
          label: "Remove the linked VRM from a webview.",
          className: "api-method delete",
        },
        {
          type: "doc",
          id: "reference/api/navigate",
          label: "Navigate to a new URL.",
          className: "api-method post",
        },
        {
          type: "doc",
          id: "reference/api/navigate-back",
          label: "Navigate back in history.",
          className: "api-method post",
        },
        {
          type: "doc",
          id: "reference/api/navigate-forward",
          label: "Navigate forward in history.",
          className: "api-method post",
        },
        {
          type: "doc",
          id: "reference/api/reload",
          label: "Reload the current page.",
          className: "api-method post",
        },
      ],
    },
    {
      type: "category",
      label: "coordinates",
      items: [
        {
          type: "doc",
          id: "reference/api/global-viewport",
          label: "Convert world coordinates to global viewport coordinates.",
          className: "api-method get",
        },
        {
          type: "doc",
          id: "reference/api/world-2-d",
          label: "Convert a global viewport position to a 2D world position.",
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
          id: "reference/api/stamp",
          label: "Show a stamp effect.",
          className: "api-method post",
        },
      ],
    },
    {
      type: "category",
      label: "preferences",
      items: [
        {
          type: "doc",
          id: "reference/api/list",
          label: "List all saved preference keys.",
          className: "api-method get",
        },
        {
          type: "doc",
          id: "reference/api/load",
          label: "Load a preference value by key.",
          className: "api-method get",
        },
        {
          type: "doc",
          id: "reference/api/save",
          label: "Save a preference value by key.",
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
          id: "reference/api/list",
          label: "List all active signal channels.",
          className: "api-method get",
        },
        {
          type: "doc",
          id: "reference/api/stream",
          label: "Stream events for a specific signal via SSE.",
          className: "api-method get",
        },
        {
          type: "doc",
          id: "reference/api/send",
          label: "Send a signal to all subscribers.",
          className: "api-method post",
        },
      ],
    },
    {
      type: "category",
      label: "settings",
      items: [
        {
          type: "doc",
          id: "reference/api/get",
          label: "Get the current frame rate (FPS).",
          className: "api-method get",
        },
        {
          type: "doc",
          id: "reference/api/put",
          label: "Set the frame rate (FPS). Persists and applies immediately.",
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
          id: "reference/api/get",
          label: "Get the current alpha value of the shadow panel.",
          className: "api-method get",
        },
        {
          type: "doc",
          id: "reference/api/put",
          label: "Set the alpha value of the shadow panel.",
          className: "api-method put",
        },
      ],
    },
    {
      type: "category",
      label: "displays",
      items: [
        {
          type: "doc",
          id: "reference/api/all",
          label: "Get all display information.",
          className: "api-method get",
        },
      ],
    },
    {
      type: "category",
      label: "mods",
      items: [
        {
          type: "doc",
          id: "reference/api/list",
          label: "List all loaded mods.",
          className: "api-method get",
        },
        {
          type: "doc",
          id: "reference/api/list-menus",
          label: "List all registered mod menus.",
          className: "api-method get",
        },
        {
          type: "doc",
          id: "reference/api/get-one",
          label: "Get a single mod by name.",
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
          label: "Execute a mod command with NDJSON streaming output.",
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
          id: "reference/api/list",
          label: "List available assets, optionally filtered by type or mod name.",
          className: "api-method get",
        },
      ],
    },
    {
      type: "category",
      label: "rpc",
      items: [
        {
          type: "doc",
          id: "reference/api/call",
          label: "Proxy `POST /rpc/call` to the MOD service's local HTTP server.",
          className: "api-method post",
        },
        {
          type: "doc",
          id: "reference/api/deregister",
          label: "Deregister a MOD service's RPC endpoint.",
          className: "api-method post",
        },
        {
          type: "doc",
          id: "reference/api/register",
          label: "Register or update a MOD service's RPC methods.",
          className: "api-method post",
        },
        {
          type: "doc",
          id: "reference/api/list-registrations",
          label: "List all current RPC registrations (for introspection / debugging).",
          className: "api-method get",
        },
      ],
    },
  ],
};

export default sidebar.apisidebar;
