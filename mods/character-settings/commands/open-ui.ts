#!/usr/bin/env tsx

/// <reference types="node" />

import { audio, Webview, webviewSource } from "@hmcs/sdk";
import { input } from "@hmcs/sdk/commands";
try {
  const vrm = await input.parseMenu();
  await Webview.open({
    source: webviewSource.local("character-settings:ui"),
    size: [1, 0.9],
    viewportSize: [900, 700],
    offset: [1.1, 0],
    linkedVrm: vrm.entity,
  });
  await audio.se.play("se:open");
} catch (e) {
  console.error(e)
}
process.exit(0);
