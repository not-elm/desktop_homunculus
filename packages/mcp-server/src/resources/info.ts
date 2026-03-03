import type { McpServer } from "@modelcontextprotocol/sdk/server/mcp.js";
import { app } from "@hmcs/sdk";

export function registerInfoResource(server: McpServer): void {
  server.resource("homunculus-info", "homunculus://info", async (uri) => ({
    contents: [{
      uri: uri.href,
      mimeType: "application/json",
      text: JSON.stringify(await app.info(), null, 2),
    }],
  }));
}
