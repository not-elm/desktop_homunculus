#!/usr/bin/env tsx

/// <reference types="node" />

import { audio, Webview, WebviewLayer, webviewSource } from '@hmcs/sdk';
import { input, output } from '@hmcs/sdk/commands';
import { z } from 'zod';

try {
  const { linkedPersona: personaId } = await input.parse(z.object({ linkedPersona: z.string() }));
  await Webview.open({
    source: webviewSource.local('openclaw:ui'),
    size: [0.45, 0.27],
    viewportSize: [500, 300],
    transform: { translation: [1.1, 0.8, WebviewLayer.UI] },
    linkedPersona: personaId,
  });
  await audio.se.play('se:open');
  output.succeed();
} catch (e) {
  output.fail('OPEN_UI_FAILED', (e as Error).message);
}
