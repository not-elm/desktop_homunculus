#!/usr/bin/env tsx
/// <reference types="node" />

import { z } from "zod";
import { rpc } from "@hmcs/sdk/rpc";
import { input, output } from "@hmcs/sdk/commands";
import { signals } from "@hmcs/sdk";

try {
  const { linkedPersona: personaId } = await input.parse(z.object({ linkedPersona: z.string() }));
  await rpc.call({
    modName: "@hmcs/agent",
    method: "start-session",
    body: { personaId },
  });
  output.succeed();
} catch (e) {
  const message = e instanceof Error ? e.message : String(e);
  signals.send("agent:error", { personaId: "*", message }).catch(() => {});
  output.fail("START_SESSION_FAILED", message);
}
