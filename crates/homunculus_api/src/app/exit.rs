use crate::app::AppApi;
use crate::error::ApiResult;
use bevy::prelude::*;
use bevy_flurx::prelude::*;

impl AppApi {
    /// Exists the application without any problems.
    pub async fn exit(&self) -> ApiResult {
        self.0
            .schedule(|task| async move {
                task.will(Update, once::run(exit_app)).await;
            })
            .await
    }
}

fn exit_app(mut ew: EventWriter<AppExit>) {
    ew.write(AppExit::Success);
}
