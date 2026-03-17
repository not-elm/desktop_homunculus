//! MCP resource definitions and read logic.

use homunculus_api::assets::AssetFilter;
use rmcp::model::{
    AnnotateAble, RawResource, ReadResourceRequestParams, ReadResourceResult, Resource,
    ResourceContents,
};

use super::{FEATURES, HomunculusMcpHandler, api_err, to_json_string};

/// Returns the list of resources exposed by this MCP server.
pub(super) fn resource_definitions() -> Vec<Resource> {
    vec![
        RawResource::new("homunculus://info", "homunculus-info")
            .with_description("Application info including version, platform, features, and mods")
            .with_mime_type("application/json")
            .no_annotation(),
        RawResource::new("homunculus://characters", "homunculus-characters")
            .with_description("Detailed snapshot of all loaded VRM characters")
            .with_mime_type("application/json")
            .no_annotation(),
        RawResource::new("homunculus://mods", "homunculus-mods")
            .with_description("List of installed mods")
            .with_mime_type("application/json")
            .no_annotation(),
        RawResource::new("homunculus://assets", "homunculus-assets")
            .with_description("List of available assets across all mods")
            .with_mime_type("application/json")
            .no_annotation(),
        RawResource::new("homunculus://rpc", "homunculus-rpc")
            .with_description("Registered RPC methods by MOD service. Use with call_rpc tool.")
            .with_mime_type("application/json")
            .no_annotation(),
    ]
}

/// Reads a single resource by URI and returns its JSON content.
pub(super) async fn read_resource(
    handler: &HomunculusMcpHandler,
    request: ReadResourceRequestParams,
) -> Result<ReadResourceResult, rmcp::ErrorData> {
    let uri = &request.uri;
    let json_text = match uri.as_str() {
        "homunculus://info" => {
            let mod_list = handler.mods_api.list().await.map_err(api_err)?;

            let info = serde_json::json!({
                "version": env!("CARGO_PKG_VERSION"),
                "platform": {
                    "os": std::env::consts::OS,
                    "arch": std::env::consts::ARCH,
                },
                "features": FEATURES,
                "mods": mod_list,
            });
            to_json_string(&info)?
        }
        "homunculus://characters" => {
            let snapshots = handler.vrm_api.snapshot().await.map_err(api_err)?;
            to_json_string(&snapshots)?
        }
        "homunculus://mods" => {
            let mods = handler.mods_api.list().await.map_err(api_err)?;
            to_json_string(&mods)?
        }
        "homunculus://assets" => {
            let assets = handler
                .assets_api
                .list(AssetFilter::default())
                .await
                .map_err(api_err)?;
            to_json_string(&assets)?
        }
        "homunculus://rpc" => {
            let reg = handler
                .rpc_registry
                .read()
                .unwrap_or_else(|e| e.into_inner());
            to_json_string(reg.all())?
        }
        _ => {
            return Err(rmcp::ErrorData::resource_not_found(
                format!("Unknown resource: {uri}"),
                None,
            ));
        }
    };

    Ok(ReadResourceResult::new(vec![
        ResourceContents::text(json_text, uri.clone()).with_mime_type("application/json"),
    ]))
}
