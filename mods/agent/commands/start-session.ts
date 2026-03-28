#!/usr/bin/env tsx
/// <reference types="node" />

import { rpc } from "@hmcs/sdk/rpc";
import { input, output } from "@hmcs/sdk/commands";
import { signals } from "@hmcs/sdk";

try {
  const vrm = await input.parseMenu();
  const characterId = await vrm.name();
  await rpc.call({
    modName: "@hmcs/agent",
    method: "start-session",
    body: { characterId },
  });
  output.succeed();
} catch (e) {
  const message = e instanceof Error ? e.message : String(e);
  signals.send("agent:error", { characterId: "*", message }).catch(() => {});
  output.fail("START_SESSION_FAILED", message);
}
