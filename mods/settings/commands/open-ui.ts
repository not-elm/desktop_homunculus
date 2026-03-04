#!/usr/bin/env tsx

/// <reference types="node" />

import { audio, Webview, webviewSource } from "@hmcs/sdk";

try {
  await Webview.open({
    source: webviewSource.local("settings:ui"),
    size: [0.6, 0.6],
    viewportSize: [500, 400],
    offset: [1.1, 0],
  });
  await audio.se.play("se:open");
} catch (e) {
  console.error(e);
}
