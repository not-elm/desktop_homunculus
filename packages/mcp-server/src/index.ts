#!/usr/bin/env node
import { McpServer } from "@modelcontextprotocol/sdk/server/mcp.js";
import { StdioServerTransport } from "@modelcontextprotocol/sdk/server/stdio.js";
import { host } from "@hmcs/sdk";
import { HomunculusMcpState } from "./state.js";
import { registerTools } from "./tools/index.js";
import { registerResources } from "./resources/index.js";
import { registerPrompts } from "./prompts/index.js";

const BASE_URL = process.env.HOMUNCULUS_HOST
  ? `http://${process.env.HOMUNCULUS_HOST}`
  : "http://localhost:3100";

host.configure({ baseUrl: BASE_URL });

const server = new McpServer({
  name: "homunculus",
  version: "0.1.0",
});

const state = new HomunculusMcpState();

registerTools(server, state);
registerResources(server);
registerPrompts(server);

const transport = new StdioServerTransport();
await server.connect(transport);
