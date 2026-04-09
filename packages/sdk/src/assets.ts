import {host} from "./host";

/**
 * Assets API namespace for querying available mod assets.
 *
 * Provides access to the asset registry, which contains all assets declared
 * by installed mods. Assets are referenced by their globally unique ID using
 * the format `"mod-name:asset-name"` (e.g., `"elmer:idle"`, `"my-mod:click"`).
 *
 * @example
 * ```typescript
 * // List all available assets
 * const all = await assets.list();
 *
 * // Filter by type
 * const vrms = await assets.list({ type: "vrm" });
 *
 * // Filter by mod
 * const elmerAssets = await assets.list({ mod: "elmer" });
 * ```
 */
export namespace assets {
    /** The type of an asset. */
    export type AssetType = "vrm" | "vrma" | "sound" | "image" | "html";

    /** Information about a registered asset. */
    export interface AssetInfo {
        /** Globally unique asset ID. */
        id: string;
        /** The asset type. */
        type: AssetType;
        /** The mod that provides this asset. */
        mod: string;
        /** Optional description of the asset. */
        description?: string;
    }

    /** Filter options for listing assets. */
    export interface AssetFilter {
        /** Filter by asset type. */
        type?: AssetType;
        /** Filter by mod name. */
        mod?: string;
    }

    /**
     * Lists available assets, optionally filtered by type and/or mod.
     *
     * @param filter - Optional filter criteria
     * @returns Array of matching asset info objects
     *
     * @example
     * ```typescript
     * // Get all assets
     * const all = await assets.list();
     *
     * // Get only VRM models
     * const vrms = await assets.list({ type: "vrm" });
     *
     * // Get assets from a specific mod
     * const modAssets = await assets.list({ mod: "elmer" });
     *
     * // Combine filters
     * const sounds = await assets.list({ type: "sound", mod: "my-mod" });
     * ```
     */
    export async function list(filter?: AssetFilter): Promise<AssetInfo[]> {
        const response = await host.get(host.createUrl("assets", filter));
        return await response.json() as AssetInfo[];
    }

    /** Parameters for importing an asset file. */
    export interface ImportAssetParams {
        /** Absolute path to the source file */
        sourcePath: string;
        /** Asset ID to register (e.g., "vrm:local:my-persona") */
        assetId: string;
        /** Asset type */
        assetType: AssetType;
        /** Optional description */
        description?: string;
    }

    /** Result of an asset import. */
    export interface ImportAssetResult {
        /** The registered asset ID */
        assetId: string;
    }

    /**
     * Imports a file as a managed asset.
     *
     * Copies the source file to the managed assets directory, registers it in the
     * asset registry, and persists the registration to the database.
     *
     * @returns The registered asset info
     *
     * @example
     * ```typescript
     * const result = await assets.importAsset({
     *   sourcePath: "/Users/me/Downloads/model.vrm",
     *   assetId: "vrm:local:alice",
     *   assetType: "vrm",
     *   description: "Alice's custom model",
     * });
     * console.log("Imported:", result.assetId);
     * ```
     */
    export async function importAsset(params: ImportAssetParams): Promise<ImportAssetResult> {
        const response = await host.post(host.createUrl("assets/import"), params);
        return await response.json() as ImportAssetResult;
    }
}
