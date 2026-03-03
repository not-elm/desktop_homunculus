import type { McpServer } from "@modelcontextprotocol/sdk/server/mcp.js";
import { registerInfoResource } from "./info.js";
import { registerCharactersResource } from "./characters.js";
import { registerModsResource } from "./mods-resource.js";
import { registerAssetsResource } from "./assets-resource.js";

export function registerResources(server: McpServer): void {
  registerInfoResource(server);
  registerCharactersResource(server);
  registerModsResource(server);
  registerAssetsResource(server);
}
