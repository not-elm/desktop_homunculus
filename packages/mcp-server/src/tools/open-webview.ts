import type { McpServer } from "@modelcontextprotocol/sdk/server/mcp.js";
import { z } from "zod";
import { Webview, webviewSource } from "@hmcs/sdk";
import type { HomunculusMcpState } from "../state.js";
import { handleApiError } from "./shared.js";

export function registerOpenWebview(server: McpServer, state: HomunculusMcpState): void {
  server.tool(
    "open_webview",
    "Open a webview panel displaying HTML content or a URL near the active character. Returns the webview entity ID. Use close_webview to close it.",
    {
      html: z.string().optional().describe("Inline HTML content to display (mutually exclusive with url)"),
      url: z.string().optional().describe("URL or mod asset path to load (mutually exclusive with html)"),
      size_x: z.number().optional().default(0.7).describe("Panel width in world units"),
      size_y: z.number().optional().default(0.5).describe("Panel height in world units"),
      viewport_width: z.number().optional().default(800).describe("Internal browser width in pixels"),
      viewport_height: z.number().optional().default(600).describe("Internal browser height in pixels"),
      offset_x: z.number().optional().default(0).describe("Horizontal offset from character center"),
      offset_y: z.number().optional().default(0.5).describe("Vertical offset from character center (positive = above)"),
    },
    async (args) => {
      try {
        if (!args.html && !args.url) {
          return {
            content: [{ type: "text" as const, text: "Either 'html' or 'url' must be provided." }],
            isError: true,
          };
        }

        const source = args.html
          ? webviewSource.html(args.html)
          : webviewSource.url(args.url!);

        let linkedVrm: number | undefined;
        try {
          linkedVrm = await state.resolveCharacter();
        } catch {
          linkedVrm = undefined;
        }

        const webview = await Webview.open({
          source,
          size: [args.size_x, args.size_y],
          viewportSize: [args.viewport_width, args.viewport_height],
          offset: [args.offset_x, args.offset_y],
          linkedVrm,
        });

        state.trackWebview(webview.entity);

        return {
          content: [{ type: "text" as const, text: `Opened webview (entity ${webview.entity})` }],
        };
      } catch (error) {
        return handleApiError(error);
      }
    },
  );
}
