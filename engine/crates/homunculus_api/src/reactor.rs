use crate::error::ApiResult;
use async_channel::{Receiver, Sender};
use bevy::prelude::*;
use bevy_flurx::prelude::*;
use std::any::type_name;
use std::pin::Pin;

pub type BoxedTask =
    Box<dyn FnOnce(ReactorTask) -> Pin<Box<dyn Future<Output = ()> + Send + Sync>> + Send + Sync>;

#[derive(Clone, Deref, Resource)]
pub struct TaskReceiver(pub Receiver<BoxedTask>);

#[derive(Clone, Resource, Debug)]
pub struct ApiReactor(Sender<BoxedTask>);

impl ApiReactor {
    pub async fn schedule<F, Fut, O>(&self, f: F) -> ApiResult<O>
    where
        F: FnOnce(ReactorTask) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = O> + Send + Sync + 'static,
        O: Send + Sync + 'static,
    {
        let (tx, rx) = async_channel::unbounded::<O>();
        self.0
            .send(Box::new(move |task| {
                Box::pin(async move {
                    let out = f(task).await;
                    if let Err(e) = tx.send(out).await {
                        error!("Failed to send task result type: {}\n{e}", type_name::<O>());
                    }
                })
            }))
            .await?;
        Ok(rx.recv().await?)
    }
}

pub(crate) struct ApiReactorPlugin;

impl Plugin for ApiReactorPlugin {
    fn build(&self, app: &mut App) {
        let (tx, rx) = async_channel::unbounded::<BoxedTask>();
        let reactor = ApiReactor(tx);
        app.insert_resource(TaskReceiver(rx))
            .insert_resource(reactor)
            .add_systems(First, schedule_reactor);
    }
}

fn schedule_reactor(world: &mut World) {
    let rx = world.resource::<TaskReceiver>().clone();
    while let Ok(f) = rx.try_recv() {
        world.spawn(Reactor::schedule(move |task| async move {
            f(task).await;
        }));
    }
    // Flush deferred commands so the last `step_reactor` (queued by
    // `NativeReactor::on_add` during `world.spawn()`) is processed in
    // this frame.  Without this, exclusive-system `apply_deferred` is a
    // no-op and the command stays in World's internal queue indefinitely,
    // causing the first HTTP request after an idle period to hang.
    world.flush();
}
