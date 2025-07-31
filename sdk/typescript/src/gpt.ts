import {host} from "./host";
import {SpeakOnVoiceVoxOptions} from "./vrm";

/**
 * GPT API namespace for interacting with language models and chat functionality.
 *
 * Provides comprehensive functionality for:
 * - Chat interactions with AI models
 * - Model selection and configuration
 * - System prompt management
 * - Web search integration
 * - Voice synthesis integration via VoiceVox
 *
 * @example
 * ```typescript
 * // Chat with a VRM using GPT
 * const vrm = await Vrm.findByName("MyCharacter");
 * const response = await gpt.chat("Hello, how are you?", {
 *   vrm: vrm.entity,
 *   speaker: 1
 * });
 * console.log(response.message);
 *
 * // Change GPT model
 * await gpt.saveModel("gpt-4");
 *
 * // Set system prompt for a specific VRM
 * await gpt.saveSystemPrompt("You are a helpful assistant.", { vrm: vrm.entity });
 * ```
 */
export namespace gpt {
    /**
     * General options for GPT operations that can be scoped to a specific VRM.
     */
    export interface Options {
        /** Optional VRM entity ID to scope the operation to a specific character */
        vrm?: number;
    }

    /**
     * Options for chat operations including VRM targeting and voice synthesis.
     */
    export interface ChatOptions extends SpeakOnVoiceVoxOptions {
        /** VRM entity ID that will respond to the chat message */
        vrm: number;
    }

    /**
     * Response from a GPT chat interaction.
     */
    export interface ChatResponse {
        /** The raw message response from the AI model */
        message: string;
        /** Processed dialogue text suitable for speech synthesis */
        dialogue: string;
        /** Detected or assigned emotion for the response */
        emotion: string;
    }

    /**
     * Fetches the list of available GPT models from the configured providers.
     *
     * @returns A promise that resolves to an array of model names
     * @example
     * ```typescript
     * const models = await gpt.availableModels();
     * console.log("Available models:", models);
     * // Output: ["gpt-3.5-turbo", "gpt-4", "claude-3-sonnet"]
     * ```
     */
    export const availableModels = async (): Promise<string[]> => {
        const response = await host.get(host.createUrl("gpt/available-models"));
        return await response.json();
    }

    /**
     * Gets the current GPT model being used.
     *
     * @param options - Optional parameters to scope to a specific VRM
     * @returns A promise that resolves to the current model name
     * @example
     * ```typescript
     * // Get global model
     * const globalModel = await gpt.model();
     *
     * // Get model for specific VRM
     * const vrmModel = await gpt.model({ vrm: vrmEntity });
     * ```
     */
    export const model = async (
        options?: Options,
    ): Promise<string> => {
        const response = await host.get(host.createUrl("gpt/model", options));
        return await response.json();
    }

    /**
     * Saves the GPT model configuration for future use.
     *
     * @param model - The model name to set (must be from availableModels())
     * @param options - Optional parameters to scope to a specific VRM
     * @example
     * ```typescript
     * // Set global model
     * await gpt.saveModel("gpt-4");
     *
     * // Set model for specific VRM
     * await gpt.saveModel("claude-3-sonnet", { vrm: vrmEntity });
     * ```
     */
    export const saveModel = async (
        model: string,
        options?: Options,
    ) => {
        await host.put(host.createUrl("gpt/model"), {
            model,
            ...options,
        });
    }

    /**
     * Gets the current web search setting for GPT interactions.
     *
     * When enabled, the AI can search the web for current information to enhance responses.
     *
     * @param options - Optional parameters to scope to a specific VRM
     * @returns A promise that resolves to the current web search setting
     * @example
     * ```typescript
     * const webSearchEnabled = await gpt.useWebSearch();
     * console.log("Web search enabled:", webSearchEnabled);
     * ```
     */
    export const useWebSearch = async (
        options?: Options,
    ): Promise<boolean> => {
        const response = await host.get(host.createUrl("gpt/use-web-search", options));
        return await response.json();
    }

    /**
     * Saves the web search setting for GPT interactions.
     *
     * @param useWebSearch - Whether to enable web search capabilities
     * @param options - Parameters to scope to a specific VRM
     * @example
     * ```typescript
     * // Enable web search for a specific VRM
     * await gpt.saveUseWebSearch(true, { vrm: vrmEntity });
     * ```
     */
    export const saveUseWebSearch = async (
        useWebSearch: boolean,
        options: Options,
    ): Promise<void> => {
        await host.put(host.createUrl("gpt/use-web-search"), {
            useWebSearch,
            ...options,
        });
    }

    /**
     * Gets the current system prompt used to configure AI behavior.
     *
     * The system prompt defines the AI's personality, role, and behavioral guidelines.
     *
     * @param options - Optional parameters to scope to a specific VRM
     * @returns A promise that resolves to the current system prompt
     * @example
     * ```typescript
     * const prompt = await gpt.systemPrompt({ vrm: vrmEntity });
     * console.log("Current system prompt:", prompt);
     * ```
     */
    export const systemPrompt = async (options?: Options): Promise<string> => {
        const response = await host.get(host.createUrl("gpt/system-prompt", options));
        return await response.json();
    }

    /**
     * Saves a new system prompt to configure AI behavior.
     *
     * @param systemPrompt - The system prompt text that defines AI personality and behavior
     * @param options - Optional parameters to scope to a specific VRM
     * @example
     * ```typescript
     * await gpt.saveSystemPrompt(
     *   "You are a friendly assistant who loves to help with programming questions.",
     *   { vrm: vrmEntity }
     * );
     * ```
     */
    export const saveSystemPrompt = async (
        systemPrompt: string,
        options?: Options,
    ): Promise<void> => {
        await host.put(host.createUrl("gpt/system-prompt"), {
            systemPrompt,
            ...options,
        });
    }

    /**
     * Sends a chat message to the AI model and optionally makes a VRM speak the response.
     *
     * This is the primary method for interactive conversations with AI-powered VRM characters.
     *
     * @param userMessage - The message to send to the AI
     * @param options - Optional chat configuration including VRM and voice settings
     * @returns A promise that resolves to the AI's response
     * @example
     * ```typescript
     * // Simple chat without VRM
     * const response = await gpt.chat("What's the weather like?");
     *
     * // Chat with VRM character that will speak the response
     * const response = await gpt.chat("Tell me a joke!", {
     *   vrm: vrmEntity,
     *   speaker: 1,
     *   subtitle: {
     *     fontSize: 24,
     *     color: [1, 1, 1, 1]
     *   }
     * });
     *
     * console.log("AI Response:", response.message);
     * console.log("Emotion:", response.emotion);
     * ```
     */
    export const chat = async (
        userMessage: string,
        options?: ChatOptions,
    ): Promise<ChatResponse> => {
        const response = await host.post(host.createUrl("gpt/chat"), {
            userMessage,
            options,
        });
        return await response.json();
    }

    /**
     * Gets the current VoiceVox speaker ID used for text-to-speech synthesis.
     *
     * VoiceVox speakers represent different voice characters with unique tones and styles.
     *
     * @param options - Optional parameters to scope to a specific VRM
     * @returns A promise that resolves to the current speaker ID
     * @example
     * ```typescript
     * const speakerId = await gpt.voicevoxSpeaker({ vrm: vrmEntity });
     * console.log("Current VoiceVox speaker:", speakerId);
     * ```
     */
    export const voicevoxSpeaker = async (options?: Options): Promise<number> => {
        const response = await host.get(host.createUrl("gpt/speaker/voicevox", options));
        return await response.json();
    }

    /**
     * Saves the VoiceVox speaker ID for text-to-speech synthesis.
     *
     * @param id - The VoiceVox speaker ID to use (must be valid speaker from VoiceVox)
     * @param options - Optional parameters to scope to a specific VRM
     * @example
     * ```typescript
     * // Set speaker for specific VRM
     * await gpt.saveVoicevoxSpeaker(2, { vrm: vrmEntity });
     * ```
     */
    export const saveVoicevoxSpeaker = async (
        id: number,
        options?: Options,
    ): Promise<void> => {
        await host.put(host.createUrl("gpt/speaker/voicevox"), {
            id,
            ...options,
        });
    }
}