#!/usr/bin/env tsx

/// <reference types="node" />

import { audio, Webview, WebviewLayer, webviewSource } from '@hmcs/sdk';
import { output } from '@hmcs/sdk/commands';

try {
  await Webview.open({
    source: webviewSource.local('openclaw:ui'),
    size: [0.7, 0.8],
    viewportSize: [620, 560],
    transform: { translation: [0, 0, WebviewLayer.UI] },
  });
  await audio.se.play('se:open');
  output.succeed();
} catch (e) {
  output.fail('OPEN_UI_FAILED', (e as Error).message);
}
