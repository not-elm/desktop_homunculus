import type { McpServer } from "@modelcontextprotocol/sdk/server/mcp.js";
import { Vrm } from "@hmcs/sdk";

export function registerCharactersResource(server: McpServer): void {
  server.resource("homunculus-characters", "homunculus://characters", async (uri) => ({
    contents: [{
      uri: uri.href,
      mimeType: "application/json",
      text: JSON.stringify(await Vrm.findAllDetailed(), null, 2),
    }],
  }));
}
