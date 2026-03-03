import type { McpServer } from "@modelcontextprotocol/sdk/server/mcp.js";
import { z } from "zod";
import { mods } from "@hmcs/sdk";
import type { HomunculusMcpState } from "../state.js";
import { handleApiError } from "./shared.js";

export function registerSpeakMessage(server: McpServer, state: HomunculusMcpState): void {
  server.tool(
    "speak_message",
    "Make the character speak a message aloud using text-to-speech. Requires VoiceVox MOD to be installed and running.",
    {
      text: z.union([z.string(), z.array(z.string())]).describe("Text for the character to speak. Pass an array of short sentences for more reliable TTS."),
      speaker: z.number().default(0).describe("VoiceVox speaker ID"),
      timeoutMs: z.number().min(1000).max(120000).default(30000).optional().describe("Timeout in milliseconds for the speak command (default: 30000)"),
    },
    async (args) => {
      try {
        const entity = await state.resolveCharacter();
        const timeout = args.timeoutMs ?? 30000;
        const result = await mods.executeCommand({
          command: "speak",
          stdin: JSON.stringify({ entity, text: args.text, speaker: args.speaker, fetch_timeout_ms: timeout }),
          timeoutMs: timeout + 5000,
        });
        if (result.exitCode !== 0) {
          return {
            content: [{ type: "text" as const, text: `Speech failed: ${result.stderr || "Unknown error"}. Is VoiceVox MOD installed and VoiceVox engine running?` }],
            isError: true,
          };
        }
        const displayText = Array.isArray(args.text) ? args.text.join(" ") : args.text;
        return {
          content: [{ type: "text" as const, text: `Character is speaking: "${displayText}"` }],
        };
      } catch (error) {
        return handleApiError(error);
      }
    },
  );
}
