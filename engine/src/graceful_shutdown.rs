//! Graceful shutdown plugin that bridges OS termination signals into Bevy's
//! `AppExit` message system.
//!
//! # Architecture
//!
//! This plugin provides a 2-layer defense:
//! - **Layer 1 (macOS)**: NSApplicationDelegate intercepts Dock Quit / Cmd+Q
//! - **Layer 2 (Unix)**: `signal-hook` intercepts SIGTERM / SIGINT
//!
//! Both layers set a shared `AtomicBool` flag. A `First`-schedule system
//! polls this flag and writes `AppExit` via `MessageWriter`, triggering all
//! existing `on_message::<AppExit>` cleanup systems in the same frame.

use bevy::prelude::*;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

/// Shared flag set by platform-specific termination handlers.
#[derive(Resource, Clone)]
pub struct ShutdownFlag(pub Arc<AtomicBool>);

/// Plugin that converts OS termination requests into Bevy `AppExit` messages.
pub struct GracefulShutdownPlugin;

impl Plugin for GracefulShutdownPlugin {
    fn build(&self, app: &mut App) {
        let flag = Arc::new(AtomicBool::new(false));
        app.insert_resource(ShutdownFlag(flag.clone()));

        // Layer 2: Unix signal handling (SIGTERM, SIGINT)
        #[cfg(unix)]
        register_signal_handlers(&flag);

        // Layer 1: macOS NSApplicationDelegate
        #[cfg(target_os = "macos")]
        register_macos_delegate(&flag);

        app.add_systems(First, check_shutdown_flag);
    }
}

/// Polls the shared shutdown flag and writes `AppExit` when set.
fn check_shutdown_flag(flag: Res<ShutdownFlag>, mut ew: MessageWriter<AppExit>) {
    if flag.0.load(Ordering::Relaxed) {
        info!("Shutdown flag detected, sending AppExit");
        ew.write(AppExit::Success);
    }
}

/// Registers SIGTERM and SIGINT handlers that set the shared `AtomicBool`.
///
/// A second signal forces immediate process exit (failsafe against hung cleanup).
#[cfg(unix)]
fn register_signal_handlers(flag: &Arc<AtomicBool>) {
    use signal_hook::consts::{SIGINT, SIGTERM};
    use signal_hook::flag;

    for sig in [SIGTERM, SIGINT] {
        // First signal: set the flag for graceful shutdown
        let _ = flag::register(sig, Arc::clone(flag));
        // Second signal: force exit (failsafe against hung cleanup)
        let _ = flag::register_conditional_shutdown(sig, 1, Arc::clone(flag));
    }
}

/// Registers an NSApplicationDelegate that intercepts Dock Quit / Cmd+Q / Menu Quit.
///
/// Returns `NSTerminateCancel` on first request (sets `AtomicBool` for Bevy to
/// handle graceful exit), `NSTerminateNow` on second request (failsafe).
#[cfg(target_os = "macos")]
fn register_macos_delegate(flag: &Arc<AtomicBool>) {
    use objc2::rc::Retained;
    use objc2::{MainThreadMarker, MainThreadOnly, define_class, msg_send};
    use objc2_app_kit::{NSApp, NSApplication, NSApplicationDelegate, NSApplicationTerminateReply};
    use objc2_foundation::{NSObject, NSObjectProtocol};
    use std::cell::Cell;

    // Store the flag in a thread-local so the delegate callback can access it.
    // The delegate is only called on the main thread, so thread_local is safe.
    thread_local! {
        static SHUTDOWN_FLAG: Cell<Option<Arc<AtomicBool>>> = const { Cell::new(None) };
    }

    SHUTDOWN_FLAG.set(Some(Arc::clone(flag)));

    define_class!(
        #[unsafe(super(NSObject))]
        #[thread_kind = MainThreadOnly]
        #[name = "HomunculusAppDelegate"]
        struct HomunculusAppDelegate;

        unsafe impl NSObjectProtocol for HomunculusAppDelegate {}

        unsafe impl NSApplicationDelegate for HomunculusAppDelegate {
            #[unsafe(method(applicationShouldTerminate:))]
            #[allow(non_snake_case)]
            fn applicationShouldTerminate(
                &self,
                _sender: &NSApplication,
            ) -> NSApplicationTerminateReply {
                let already_shutting_down = SHUTDOWN_FLAG.with(|f| {
                    f.take().map_or(false, |shutdown_flag| {
                        let was_set = shutdown_flag.swap(true, Ordering::SeqCst);
                        f.set(Some(shutdown_flag));
                        was_set
                    })
                });

                if already_shutting_down {
                    // 2nd terminate request — force exit (failsafe)
                    info!("Second terminate request — forcing exit");
                    NSApplicationTerminateReply::TerminateNow
                } else {
                    // 1st request — cancel termination, let Bevy handle graceful exit
                    info!("Dock/Cmd+Q quit intercepted — initiating graceful shutdown");
                    NSApplicationTerminateReply::TerminateCancel
                }
            }
        }
    );

    let Some(mtm) = MainThreadMarker::new() else {
        warn!("GracefulShutdownPlugin: not on main thread, skipping NSApplicationDelegate");
        return;
    };

    // Create the delegate: alloc + init via msg_send
    let alloc = HomunculusAppDelegate::alloc(mtm);
    let delegate: Retained<HomunculusAppDelegate> =
        unsafe { msg_send![alloc, init] };

    // Register the delegate via msg_send to avoid NSObjectProtocol trait bound issues
    let app = NSApp(mtm);
    let () = unsafe { msg_send![&app, setDelegate: &*delegate] };

    // CRITICAL: NSApplication.delegate is a WEAK property.
    // We must keep a strong Retained<> reference alive for the app's lifetime.
    // Leak the Retained — the delegate must outlive the application.
    std::mem::forget(delegate);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shutdown_flag_triggers_app_exit() {
        let mut app = App::new();
        app.add_plugins(GracefulShutdownPlugin);

        // Set the shutdown flag externally (simulating signal/delegate)
        let flag = app.world().resource::<ShutdownFlag>().clone();
        flag.0.store(true, Ordering::SeqCst);

        app.update();

        // Verify AppExit was written
        assert!(app.should_exit().is_some());
    }

    #[test]
    fn no_exit_when_flag_is_false() {
        let mut app = App::new();
        app.add_plugins(GracefulShutdownPlugin);

        app.update();

        assert!(app.should_exit().is_none());
    }
}
