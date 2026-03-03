import type { McpServer } from "@modelcontextprotocol/sdk/server/mcp.js";
import { z } from "zod";
import { mods } from "@hmcs/sdk";
import { handleApiError } from "./shared.js";

export function registerExecuteCommand(server: McpServer): void {
  server.tool(
    "execute_command",
    "Execute a MOD command (e.g., VoiceVox speak, initialize). Returns stdout, stderr, and exit code. Use 'mods' resource to discover available commands.",
    {
      command: z.string().describe("Command name to execute"),
      args: z.array(z.string()).optional().describe("Command arguments"),
      stdin: z.string().optional().describe("Standard input to pass to the command"),
      timeoutMs: z.number().min(1000).max(300000).default(30000).describe("Timeout in milliseconds"),
    },
    async (args) => {
      try {
        const result = await mods.executeCommand({
          command: args.command,
          args: args.args,
          stdin: args.stdin,
          timeoutMs: args.timeoutMs,
        });
        const output = [
          result.stdout ? `stdout:\n${result.stdout}` : null,
          result.stderr ? `stderr:\n${result.stderr}` : null,
          `exit code: ${result.exitCode}`,
          result.timedOut ? "TIMED OUT" : null,
        ]
          .filter(Boolean)
          .join("\n\n");
        return {
          content: [{ type: "text" as const, text: output }],
          isError: result.exitCode !== 0 || result.timedOut,
        };
      } catch (error) {
        return handleApiError(error);
      }
    },
  );
}
