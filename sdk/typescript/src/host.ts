/**
 * Host API namespace for low-level HTTP communication with the Desktop Homunculus server.
 *
 * This module provides the foundational HTTP client functionality used internally
 * by all other SDK modules. It handles the base URL configuration, URL construction,
 * and HTTP methods with automatic error handling.
 *
 * **Note:** This module is primarily for internal SDK use. Most developers should
 * use the higher-level namespaces like `gpt`, `vrm`, `commands`, etc.
 *
 * @example
 * ```typescript
 * // Internal SDK usage (you typically won't need this directly)
 * const response = await host.get(host.createUrl("vrm/all"));
 * const vrms = await response.json();
 *
 * // URL construction with parameters
 * const url = host.createUrl("gpt/model", { vrm: 123 });
 * // Results in: http://localhost:3100/gpt/model?vrm=123
 * ```
 */
export namespace host {
    /** The base URL for the Desktop Homunculus HTTP server */
    export const base = "http://localhost:3100";

    /** Creates a new URL instance pointing to the base server */
    export const baseUrl = () => new URL("http://localhost:3100")

    /**
     * Creates a URL for the Desktop Homunculus API with optional query parameters.
     *
     * @param path - The API endpoint path (relative to base URL)
     * @param params - Optional query parameters to append to the URL
     * @returns A URL instance ready for use in HTTP requests
     *
     * @example
     * ```typescript
     * // Simple path
     * const url = host.createUrl("vrm/all");
     * // Result: http://localhost:3100/vrm/all
     *
     * // With query parameters
     * const url = host.createUrl("entities", { name: "VRM", root: 123 });
     * // Result: http://localhost:3100/entities?name=VRM&root=123
     * ```
     */
    export const createUrl = (path: string, params?: object): URL => {
        const url = new URL(path, base);
        if (params) {
            Object.entries(params).forEach(([key, value]) => {
                url.searchParams.append(key, String(value));
            });
        }
        return url;
    }

    /**
     * Performs a GET request to the specified URL with automatic error handling.
     *
     * @param url - The URL to send the GET request to
     * @returns The Response object if successful
     * @throws Will throw an error if the response is not ok (status >= 400)
     *
     * @example
     * ```typescript
     * const response = await host.get(host.createUrl("vrm/all"));
     * const data = await response.json();
     * ```
     */
    export async function get(url: URL): Promise<Response> {
        const response = await fetch(url);
        await throwIfError(response);
        return response;
    }

    /**
     * Performs a POST request with JSON payload and automatic error handling.
     *
     * @param url - The URL to send the POST request to
     * @param body - Optional request body that will be JSON-serialized
     * @returns The Response object if successful
     * @throws Will throw an error if the response is not ok (status >= 400)
     *
     * @example
     * ```typescript
     * const response = await host.post(
     *   host.createUrl("gpt/chat"),
     *   { userMessage: "Hello!", options: { vrm: 123 } }
     * );
     * const chatResponse = await response.json();
     * ```
     */
    export const post = async (url: URL, body?: any): Promise<Response> => {
        const response = await fetch(url, {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify(body ?? {})
        });
        await throwIfError(response);
        return response;
    }

    /**
     * Performs a PUT request with JSON payload and automatic error handling.
     *
     * @param url - The URL to send the PUT request to
     * @param body - Optional request body that will be JSON-serialized
     * @returns The Response object if successful
     * @throws Will throw an error if the response is not ok (status >= 400)
     *
     * @example
     * ```typescript
     * await host.put(
     *   host.createUrl("gpt/model"),
     *   { model: "gpt-4", vrm: 123 }
     * );
     * ```
     */
    export const put = async (url: URL, body?: any): Promise<Response> => {
        const response = await fetch(url, {
            method: "PUT",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify(body ?? {})
        });
        await throwIfError(response);
        return response;
    }

    export const deleteMethod = async (url: URL): Promise<Response> => {
        const response = await fetch(url, {
            method: "DELETE"
        });
        await throwIfError(response);
        return response;
    }
}

/**
 * Internal helper function that throws an error if the HTTP response indicates failure.
 *
 * @param response - The Response object to check
 * @throws Will throw a detailed error if response.ok is false
 */
const throwIfError = async (response: Response): Promise<void> => {
    if (!response.ok) {
        throw new Error(`url: ${response.url}\nStatus ${response.statusText}\n${await response.text()}`);
    }
}