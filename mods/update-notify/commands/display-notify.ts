#!/usr/bin/env tsx

/// <reference types="node" />

import { Webview, webviewSource } from "@hmcs/sdk";
import { output } from "@hmcs/sdk/commands";

try {
  await Webview.open({
    source: webviewSource.local("update-notify:ui"),
    size: [0.4, 0.2],
    viewportSize: [320, 140],
    offset: [1.0, 0.5],
  });
  output.succeed();
} catch (e) {
  output.fail("DISPLAY_NOTIFY_FAILED", (e as Error).message);
}
