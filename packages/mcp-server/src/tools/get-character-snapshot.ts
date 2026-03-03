import type { McpServer } from "@modelcontextprotocol/sdk/server/mcp.js";
import { Vrm } from "@hmcs/sdk";
import type { HomunculusMcpState } from "../state.js";
import { handleApiError } from "./shared.js";

export function registerGetCharacterSnapshot(server: McpServer, state: HomunculusMcpState): void {
  server.tool(
    "get_character_snapshot",
    "Get the current state of all desktop characters including name, entity ID, position, active expressions, playing animations, persona, and lookAt state.",
    {},
    async () => {
      try {
        const snapshots = await Vrm.findAllDetailed();
        const result = snapshots.map((s) => ({
          entity: s.entity,
          name: s.name,
          state: s.state,
          position: s.globalViewport,
          activeExpressions: s.expressions.expressions
            .filter((e) => e.weight > 0.01)
            .map((e) => ({ name: e.name, weight: Math.round(e.weight * 100) / 100 })),
          playingAnimations: s.animations.filter((a) => a.playing).map((a) => a.name),
          persona: { profile: s.persona.profile, personality: s.persona.personality ?? null },
          lookAt: s.lookAt,
        }));
        if (result.length > 0 && state.activeCharacterEntity === null) {
          state.activeCharacterEntity = result[0].entity;
        }
        return { content: [{ type: "text" as const, text: JSON.stringify(result, null, 2) }] };
      } catch (error) {
        return handleApiError(error);
      }
    },
  );
}
