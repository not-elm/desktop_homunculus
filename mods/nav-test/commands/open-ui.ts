#!/usr/bin/env tsx

/// <reference types="node" />

import { audio, Webview, WebviewLayer, webviewSource } from '@hmcs/sdk';
import { output } from '@hmcs/sdk/commands';

try {
  await Webview.open({
    source: webviewSource.local('nav-test:ui'),
    size: [0.6, 0.6],
    viewportSize: [500, 400],
    transform: { translation: [0, 0, WebviewLayer.UI] },
    resizable: {},
  });
  await audio.se.play('se:open');
  output.succeed();
} catch (e) {
  output.fail('OPEN_UI_FAILED', (e as Error).message);
}
