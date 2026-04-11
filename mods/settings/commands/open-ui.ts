#!/usr/bin/env tsx

/// <reference types="node" />

import { audio, Webview, WebviewLayer, webviewSource } from '@hmcs/sdk';
import { output } from '@hmcs/sdk/commands';

try {
  await Webview.open({
    source: webviewSource.local('settings:ui'),
    size: [0.8, 1],
    viewportSize: [600, 800],
    transform: { translation: [1.1, 0.8, WebviewLayer.UI] },
  });
  await audio.se.play('se:open');
  output.succeed();
} catch (e) {
  output.fail('OPEN_UI_FAILED', (e as Error).message);
}
