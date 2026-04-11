#!/usr/bin/env tsx

/// <reference types="node" />

import { input, output } from '@hmcs/sdk/commands';
import { rpc } from '@hmcs/sdk/rpc';
import { z } from 'zod';

try {
  const { linkedPersona: personaId } = await input.parse(z.object({ linkedPersona: z.string() }));
  await rpc.call({
    modName: '@hmcs/agent',
    method: 'stop-session',
    body: { personaId },
  });
  output.succeed();
} catch (e) {
  output.fail('STOP_SESSION_FAILED', (e as Error).message);
}
