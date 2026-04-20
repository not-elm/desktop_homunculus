import { host } from '@hmcs/sdk';
import type { OpenClawPluginApi } from 'openclaw/plugin-sdk/plugin-entry';
import { definePluginEntry } from 'openclaw/plugin-sdk/plugin-entry';
import type { PluginDeps } from './deps.js';
import { createBootstrapHandler } from './hooks/index.js';
import { createPluginCache } from './persona-cache.js';
import { createSyncRunner } from './sync/index.js';

export default definePluginEntry({
  id: 'hmcs-openclaw',
  name: 'Desktop Homunculus Bridge',
  description: 'Renders OpenClaw agent replies on HMCS characters.',
  register(api: OpenClawPluginApi) {
    const deps: PluginDeps = {
      api,
      cache: createPluginCache(),
      config: {
        hmcsBaseUrl: 'http://127.0.0.1:3100',
        soulMaxChars: 10000,
      },
      logger: api.logger,
    };

    // The @hmcs/sdk client stores its base URL as module-level state
    // (`host._baseUrl`). The plugin is currently the only SDK consumer inside
    // the OpenClaw runtime, so configuring once at register is safe; revisit
    // if SDK grows an instance-based client or if another consumer ships.
    host.configure({ baseUrl: deps.config.hmcsBaseUrl });

    deps.logger.info('hmcs-openclaw plugin registered');

    api.registerHook('agent:bootstrap', createBootstrapHandler(deps), {
      name: 'hmcs-openclaw.bootstrap',
    });

    const runner = createSyncRunner(deps);
    api.registerService({
      id: 'persona-sync',
      async start() {
        await runner.start();
      },
      async stop() {
        await runner.stop();
      },
    });
  },
});
