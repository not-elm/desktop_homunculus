import { host } from '@hmcs/sdk';
import { rpc } from '@hmcs/sdk/rpc';
import type { OpenClawPluginApi } from 'openclaw/plugin-sdk/plugin-entry';
import { definePluginEntry } from 'openclaw/plugin-sdk/plugin-entry';
import type { PluginDeps } from './deps.js';
import {
  createBootstrapHandler,
  createReplyDispatchHandler,
  createSessionEndHandler,
} from './hooks/index.js';
import { createPluginCache } from './persona-cache.js';
import { sanitizeForTts } from './sanitize/tts.js';
import { createSpeakDebouncer } from './speak-debouncer.js';
import { createSyncRunner } from './sync/index.js';
import { resolveTtsModName } from './tts-resolver.js';

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

    const debouncer = createSpeakDebouncer({
      speak: (payload) => speakViaTts(payload.agentId, payload.text),
      logger: deps.logger,
    });

    api.on('reply_dispatch', createReplyDispatchHandler({ debouncer, logger: deps.logger }));
    api.on('session_end', createSessionEndHandler({ debouncer }));

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

/**
 * Speak `text` on behalf of `personaId` via the TTS MOD currently selected
 * in the persona's metadata. Skips the call entirely when the selection is
 * null / absent / unreadable (text output still flows through OpenClaw).
 */
export async function speakViaTts(personaId: string, text: string): Promise<void> {
  const ttsModName = await resolveTtsModName(personaId);
  if (ttsModName === null) return;
  const { sentences } = sanitizeForTts(text);
  if (sentences.length === 0) return;
  await rpc.call({
    modName: ttsModName,
    method: 'speak',
    body: { personaId, text: sentences },
  });
}
