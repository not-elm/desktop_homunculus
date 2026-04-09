import { host } from "./host";
import { type Vec2, type Vec3 } from "./math";
import { Persona } from "./persona";

// --- Z-layer constants ---

/**
 * Z-layer constants for webview positioning.
 *
 * The scene uses an orthographic camera at z=20. Objects with higher z
 * values appear in front. VRM characters are at z~0, default webviews at z=10.
 *
 * @example
 * ```typescript
 * await Webview.open({
 *   source: webviewSource.local("my-mod:ui"),
 *   offset: [0, 0, WebviewLayer.FOREGROUND],
 * });
 * ```
 */
export const WebviewLayer = {
    /** Default webview layer (z=0). Standard depth, behind VRM characters. */
    DEFAULT: 0,
    /** Foreground layer (z=15). Renders in front of VRM characters and other webviews. */
    FOREGROUND: 15,
} as const;

// --- Webview types ---

/**
 * Represets a local webview source.
 */
export interface WebviewSourceLocal {
    type: "local";
    id: string;
}

/**
 * Represents a inline-html webview source.
 */
export interface WebviewSourceHtml {
    type: "html";
    content: string;
}

/**
 * Represents a remott webview source.
 */
export interface WebviewSourceUrl {
    type: "url";
    url: string;
}

/**
 * Webview source specification (request): URL/path, inline HTML, or local asset ID.
 *
 * @example
 * ```typescript
 * // URL or mod asset path
 * const urlSource: WebviewSource = webviewSource.url("my-mod::ui.html");
 * // Inline HTML content
 * const htmlSource: WebviewSource = webviewSource.html("<h1>Hello</h1>");
 * // Local asset by ID
 * const localSource: WebviewSource = webviewSource.local("my-mod::panel.html");
 * ```
 */
export type WebviewSource =
    | WebviewSourceUrl
    | WebviewSourceHtml
    | WebviewSourceLocal;

/**
 * Returns whether webview source is local.
 */
export function isWebviewSourceLocal(source: WebviewSource): source is WebviewSourceLocal {
    return source.type === "local";
}

/**
 * Returns whether webview source is url.
 */
export function isWebviewSourceUrl(source: WebviewSource): source is WebviewSourceUrl {
    return source.type === "url";
}

/**
 * Returns whether webview source is inline-html.
 */
export function isWebviewSourceHtml(source: WebviewSource): source is WebviewSourceHtml {
    return source.type === "html";
}

/**
 * Factory functions for creating {@link WebviewSource} objects.
 *
 * @example
 * ```typescript
 * import { Webview, webviewSource } from "@hmcs/sdk";
 *
 * await Webview.open({ source: webviewSource.local("menu:ui") });
 * await Webview.open({ source: webviewSource.url("https://example.com") });
 * await wv.navigate(webviewSource.html("<h1>Hello</h1>"));
 * ```
 */
export namespace webviewSource {
    /**
     * Create a local asset source.
     *
     * @param id - Asset ID (e.g., `"menu:ui"`, `"settings:ui"`)
     *
     * @example
     * ```typescript
     * const source = webviewSource.local("menu:ui");
     * // { type: "local", id: "menu:ui" }
     * ```
     */
    export function local(id: string): WebviewSourceLocal {
        return { type: "local", id };
    }

    /**
     * Create a URL source.
     *
     * @param url - URL string
     *
     * @example
     * ```typescript
     * const source = webviewSource.url("https://example.com");
     * // { type: "url", url: "https://example.com" }
     * ```
     */
    export function url(url: string): WebviewSourceUrl {
        return { type: "url", url };
    }

    /**
     * Create an inline HTML source.
     *
     * @param content - HTML string
     *
     * @example
     * ```typescript
     * const source = webviewSource.html("<h1>Hello</h1>");
     * // { type: "html", content: "<h1>Hello</h1>" }
     * ```
     */
    export function html(content: string): WebviewSourceHtml {
        return { type: "html", content };
    }
}

/**
 * Represents a url webview source info
 */
export interface WebviewSourceInfoUrl {
    type: "url";
    url: string;
}

/**
 * Represents a local webview source info.
 */
export interface WebviewSourceInfoLocal {
    type: "local";
    id: string;
}

/**
 * Repsents a inline-html source info.
 */
export interface WebviewSourceInfoHtml {
    type: "html";
    content?: string;
}

/**
 * Webview source information (response).
 * In list responses, Html content is omitted.
 * In detail responses, Html content is included.
 */
export type WebviewSourceInfo =
    | WebviewSourceInfoHtml
    | WebviewSourceInfoLocal
    | WebviewSourceInfoUrl;

export function isWebviewSourceInfoLocal(source: WebviewSourceInfo): source is WebviewSourceInfoLocal {
    return source.type === "local";
}

export function isWebviewSourceInfoUrl(source: WebviewSourceInfo): source is WebviewSourceInfoUrl {
    return source.type === "url";
}

export function isWebviewSourceInfoHtml(source: WebviewSourceInfoHtml): source is WebviewSourceInfoHtml {
    return source.type === "html";
}

/** Information about a webview instance. */
export interface WebviewInfo {
    entity: number;
    source: WebviewSourceInfo;
    size: Vec2;
    viewportSize: Vec2;
    offset: Vec2 | Vec3;
    /** The persona ID linked to this webview, or null/undefined if none. */
    linkedPersona?: string | null;
}

/**
 * Options for opening a webview.
 *
 * @example
 * ```typescript
 * const options: WebviewOpenOptions = {
 *   source: webviewSource.url("my-mod::ui.html"),
 *   size: [0.7, 0.7],
 *   viewportSize: [800, 600],
 *   offset: [0, 0.5],
 * };
 * ```
 */
export interface WebviewOpenOptions {
    source: WebviewSource;
    size?: Vec2;
    viewportSize?: Vec2;
    offset?: Vec2 | Vec3;
    /** The persona ID to link to this webview. */
    linkedPersona?: string;
}

/** Request body for patching webview properties. */
export interface WebviewPatchRequest {
    offset?: Vec2 | Vec3;
    size?: Vec2;
    viewportSize?: Vec2;
}

/** Request body for navigating a webview to a new source. */
export interface WebviewNavigateRequest {
    source: WebviewSource;
}

/**
 * Webview management for creating and controlling embedded web interfaces.
 *
 * Desktop Homunculus uses webviews to provide rich UI experiences that can be
 * positioned anywhere in 3D space or attached to VRM characters.
 *
 * @example
 * ```typescript
 * const webview = await Webview.open({
 *   source: webviewSource.url("my-mod::ui.html"),
 *   size: [0.7, 0.7],
 *   viewportSize: [800, 600],
 *   offset: [0, 0.5],
 * });
 *
 * if (!(await webview.isClosed())) {
 *   await webview.close();
 * }
 * ```
 */

/**
 * Represents a webview instance that can display HTML content in 3D space.
 */
export class Webview {
    constructor(readonly entity: number) {
        this.entity = entity;
    }

    /**
     * Closes the webview.
     */
    async close(): Promise<void> {
        await host.deleteMethod(host.createUrl(`webviews/${this.entity}`));
    }

    /**
     * Checks whether this webview has been closed.
     *
     * @returns A promise that resolves to true if the webview is closed
     */
    async isClosed(): Promise<boolean> {
        const response = await host.get(host.createUrl(`webviews/${this.entity}/is-closed`));
        return await response.json();
    }

    /**
     * Gets information about this webview.
     *
     * @returns A promise that resolves to the webview info
     */
    async info(): Promise<WebviewInfo> {
        const response = await host.get(host.createUrl(`webviews/${this.entity}`));
        return await response.json() as WebviewInfo;
    }

    /**
     * Patches webview properties (offset, size, viewportSize).
     *
     * @param options - The properties to update
     */
    async patch(options: WebviewPatchRequest): Promise<void> {
        await host.patch(host.createUrl(`webviews/${this.entity}`), options);
    }

    /**
     * Sets the offset of the webview.
     *
     * @param offset - The new offset
     */
    async setOffset(offset: Vec2 | Vec3): Promise<void> {
        await this.patch({ offset });
    }

    /**
     * Sets the size of the webview.
     *
     * @param size - The new size
     */
    async setSize(size: Vec2): Promise<void> {
        await this.patch({ size });
    }

    /**
     * Sets the viewport size of the webview.
     *
     * @param size - The new viewport size
     */
    async setViewportSize(size: Vec2): Promise<void> {
        await this.patch({ viewportSize: size });
    }

    /**
     * Navigates the webview to a new source.
     *
     * @param source - The new source (URL/path, inline HTML, or local asset ID)
     *
     * @example
     * ```typescript
     * const wv = new Webview(entity);
     * // Navigate to a mod asset
     * await wv.navigate(webviewSource.url("my-mod::page.html"));
     * // Navigate to inline HTML
     * await wv.navigate(webviewSource.html("<h1>Hello</h1>"));
     * // Navigate to a local asset by ID
     * await wv.navigate(webviewSource.local("my-mod::panel.html"));
     * ```
     */
    async navigate(source: WebviewSource): Promise<void> {
        await host.post(host.createUrl(`webviews/${this.entity}/navigate`), { source });
    }

    /**
     * Reloads the webview content.
     */
    async reload(): Promise<void> {
        await host.post(host.createUrl(`webviews/${this.entity}/reload`));
    }

    /**
     * Navigates the webview back in history.
     *
     * @example
     * ```typescript
     * const wv = (await Webview.list())[0];
     * await new Webview(wv.entity).navigateBack();
     * ```
     */
    async navigateBack(): Promise<void> {
        await host.post(host.createUrl(`webviews/${this.entity}/navigate/back`));
    }

    /**
     * Navigates the webview forward in history.
     *
     * @example
     * ```typescript
     * const wv = (await Webview.list())[0];
     * await new Webview(wv.entity).navigateForward();
     * ```
     */
    async navigateForward(): Promise<void> {
        await host.post(host.createUrl(`webviews/${this.entity}/navigate/forward`));
    }

    /**
     * Gets the persona linked to this webview.
     *
     * @returns The linked Persona instance, or undefined if no persona is linked
     *
     * @example
     * ```typescript
     * const wv = new Webview(entity);
     * const p = await wv.linkedPersona();
     * if (p) {
     *   console.log(`Linked to persona: ${p.id}`);
     * }
     * ```
     */
    async linkedPersona(): Promise<Persona | undefined> {
        const response = await host.get(
            host.createUrl(`webviews/${this.entity}/linked-persona`)
        );
        const id = await response.json();
        return id !== null ? new Persona(id as string) : undefined;
    }

    /**
     * Links this webview to a persona.
     *
     * @param personaId - The persona ID to link
     *
     * @example
     * ```typescript
     * const wv = new Webview(entity);
     * await wv.setLinkedPersona("alice");
     * ```
     */
    async setLinkedPersona(personaId: string): Promise<void> {
        await host.put(
            host.createUrl(`webviews/${this.entity}/linked-persona`),
            { personaId }
        );
    }

    /**
     * Removes the persona link from this webview.
     *
     * @example
     * ```typescript
     * const wv = new Webview(entity);
     * await wv.unlinkPersona();
     * ```
     */
    async unlinkPersona(): Promise<void> {
        await host.deleteMethod(
            host.createUrl(`webviews/${this.entity}/linked-persona`)
        );
    }

    /**
     * Gets all open webviews.
     *
     * @returns A promise that resolves to an array of webview info
     */
    static async list(): Promise<WebviewInfo[]> {
        const response = await host.get(host.createUrl("webviews"));
        return await response.json() as WebviewInfo[];
    }

    /**
     * Creates and opens a webview positioned in world space.
     *
     * @param options - Configuration for the webview
     * @returns A promise that resolves to a new Webview instance
     *
     * @example
     * ```typescript
     * // Open with a mod asset URL
     * const panel = await Webview.open({
     *   source: webviewSource.url("my-mod::settings.html"),
     *   size: [0.7, 0.5],
     *   viewportSize: [800, 600],
     *   offset: [0, 1.0],
     * });
     *
     * // Open with inline HTML
     * const inline = await Webview.open({
     *   source: webviewSource.html("<h1>Hello World</h1>"),
     * });
     *
     * // Open with a local asset
     * const local = await Webview.open({
     *   source: webviewSource.local("my-mod::panel.html"),
     *   offset: [0.5, 0],
     * });
     * ```
     */
    static async open(options: WebviewOpenOptions): Promise<Webview> {
        const response = await host.post(host.createUrl(`webviews`), options);
        return new Webview(Number(await response.json()));
    }

    /**
     * Gets the current webview instance if called from within a webview context.
     *
     * @returns The current Webview instance, or undefined if not in a webview context
     */
    static current(): Webview | undefined {
        // @ts-expect-error -- CEF injects WEBVIEW_ENTITY on the window object
        const entity: number | undefined = window.WEBVIEW_ENTITY;
        return entity !== undefined ? new Webview(entity) : undefined;
    }
}
