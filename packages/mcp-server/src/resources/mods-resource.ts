import type { McpServer } from "@modelcontextprotocol/sdk/server/mcp.js";
import { mods } from "@hmcs/sdk";

export function registerModsResource(server: McpServer): void {
  server.resource("homunculus-mods", "homunculus://mods", async (uri) => ({
    contents: [{
      uri: uri.href,
      mimeType: "application/json",
      text: JSON.stringify(await mods.list(), null, 2),
    }],
  }));
}
