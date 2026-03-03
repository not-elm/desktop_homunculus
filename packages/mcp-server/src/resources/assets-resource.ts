import type { McpServer } from "@modelcontextprotocol/sdk/server/mcp.js";
import { assets } from "@hmcs/sdk";

export function registerAssetsResource(server: McpServer): void {
  server.resource("homunculus-assets", "homunculus://assets", async (uri) => ({
    contents: [{
      uri: uri.href,
      mimeType: "application/json",
      text: JSON.stringify(await assets.list(), null, 2),
    }],
  }));
}
