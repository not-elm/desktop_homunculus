#!/usr/bin/env tsx

/// <reference types="node" />

import { execSync } from "node:child_process";
import { preferences } from "@hmcs/sdk";
import { output } from "@hmcs/sdk/commands";

interface ReleaseInfo {
  tag: string;
  url: string;
}

try {
  const release = await preferences.load<ReleaseInfo>("update-notify:release");
  if (!release?.url) {
    output.fail("NO_RELEASE_URL", "No release URL found in preferences");
    process.exit(1);
  }

  if (!release.url.startsWith("https://github.com/")) {
    output.fail("INVALID_URL", "Release URL is not a GitHub URL");
    process.exit(1);
  }

  const cmd = buildOpenCommand(release.url);
  execSync(cmd);
  output.succeed();
} catch (e) {
  output.fail("BROWSER_OPEN_FAILED", (e as Error).message);
}

function buildOpenCommand(url: string): string {
  switch (process.platform) {
    case "darwin":
      return `open "${url}"`;
    case "win32":
      return `start "" "${url}"`;
    default:
      return `xdg-open "${url}"`;
  }
}
