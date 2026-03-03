import { HomunculusApiError } from "@hmcs/sdk";

export function handleApiError(error: unknown): { content: { type: "text"; text: string }[]; isError: true } {
  if (error instanceof HomunculusApiError) {
    if (error.statusCode === 404) {
      return {
        content: [{ type: "text", text: `Not found: ${error.body}. Use get_character_snapshot to check available characters.` }],
        isError: true,
      };
    }
    if (error.statusCode === 400) {
      return {
        content: [{ type: "text", text: `Invalid request: ${error.body}` }],
        isError: true,
      };
    }
    return {
      content: [{ type: "text", text: `API error (${error.statusCode}): ${error.body}` }],
      isError: true,
    };
  }
  if (error instanceof Error) {
    if ("code" in error && (error as NodeJS.ErrnoException).code === "ECONNREFUSED") {
      return {
        content: [{ type: "text", text: "Desktop Homunculus is not running. Please start the application first." }],
        isError: true,
      };
    }
    return {
      content: [{ type: "text", text: `Error: ${error.message}` }],
      isError: true,
    };
  }
  return {
    content: [{ type: "text", text: `Unknown error: ${String(error)}` }],
    isError: true,
  };
}
