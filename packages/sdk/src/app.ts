import {host} from "./host";

/**
 * Provides access to the application API.
 */
export namespace app {
    /**
     * Exits the application without any problems.
     */
    export async function exit() {
        await host.post(host.createUrl("app/exit"));
    }

    /**
     * Checks if the Desktop Homunculus server is running and healthy.
     *
     * Returns `true` if the server responds with a successful health check,
     * `false` if the server is unreachable or unhealthy.
     *
     * @example
     * ```typescript
     * const alive = await app.health();
     * if (!alive) {
     *   console.error("Homunculus server is not running");
     * }
     * ```
     */
    export async function health(): Promise<boolean> {
        try {
            const response = await fetch(host.createUrl("app/health"));
            return response.ok;
        } catch {
            return false;
        }
    }

    /**
     * Platform information about the running system.
     */
    export interface PlatformInfo {
        /** Operating system name (e.g., "macos", "windows", "linux"). */
        os: string;
        /** CPU architecture (e.g., "aarch64", "x86_64"). */
        arch: string;
    }

    /**
     * Summary of a loaded mod as returned by the info endpoint.
     */
    export interface InfoMod {
        /** The mod package name. */
        name: string;
        /** The mod package version. */
        version: string;
        /** Human-readable description. */
        description: string | null;
        /** The mod author. */
        author: string | null;
        /** The mod license. */
        license: string | null;
        /** Whether the mod has a running main process. */
        hasMain: boolean;
        /** Available MOD command names. */
        binCommands: string[];
        /** Registered asset IDs. */
        assetIds: string[];
    }

    /**
     * Application metadata returned by the info endpoint.
     */
    export interface AppInfo {
        /** The engine version string (e.g., "0.1.0-alpha.3.2"). */
        version: string;
        /** Platform information. */
        platform: PlatformInfo;
        /** Engine-level features available in this build. */
        features: string[];
        /** All loaded mods with metadata. */
        mods: InfoMod[];
    }

    /**
     * Returns metadata about the running Desktop Homunculus instance.
     *
     * Provides the engine version, platform info, compiled features,
     * and loaded mods in a single request. Useful for startup checks,
     * feature detection, and status displays.
     *
     * @returns Application info including version, platform, features, and mods
     *
     * @example
     * ```typescript
     * const info = await app.info();
     * console.log(`Engine v${info.version} on ${info.platform.os}/${info.platform.arch}`);
     * console.log(`Features: ${info.features.join(", ")}`);
     * console.log(`${info.mods.length} mods loaded`);
     * ```
     */
    export async function info(): Promise<AppInfo> {
        const response = await host.get(host.createUrl("app/info"));
        return await response.json() as AppInfo;
    }
}
