use crate::prelude::{GlobalWindow, GlobalWindows};
use bevy::math::Rect;
use std::{
    char::{REPLACEMENT_CHARACTER, decode_utf16},
    sync::Mutex,
};
use windows::{
    Win32::{
        Foundation::{HWND, LPARAM},
        UI::WindowsAndMessaging::{
            EnumWindows, GetForegroundWindow, GetWindowRect, GetWindowTextW, IsWindowVisible,
        },
    },
    core::BOOL,
};

static FOUND_WINDOWS: Mutex<Vec<GlobalWindow>> = Mutex::new(Vec::new());

extern "system" fn enum_windows_proc(hwnd: HWND, _: LPARAM) -> BOOL {
    unsafe {
        let Ok(frame) = obtain_window_rect(hwnd) else {
            return BOOL(1);
        };
        if IsWindowVisible(hwnd).into() {
            let Ok(mut found_windows) = FOUND_WINDOWS.lock() else {
                return BOOL(1);
            };
            let current = GetForegroundWindow();
            if 0. < frame.width() && 0. < frame.height() && current != hwnd {
                let mut buf = vec![0u16; 1024];
                let len = GetWindowTextW(hwnd, &mut buf) as usize;
                let title = (0 < len).then(|| decode_title(&buf, len));
                found_windows.push(GlobalWindow {
                    title,
                    frame,
                    hwnd: hwnd.0 as i64,
                });
            }
        }
    }
    BOOL(1)
}

fn decode_title(buf: &[u16], len: usize) -> String {
    decode_utf16(buf[..len].iter().copied())
        .map(|r| r.unwrap_or(REPLACEMENT_CHARACTER))
        .collect()
}

pub fn find_all() -> Option<GlobalWindows> {
    unsafe {
        EnumWindows(Some(enum_windows_proc), LPARAM(0)).ok()?;
    }
    FOUND_WINDOWS
        .lock()
        .map(|mut windows| GlobalWindows::new(std::mem::take(&mut windows)))
        .ok()
}

pub fn update_window(hwnd: i64) -> Option<Rect> {
    /// # Safety
    ///
    /// This function assumes that the `hwnd` is a valid handle to a window.
    unsafe {
        obtain_window_rect(HWND(hwnd as *mut _)).ok()
    }
}

/// # Safety
///
/// You must ensure that the `hwnd` is a valid handle to a window.
unsafe fn obtain_window_rect(hwnd: HWND) -> windows::core::Result<Rect> {
    let mut rect = windows::Win32::Foundation::RECT::default();
    /// # Safety
    ///
    /// This function assumes that `hwnd` is a valid handle to a window.
    unsafe {
        GetWindowRect(hwnd, &mut rect)?;
    }
    Ok(Rect::new(
        rect.left as f32,
        rect.bottom as f32,
        rect.right as f32,
        rect.top as f32,
    ))
}
