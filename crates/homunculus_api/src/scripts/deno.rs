use crate::error::ApiResult;
use crate::scripts::ScriptsApi;
use bevy::prelude::*;
use bevy_flurx::action::once;
use homunculus_core::prelude::ModModuleSource;
use homunculus_deno::prelude::DenoScriptHandle;

impl ScriptsApi {
    /// Calls a Javascript(or Typescript) file located in `assets/mods`.
    ///
    /// The script will be called on the built-in Deno runtime.
    ///
    /// When the Javascript file being executed is changed, a hot reload will occur, and it will be re-executed.
    pub async fn call_javascript(&self, source: ModModuleSource) -> ApiResult {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(call_javascript_on_deno).with(source))
                    .await
            })
            .await?
    }
}

fn call_javascript_on_deno(
    In(id): In<ModModuleSource>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) -> ApiResult {
    let path = id.to_string();
    commands.spawn(DenoScriptHandle(asset_server.load(path)));
    Ok(())
}
