use bevy::asset::io::file::FileAssetReader;
use bevy::prelude::*;
use bevy::tasks::IoTaskPool;
use deno_resolver::npm::{DenoInNpmPackageChecker, NpmResolver};
use deno_runtime::deno_core::error::CoreError;
use deno_runtime::deno_core::{
    FsModuleLoader, JsRuntime, ModuleCodeString, ModuleSpecifier, PollEventLoopOptions,
};
use deno_runtime::deno_fs::RealFs;
use deno_runtime::deno_permissions::PermissionsContainer;
use deno_runtime::permissions::RuntimePermissionDescriptorParser;
use deno_runtime::worker::{MainWorker, WorkerOptions, WorkerServiceOptions};
use deno_runtime::{BootstrapOptions, FeatureChecker, UNSTABLE_FEATURES};
use homunculus_core::prelude::OutputLog;
use std::path::PathBuf;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;
use std::task::{Context, Poll};

/// Request to the Deno runtime to perform actions like calling scripts or advancing the event loop.
#[derive(Debug, Event, Clone, PartialEq, Eq, Hash)]
pub(crate) enum RequestDeno {
    /// Request to call a script.
    CallScript {
        name: Option<&'static str>,
        content: String,
    },

    /// Request to advance the event loop.
    /// Normally, please request it every frame.
    TickEventLoop,
}

#[derive(Resource, Deref, DerefMut)]
pub(crate) struct RequestSender(async_channel::Sender<RequestDeno>);

#[derive(Resource)]
struct RequestReceiver(async_channel::Receiver<RequestDeno>);

pub(super) struct DenoRuntimePlugin;

impl Plugin for DenoRuntimePlugin {
    fn build(&self, app: &mut App) {
        let (sender, receiver) = async_channel::unbounded();
        app.insert_resource(RequestSender(sender))
            .insert_resource(RequestReceiver(receiver))
            .add_systems(Startup, setup_deno_runtime);
    }
}

fn setup_deno_runtime(rx: Res<RequestReceiver>) {
    let rx = rx.0.clone();
    let base_path = FileAssetReader::get_base_path();
    let main_module_path = base_path.join("assets").join("scripts").join("denoMain.js");
    IoTaskPool::get()
        .spawn(async move {
            let rt = match tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
            {
                Ok(rt) => rt,
                Err(e) => {
                    error!("Failed to create Tokio runtime: {e}");
                    return;
                }
            };
            rt.block_on(async move {
                if let Err(e) = setup_main_worker(main_module_path, rx).await {
                    error!("Failed to run script: {e}");
                }
            });
        })
        .detach();
}

async fn setup_main_worker(
    main_module_path: PathBuf,
    rx: async_channel::Receiver<RequestDeno>,
) -> Result<MainWorker, CoreError> {
    let main_module = ModuleSpecifier::from_file_path(main_module_path).unwrap();
    let mut worker = create_main_worker(&main_module);
    worker.execute_main_module(&main_module).await?;
    worker.run_event_loop(false).await?;
    while let Ok(request) = rx.recv().await {
        match request {
            RequestDeno::CallScript { name, content } => {
                info!("Calling script: {name:?}");
                worker
                    .execute_script(name.unwrap_or(""), ModuleCodeString::from(content))
                    .output_log_if_error("DenoRuntime");
            }
            RequestDeno::TickEventLoop => {
                EventLoopFuture {
                    runtime: &mut worker.js_runtime,
                }
                .await
                .output_log_if_error("DenoRuntime");
            }
        }
    }
    Ok(worker)
}

fn create_main_worker(main_module: &ModuleSpecifier) -> MainWorker {
    let permission_desc_parser = Arc::new(RuntimePermissionDescriptorParser::new(
        sys_traits::impls::RealSys,
    ));
    let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
    let mut feature_checker = FeatureChecker::default();
    feature_checker.enable_feature(deno_runtime::deno_cron::UNSTABLE_FEATURE_NAME);
    let fs = Arc::new(RealFs);
    MainWorker::bootstrap_from_options(
        main_module,
        WorkerServiceOptions::<
            DenoInNpmPackageChecker,
            NpmResolver<sys_traits::impls::RealSys>,
            sys_traits::impls::RealSys,
        > {
            module_loader: Rc::new(FsModuleLoader),
            permissions: PermissionsContainer::allow_all(permission_desc_parser),
            blob_store: Default::default(),
            broadcast_channel: Default::default(),
            feature_checker: Arc::new(feature_checker),
            node_services: Default::default(),
            npm_process_state_provider: Default::default(),
            root_cert_store_provider: Default::default(),
            fetch_dns_resolver: Default::default(),
            shared_array_buffer_store: Default::default(),
            compiled_wasm_module_store: Default::default(),
            v8_code_cache: Default::default(),
            fs,
            deno_rt_native_addon_loader: Default::default(),
        },
        WorkerOptions {
            bootstrap: BootstrapOptions {
                unstable_features: unstable_feature_flags(),
                ..default()
            },
            // startup_snapshot: HOMUNCULUS_SNAPSHOT,
            ..Default::default()
        },
    )
}

fn unstable_feature_flags() -> Vec<i32> {
    UNSTABLE_FEATURES.iter().map(|f| f.id).collect()
}

struct EventLoopFuture<'w> {
    runtime: &'w mut JsRuntime,
}

impl Future for EventLoopFuture<'_> {
    type Output = Result<(), CoreError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let p = self.get_mut().runtime.poll_event_loop(
            cx,
            PollEventLoopOptions {
                wait_for_inspector: true,
                pump_v8_message_loop: true,
            },
        );
        match p {
            Poll::Ready(Err(e)) => Poll::Ready(Err(e)),
            _ => Poll::Ready(Ok(())),
        }
    }
}
