use crate::error::ApiResult;
use crate::vrm::VrmApi;
use async_channel::Receiver;
use bevy::prelude::{Res, Update};
use bevy::tasks::IoTaskPool;
use bevy_flurx::prelude::once;
use homunculus_core::prelude::{OutputLog, VrmEventReceiver, VrmMetadata};

impl VrmApi {
    pub async fn observer(&self) -> ApiResult<Receiver<VrmMetadata>> {
        let (tx, rx) = async_channel::unbounded();
        let vrms = self.fetch_all().await?;
        let observer = self
            .0
            .schedule(|task| async move { task.will(Update, once::run(receiver)).await })
            .await?;
        for vrm in vrms {
            tx.send(vrm).await.output_log_if_error("LoadObserver");
        }
        self.observe_load(tx, observer);
        Ok(rx)
    }

    fn observe_load(
        &self,
        tx: async_channel::Sender<VrmMetadata>,
        mut receiver: VrmEventReceiver<VrmMetadata>,
    ) {
        IoTaskPool::get()
            .spawn(async move {
                while let Ok(vrm) = receiver.recv().await {
                    if tx.send(vrm.payload).await.is_err() {
                        return;
                    }
                }
            })
            .detach();
    }
}

fn receiver(rx: Res<VrmEventReceiver<VrmMetadata>>) -> VrmEventReceiver<VrmMetadata> {
    rx.clone()
}
