#!/usr/bin/env tsx
/// <reference types="node" />

import { audio, Webview, webviewSource } from "@hmcs/sdk";
import { input, output } from "@hmcs/sdk/commands";

try {
  const vrm = await input.parseMenu();
  await Webview.open({
    source: webviewSource.local("agent:settings-ui"),
    size: [0.55, 0.75],
    viewportSize: [800, 600],
    linkedVrm: vrm.entity,
    offset: [-0.6, -0.3],
  });
  await audio.se.play("se:open");
  output.succeed();
} catch (e) {
  output.fail("OPEN_SETTINGS_FAILED", (e as Error).message);
}
