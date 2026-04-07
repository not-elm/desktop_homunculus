#!/usr/bin/env tsx
/// <reference types="node" />

import { audio, Webview, webviewSource } from "@hmcs/sdk";
import { input, output } from "@hmcs/sdk/commands";

try {
  const vrm = await input.parseMenu();

  await Webview.open({
    source: webviewSource.local("agent:session-ui"),
    size: [1.07, 0.8],
    viewportSize: [800, 500],
    offset: [1.3, -0.5],
    linkedVrm: vrm.entity,
  });
  await audio.se.play("se:open");
  output.succeed();
} catch (e) {
  output.fail("OPEN_AGENT_FAILED", (e as Error).message);
}
