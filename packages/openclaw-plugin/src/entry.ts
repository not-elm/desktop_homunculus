import { definePluginEntry } from "openclaw/plugin-sdk/plugin-entry";
import type { OpenClawPluginApi } from "openclaw/plugin-sdk/plugin-entry";
import type { PluginDeps } from "./deps.js";
import { createPluginCache } from "./persona-cache.js";
import { createBootstrapHandler } from "./hooks/index.js";
import { createSyncRunner } from "./sync/index.js";

export default definePluginEntry({
  id: "hmcs-openclaw",
  name: "Desktop Homunculus Bridge",
  description: "Renders OpenClaw agent replies on DH characters.",
  register(api: OpenClawPluginApi) {
    const deps: PluginDeps = {
      api,
      cache: createPluginCache(),
      config: {
        dhBaseUrl: "http://127.0.0.1:3100",
        reconcileIntervalSec: 30,
        soulMaxChars: 10000,
      },
      logger: api.logger,
    };

    deps.logger.info("hmcs-openclaw plugin registered");

    api.registerHook("agent:bootstrap", createBootstrapHandler(deps), {
      name: "hmcs-openclaw.bootstrap",
    });

    const runner = createSyncRunner(deps);
    api.registerService({
      id: "persona-sync",
      async start() {
        await runner.start();
      },
      async stop() {
        await runner.stop();
      },
    });
  },
});
