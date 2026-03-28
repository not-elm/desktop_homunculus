#!/usr/bin/env tsx
/// <reference types="node" />

import { rpc } from "@hmcs/sdk/rpc";
import { input, output } from "@hmcs/sdk/commands";

try {
  const vrm = await input.parseMenu();
  const characterId = await vrm.name();
  await rpc.call({
    modName: "@hmcs/agent",
    method: "stop-session",
    body: { characterId },
  });
  output.succeed();
} catch (e) {
  output.fail("STOP_SESSION_FAILED", (e as Error).message);
}
