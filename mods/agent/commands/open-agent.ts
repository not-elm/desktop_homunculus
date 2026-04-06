#!/usr/bin/env tsx
/// <reference types="node" />

import { audio, Webview, webviewSource } from "@hmcs/sdk";
import { input, output } from "@hmcs/sdk/commands";
import { rpc } from "@hmcs/sdk/rpc";

try {
  const vrm = await input.parseMenu();

  // Determine initial geometry from session status
  let isSession = false;
  try {
    const characterId = await vrm.name();
    const { status } = await rpc.call<{ status: string }>({
      modName: "@hmcs/agent",
      method: "get-session-status",
      body: { characterId },
    });
    isSession = status !== "idle";
  } catch {
    // If RPC fails (service not ready), default to settings mode
  }

  await Webview.open({
    source: webviewSource.local("agent:session-ui"),
    size:         isSession ? [0.6, 0.8]        : [1.3333, 1.0],
    viewportSize: isSession ? [400, 500]        : [1200, 900],
    offset:       isSession ? [0.8, -0.5]       : [-0.6, -0.3, -10.0],
    linkedVrm: vrm.entity,
  });
  await audio.se.play("se:open");
  output.succeed();
} catch (e) {
  output.fail("OPEN_AGENT_FAILED", (e as Error).message);
}
