use crate::error::PermissionError;

/// Ensure microphone permission is granted (platform-specific).
pub async fn ensure_microphone_permission() -> Result<(), PermissionError> {
    platform::check().await
}

#[cfg(target_os = "macos")]
mod platform {
    use super::PermissionError;

    pub async fn check() -> Result<(), PermissionError> {
        use objc2_av_foundation::{AVAuthorizationStatus, AVCaptureDevice, AVMediaTypeAudio};

        let status = unsafe { AVCaptureDevice::authorizationStatusForMediaType(AVMediaTypeAudio) };

        match status {
            AVAuthorizationStatus::Authorized => Ok(()),
            AVAuthorizationStatus::Denied | AVAuthorizationStatus::Restricted => {
                Err(PermissionError::Denied)
            }
            AVAuthorizationStatus::NotDetermined => {
                let granted = tokio::task::spawn_blocking(|| {
                    let (tx, rx) = std::sync::mpsc::channel();
                    unsafe {
                        AVCaptureDevice::requestAccessForMediaType_completionHandler(
                            AVMediaTypeAudio,
                            &|granted| {
                                let _ = tx.send(granted);
                            },
                        );
                    }
                    rx.recv().unwrap_or(false)
                })
                .await
                .unwrap_or(false);

                if granted {
                    Ok(())
                } else {
                    Err(PermissionError::Denied)
                }
            }
            _ => Err(PermissionError::Unknown("unexpected status".into())),
        }
    }
}

#[cfg(target_os = "windows")]
mod platform {
    use super::PermissionError;

    pub async fn check() -> Result<(), PermissionError> {
        Ok(())
    }
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
mod platform {
    use super::PermissionError;

    pub async fn check() -> Result<(), PermissionError> {
        Ok(())
    }
}
