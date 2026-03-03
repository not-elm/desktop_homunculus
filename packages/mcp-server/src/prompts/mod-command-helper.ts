import type { McpServer } from "@modelcontextprotocol/sdk/server/mcp.js";
import { z } from "zod";

export function registerModCommandHelperPrompt(server: McpServer): void {
  server.prompt(
    "mod-command-helper",
    "Discover and execute MOD commands",
    { mod_name: z.string().describe("MOD name to explore") },
    (args) => ({
      messages: [{
        role: "user",
        content: {
          type: "text",
          text: `Help me use the "${args.mod_name}" MOD. First, read the homunculus://mods resource to find available commands for this MOD. Then explain what each command does and how to use it with the execute_command tool. Show example execute_command calls with proper arguments.`,
        },
      }],
    }),
  );
}
