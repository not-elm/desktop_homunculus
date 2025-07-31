import {
    AssistantRuntimeProvider,
    type ChatModelAdapter,
    type TextMessagePart,
    useLocalRuntime
} from "@assistant-ui/react";
import type { ReactNode } from "react";
import { gpt, Vrm } from "@homunculus/api";

const homunculusRuntimeProvider: ChatModelAdapter = {
    async run(options) {
        const vrm = Vrm.caller();
        if (!vrm) {
            return {
                content: []
            }
        }
        const userMessage = options
            .messages
            .flatMap(m => m.content as unknown)
            .map(m => isTextMessagePart(m) ? m.text : "")
            .join("\n");
        const response = await gpt.chat(userMessage, {
            vrm: vrm.entity,
        });
        return {
            content: [{
                type: "text",
                text: response.message,
            }]
        }
    }
}

const isTextMessagePart = (part: unknown): part is TextMessagePart => {
    return typeof part === "object" && part !== null && "type" in part && part.type === "text";
}

export function HomunculusProvider({ children }: Readonly<{
    children: ReactNode;
}>) {
    const runtime = useLocalRuntime(homunculusRuntimeProvider);
    return (
        <AssistantRuntimeProvider runtime={runtime}>
            {children}
        </AssistantRuntimeProvider>
    );
}

