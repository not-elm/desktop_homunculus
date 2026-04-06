#!/usr/bin/env tsx

/// <reference types="node" />

import { audio, Webview, webviewSource } from "@hmcs/sdk";
import { output } from "@hmcs/sdk/commands";

try {
  await Webview.open({
    source: webviewSource.local("stt:ui"),
    size: [0.8, 1],
    viewportSize: [700, 800],
    offset: [1.1, 0],
  });
  await audio.se.play("se:open");
  output.succeed();
} catch (e) {
  output.fail("OPEN_UI_FAILED", (e as Error).message);
}
